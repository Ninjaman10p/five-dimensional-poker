use crate::board::*;
use crate::cards::calculate_winners;
use crate::game::*;
use crate::player::*;
use rand::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Multiverse {
    pub players: Vec<Player>,
    pub timelines: Vec<Timeline>,
    pub active_player: usize, // cache, so that the board can be hidden.
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
        let mut boards = vec![];
        /* for i in 0..starting_time {
            boards.push(Board::new(vec![], 0));
        } */
        boards.push(Board::timeline_intersect(target_board, self.get_turn()));
        self.timelines.push(Timeline {
            parent_index,
            starting_time: starting_time,
            boards: boards,
        });
        // will always be the first index
        (self.timelines.len() - 1, 0)
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

    pub fn current_turn(&self, timeline: usize) -> &Turn {
        self.timelines[timeline]
            .boards
            .last()
            .unwrap()
            .get_turn(Some(self.get_turn()))
    }

    pub fn try_initial_bet(&mut self, timeline: usize) -> bool {
        if let Some(amount) =
            gloo_dialogs::prompt("Enter initial bet (min 1⏲)", None)
                .and_then(|a| a.parse().ok())
        {
            if amount >= 1 {
                return self.try_bet(timeline, amount);
            }
        }
        return false;
    }

    pub fn try_raise_or_bet(&mut self, timeline: usize) -> bool {
        if self.current_turn(timeline).bet_amount == 0 {
            self.try_initial_bet(timeline)
        } else {
            self.try_raise(timeline)
        }
    }

    pub fn try_raise(&mut self, timeline: usize) -> bool {
        let min_amount = 2 * self.current_turn(timeline).bet_amount; // TODO
        if let Some(amount) = gloo_dialogs::prompt(
            &format!(
                "Enter new bet (min {}⏲ to double previous bet)",
                min_amount
            ),
            None,
        )
        .and_then(|a| a.parse::<i64>().ok())
        {
            if amount >= min_amount {
                return self.try_bet(timeline, amount);
            }
        }
        false
    }

    pub fn can_bet(
        &self,
        timeline: usize,
        _player: usize,
        amount: i64,
    ) -> bool {
        return amount
            >= self.timelines[timeline]
                .boards
                .last()
                .unwrap()
                .get_turn(Some(self.get_turn()))
                .bet_amount;
    }

    pub fn try_check(&mut self, timeline: usize) -> bool {
        if self.current_turn(timeline).bet_amount == 0 {
            let mut turn = self.current_turn(timeline).clone();
            turn.num_checks += 1;
            self.timelines[timeline].current_board_mut().0.push(turn);
            self.try_increase_stage(timeline);
            return true;
        }
        return false;
    }

    pub fn try_bet(&mut self, timeline: usize, amount: i64) -> bool {
        if self.can_bet(timeline, self.get_active_player(), amount) {
            let mut turn = self.current_turn(timeline).clone();
            turn.player_states[self.get_active_player()]
                .bet
                .push(amount);
            turn.bet_amount = amount;
            self.timelines[timeline].current_board_mut().0.push(turn);
            self.try_increase_stage(timeline);
            return true;
        }
        return false;
    }

    pub fn try_call(&mut self, timeline: usize) -> bool {
        let bet_amount = self.timelines[timeline]
            .boards
            .last()
            .unwrap()
            .get_turn(Some(self.get_turn()))
            .bet_amount;
        if self.can_bet(timeline, self.get_active_player(), bet_amount) {
            return self.try_bet(timeline, bet_amount);
        }
        return false;
    }

    pub fn fold(&mut self, timeline: usize) {
        let active_player = self.get_active_player();
        let mut state = self.timelines[timeline]
            .current_board()
            .0
            .last()
            .unwrap()
            .clone();
        state.player_states[active_player].folded = true;
        self.timelines[timeline].current_board_mut().0.push(state);
        self.try_increase_stage(timeline);
    }

    pub fn skip(&mut self, timeline: usize, epoch: usize) {
        let board = &mut self.timelines[timeline].boards[epoch];
        board.0.push(board.0.last().unwrap().clone());
    }

    pub fn try_increase_stage(&mut self, timeline: usize) {
        let board_num = self.timelines[timeline].boards.len() - 1;
        let board_turn = self.timelines[timeline].boards[board_num].0.len() - 1;
        let mut state =
            &mut self.timelines[timeline].boards[board_num].0[board_turn];

        let players_not_folded = state
            .player_states
            .iter()
            .filter(|p| !p.folded)
            .collect::<Vec<_>>()
            .len();
        if state.num_checks == players_not_folded {
            for player in &mut state.player_states {
                if !player.folded {
                    player.bet.push(0);
                }
            }
        }

        let mut next_stage = state
            .player_states
            .iter()
            .filter(|p| !p.folded)
            .map(|p| p.bet.len())
            .min()
            .unwrap();
        if players_not_folded <= 1 {
            next_stage = 4;
        }
        if next_stage > state.completed_stage {
            state.num_checks = 0;
            state.completed_stage = next_stage;
            match state.completed_stage {
                1 => {
                    // draw the initial 3 cards
                    state.bet_amount = 0;
                    state.open_cards.push(state.deck.pop().unwrap());
                    state.open_cards.push(state.deck.pop().unwrap());
                    state.open_cards.push(state.deck.pop().unwrap());
                }
                2 | 3 => {
                    // draw the extra card
                    state.bet_amount = 0;
                    state.open_cards.push(state.deck.pop().unwrap());
                }
                4 => {
                    // reveal a fifth here for hold'em
                    // showdown
                    let winning_hand_type = self.showdown(timeline);
                    let mut state = &mut self.timelines[timeline].boards
                        [board_num]
                        .0[board_turn];
                    state.winning_hand_type = Some(winning_hand_type);
                }
                _ => { /* unreachable */ }
            }
        }
    }

    pub fn showdown(&mut self, timeline: usize) -> crate::cards::HandType {
        let state = &mut self.timelines[timeline]
            .boards
            .last_mut()
            .unwrap()
            .0
            .last()
            .unwrap();
        let hands = (0..self.players.len())
            .filter(|i| !state.player_states[i.clone()].folded)
            .map(|i| (i, state.player_states[i].hand.clone()))
            .collect();
        let (winners, winning_hand) = calculate_winners(&hands);
        let mut deck = state.deck.clone();
        for i in state.player_states.iter() {
            deck.append(&mut i.hand.clone());
        }
        deck.append(&mut state.open_cards.clone());
        let mut winnings: Vec<f64> =
            (0..state.player_states.len()).map(|_| 0_f64).collect();
        for (i, player) in state.player_states.iter().enumerate() {
            winnings[i] -= player.commitment() as f64;
            for winner in &winners {
                winnings[winner.clone()] +=
                    player.commitment() as f64 / winners.len() as f64;
            }
        }
        drop(state);
        for (i, delta) in winnings.into_iter().enumerate() {
            self.players[i].chips += delta as i64;
        }
        deck.shuffle(&mut thread_rng());
        self.timelines[timeline]
            .boards
            .push(Board::new(deck, self.players.len()));
        winning_hand
    }
}
