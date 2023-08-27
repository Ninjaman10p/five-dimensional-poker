use crate::cards::*;
use crate::player::PlayerState;

#[derive(Clone, PartialEq, Debug)]
pub struct Turn {
    pub player_states: Vec<PlayerState>,
    pub open_cards: Vec<Card>,
    pub deck: Vec<Card>,
    pub completed_stage: usize,
    pub bet_amount: i64,
    pub num_checks: usize,
    pub winning_hand_type: Option<HandType>,
}

impl Turn {
    pub fn first_round(mut deck: Vec<Card>, num_players: usize) -> Self {
        let mut player_states = vec![];
        for _ in 1..=num_players {
            player_states.push(PlayerState {
                bet: vec![],
                folded: false,
                hand: vec![
                    deck.pop().unwrap(),
                    deck.pop().unwrap(),
                    // deck.pop().unwrap(), // remove these for hold-em
                    // deck.pop().unwrap(),
                ],
            });
        }
        let open = vec![];
        Self {
            deck,
            open_cards: open,
            player_states,
            completed_stage: 0,
            bet_amount: 0,
            num_checks: 0,
            winning_hand_type: None,
        }
    }
}
