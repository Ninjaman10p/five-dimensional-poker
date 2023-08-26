pub mod turn;

pub use turn::Turn;
use crate::cards::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Board(pub Vec<Turn>, pub bool); // second element is whether to show the present

impl Board {
    pub fn new(deck: Vec<Card>, num_players: usize) -> Self {
        Board(vec![Turn::first_round(deck, num_players)], false)
    }

    pub fn timeline_intersect(base: &Board, turn: usize) -> Board {
        let mut new_board = base.clone();
        new_board.0.truncate(turn);
        new_board
    }

    pub fn is_past(&self, turn_limit: Option<usize>) -> bool {
        if let Some(turn_limit) = turn_limit {
            return self.0.len() > turn_limit + 1;
        } else {
            return false;
        }
    }

    pub fn get_turn(&self, turn_limit: Option<usize>) -> &Turn {
        &self.0[turn_limit.unwrap_or(self.0.len() - 1)]
    }

    pub fn get_turn_mut(&mut self, turn_limit: Option<usize>) -> &mut Turn {
        let index = turn_limit.unwrap_or(self.0.len() - 1);
        &mut self.0[index]
    }
}
