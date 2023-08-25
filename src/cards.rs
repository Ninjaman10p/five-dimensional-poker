#[derive(PartialEq, Copy, Clone)]
pub enum Suite {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

/// Require 1 <= internal_value <= 13
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Rank(u8);

#[derive(Clone, Copy)]
struct Card {
    suite: Suite,
    rank: Rank,
}

fn in_sequence(hand: Vec<Card>) -> bool {
    let mut hand = hand.clone();
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    let mut value = None;
    for i in hand.iter() {
        if let Some(rank) = value {
            if i.rank.0 != rank + 1 {
                return false;
            }
        }
        value = Some(i.rank.0);
    }
    true
}

pub enum HandType {
    FiveOfAKind,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    NoPair,
}
