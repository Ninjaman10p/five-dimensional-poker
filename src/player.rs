use crate::cards::Card;

#[derive(Clone, PartialEq, Debug)]
pub struct Player {
    pub name: String,
    pub chips: i64,
}

impl Player {
    pub fn from_name(name: String) -> Self {
        Self { name, chips: 30 }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct PlayerState {
    pub hand: Vec<Card>,
    pub bet: Vec<i64>,
    pub folded: bool,
}

impl PlayerState {
    pub fn commitment(&self) -> i64 {
        self.bet.iter().sum::<i64>() + Self::ANTE
    }

    const ANTE: i64 = 1;
}
