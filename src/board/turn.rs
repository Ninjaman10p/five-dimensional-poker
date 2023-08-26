use crate::cards::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Turn {
    pub player_states: Vec<PlayerState>,
    pub open_cards: Vec<Card>,
    pub deck: Vec<Card>,
    pub completed_stage: usize,
    pub ante: Vec<i64>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PlayerState {
    pub hand: Vec<Card>,
    pub bet: Vec<i64>,
    pub folded: bool,
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
            ante: vec![],
        }
    }

    pub fn set_ante(&mut self, amount: i64) {
        if self.ante.len() <= self.completed_stage {
            self.ante.push(amount)
        } else {
            self.ante[self.completed_stage] = amount;
        }
    }
}
