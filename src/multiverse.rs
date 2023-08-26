use crate::board::*;
use crate::cards::calculate_winner;
use crate::game::*;
use crate::player::*;
use rand::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Multiverse {
    pub players: Vec<Player>,
    pub timelines: Vec<Timeline>,
    pub active_player: usize,
}

impl Multiverse {
    pub fn from_players(players: Vec<String>) -> Self {
        let num_players = players.len();
        Self {
            players: players
                .into_iter()
                .map(|name| Player::from_name(name))
                .collect(),
            timelines: vec![
                Timeline::genesis(num_players),
                Timeline::genesis(num_players),
            ],
            active_player: usize::MAX, // cannot be a player
        }
    }

    /// returns the indices of the new timeline in same order as arguments
    pub fn spawn_timeline(
        &mut self,
        parent_index: usize,
        starting_time: usize,
    ) -> (usize, usize) {
        let target_timeline = &self.timelines[parent_index];
        let target_board = &target_timeline.boards
            [starting_time - target_timeline.starting_time];
        self.timelines.push(Timeline {
            parent_index,
            starting_time: starting_time,
            boards: vec![Board::timeline_intersect(
                target_board,
                self.get_turn(),
            )],
        });
        (self.timelines.len() - 1, starting_time)
    }

    pub fn get_active_player(&self) -> usize {
        return self.get_turn() % self.players.len();
    }

    pub fn get_turn(&self) -> usize {
        let mut turn = self.timelines[0].boards[0].0.len();
        for timeline in self.timelines.iter() {
            for board in timeline.boards.iter() {
                turn = board.0.len().min(turn);
            }
        }
        turn - 1
    }

    pub fn try_raise(&mut self, timeline: usize, min_increase: i64) -> bool {
        let min_amount = min_increase + self.get_ante(timeline); // TODO
        if let Some(amount) = gloo_dialogs::prompt(
            &format!("Enter new bet (min {})", min_amount),
            None,
        )
        .and_then(|a| a.parse::<i64>().ok())
        {
            if amount >= min_increase {
                return self.try_bet(timeline, amount);
            }
        }
        false
    }

    const ANTE: i64 = 0;
    pub fn get_ante(&self, timeline: usize) -> i64 {
        let state =
            &self.timelines[timeline].current_board().0[self.get_turn()];
        return state
            .ante
            .get(state.completed_stage)
            .unwrap_or(&Self::ANTE)
            .clone();
    }

    pub fn can_bet(
        &self,
        timeline: usize,
        _player: usize,
        amount: i64,
    ) -> bool {
        return amount >= self.get_ante(timeline); // TODO
    }

    pub fn try_bet(&mut self, timeline: usize, amount: i64) -> bool {
        if self.can_bet(timeline, self.get_active_player(), amount) {
            log::debug!(
                "{:?}",
                self.timelines[timeline].boards.last().unwrap()
            );
            let turn = self.get_turn();
            let mut state =
                self.timelines[timeline].boards.last().unwrap().0[turn].clone();
            state.player_states[self.get_active_player()]
                .bet
                .push(amount);
            state.set_ante(amount);
            self.timelines[timeline].current_board_mut().0.push(state);
            self.try_increase_stage(timeline);
            return true;
        }
        return false;
    }

    pub fn try_call(&mut self, timeline: usize) -> bool {
        if self.can_bet(
            timeline,
            self.get_active_player(),
            self.get_ante(timeline),
        ) {
            return self.try_bet(timeline, self.get_ante(timeline));
        }
        return false;
    }

    pub fn fold(&mut self, timeline: usize) {
        let active_player = self.get_active_player();
        let state = &mut self.timelines[timeline]
            .current_board_mut()
            .0
            .last_mut()
            .unwrap();
        state.player_states[active_player].folded = true;
        self.try_increase_stage(timeline);
    }

    pub fn try_increase_stage(&mut self, timeline: usize) {
        let board_num = self.timelines[timeline].boards.len() - 1;
        let board_turn = self.timelines[timeline].boards[board_num].0.len() - 1;
        let mut state =
            &mut self.timelines[timeline].boards[board_num].0[board_turn];

        let next_stage = state
            .player_states
            .iter()
            .filter(|p| !p.folded)
            .map(|p| p.bet.len())
            .min()
            .unwrap();
        if next_stage > state.completed_stage {
            state.completed_stage = next_stage;
            match state.completed_stage {
                1 => {
                    // draw the initial 3 cards
                    state.open_cards.push(state.deck.pop().unwrap());
                    state.open_cards.push(state.deck.pop().unwrap());
                    state.open_cards.push(state.deck.pop().unwrap());
                }
                2 => {
                    // draw the extra card
                    state.open_cards.push(state.deck.pop().unwrap());
                }
                3 => {
                    // draw the extra card
                    state.open_cards.push(state.deck.pop().unwrap());
                }
                4 => {
                    // reveal a fifth here for hold'em
                    // showdown
                    self.showdown(timeline)
                }
                _ => { /* unreachable */ }
            }
        }
    }

    pub fn showdown(&mut self, timeline: usize) {
        let turn = self.get_turn();
        let state =
            &mut self.timelines[timeline].boards.last_mut().unwrap().0[turn];
        let hands = (0..self.players.len())
            .filter(|i| !state.player_states[i.clone()].folded)
            .map(|i| (i, state.player_states[i].hand.clone()))
            .collect();
        let winner = calculate_winner(&hands);
        let mut deck = state.deck.clone();
        for i in state.player_states.iter() {
            deck.append(&mut i.hand.clone());
        }
        deck.append(&mut state.open_cards.clone());
        let mut winnings: Vec<i64> =
            (0..state.player_states.len()).map(|_| 0).collect();
        for (i, player) in state.player_states.iter().enumerate() {
            winnings[i] -= player.bet.iter().sum::<i64>();
            winnings[winner] += player.bet.iter().sum::<i64>();
        }
        drop(state);
        for (i, delta) in winnings.iter().enumerate() {
            self.players[i].chips += delta;
        }
        deck.shuffle(&mut thread_rng());
        self.timelines[timeline]
            .boards
            .push(Board::new(deck, self.players.len()));
    }
}
