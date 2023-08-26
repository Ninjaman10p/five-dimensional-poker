use crate::cards::*;
use rand::prelude::SliceRandom;
use crate::board::*;
use rand::thread_rng;

pub enum ButtonType {
    Call,
    Raise,
    Fold,
}

// TIMELINES

// represents a single branch of the tree...
#[derive(Clone, PartialEq, Debug)]
pub struct Timeline {
    pub parent_index: usize,
    pub starting_time: usize,
    pub boards: Vec<Board>,
}

impl Timeline {
    pub fn genesis(num_players: usize) -> Self {
        let mut deck = fresh_deck();
        deck.shuffle(&mut thread_rng());
        Self {
            parent_index: 0,
            starting_time: 0,
            boards: vec![Board::new(deck, num_players)],
        }
    }

    pub fn current_board(&self) -> &Board {
        return self.boards.last().unwrap();
    }

    pub fn current_board_mut(&mut self) -> &mut Board {
        return self.boards.last_mut().unwrap();
    }
}
