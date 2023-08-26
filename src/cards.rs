use itertools::*;
use yew::prelude::*;

#[derive(PartialEq, Copy, Clone, Eq, Debug)]
pub enum Suite {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

impl Suite {
    fn repr(&self) -> &'static str {
        use Suite::*;
        match self {
            Clubs => "♣",
            Hearts => "♥",
            Spades => "♠",
            Diamonds => "♦",
        }
    }
}

/// Require 1 <= internal_value <= 13
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Rank(u8);

impl Rank {
    fn repr(&self) -> &'static str {
        match self.0 {
            1 => "A",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "T",
            11 => "J",
            12 => "Q",
            13 => "K",
            _ => "?",
        }
    }
}

pub fn calculate_winner<T>(hands: &Vec<(T, Vec<Card>)>) -> T
where
    T: Sized + Copy + Clone,
{
    hands
        .iter()
        .max_by(|a, b| type_of_hand(&a.1).cmp(&type_of_hand(&b.1)))
        .unwrap()
        .0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    suite: Suite,
    rank: Rank,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum HandType {
    NoPair,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
}

pub fn type_of_hand(hand: &Vec<Card>) -> HandType {
    use HandType::*;
    let mut best_hand = NoPair;
    for hand in hand.iter().combinations(5) {
        best_hand =
            type_of_hand_5(&hand.into_iter().cloned().collect()).max(best_hand)
    }
    best_hand
}

pub fn type_of_hand_5(hand: &Vec<Card>) -> HandType {
    use HandType::*;
    let ranks = split_by_rank(hand);
    let suites = split_by_suite(hand);
    let sequences = split_by_sequence(hand);
    if ranks[0] >= 5 {
        return FiveOfAKind;
    }
    if sequences[0] >= 5 && suites[0] >= 5 {
        return StraightFlush;
    }
    if ranks[0] >= 4 {
        return FourOfAKind;
    }
    if ranks[0] >= 3 && ranks[1] >= 2 {
        return FullHouse;
    }
    if suites[0] >= 5 {
        return Flush;
    }
    if sequences[0] >= 5 {
        return Straight;
    }
    if ranks[0] >= 3 {
        return ThreeOfAKind;
    }
    if ranks[0] >= 2 && ranks[1] >= 2 {
        return TwoPairs;
    }
    if ranks[0] >= 2 {
        return OnePair;
    }

    NoPair
}

pub fn split_by_sequence(hand: &Vec<Card>) -> Vec<usize> {
    let mut hand: Vec<u8> =
        hand.clone().into_iter().map(|x| x.rank.0).collect();
    hand.sort();

    let mut out = vec![];
    let mut last = 13;
    for i in hand.iter() {
        if i == &(last + 1) {
            *(out.last_mut().unwrap()) += 1;
        } else {
            out.push(1);
        }
        last = *i;
    }
    out.sort();
    out
}

pub fn split_by_suite(hand: &Vec<Card>) -> Vec<usize> {
    use Suite::*;
    let mut out: Vec<_> = [Hearts, Clubs, Spades, Diamonds]
        .iter()
        .map(|suite| hand.iter().filter(|card| &card.suite == suite).count())
        .collect();
    out.sort();
    out.reverse();
    out
}

pub fn split_by_rank(hand: &Vec<Card>) -> Vec<usize> {
    let mut out: Vec<_> = (1..13)
        .map(|rank| hand.iter().filter(|card| card.rank == Rank(rank)).count())
        .collect();
    out.sort();
    out.reverse();
    out
}

pub fn fresh_deck() -> Vec<Card> {
    use Suite::*;
    let mut deck = vec![];
    for suite in vec![Clubs, Hearts, Spades, Diamonds].into_iter() {
        for rank in 1..=13 {
            deck.push(Card {
                suite,
                rank: Rank(rank),
            });
        }
    }
    deck
}

#[derive(PartialEq, Properties)]
pub struct CardDisplayProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub style: String,
    pub card: Card,
    #[prop_or(true)]
    pub visible: bool,
    #[prop_or(false)]
    pub draggable: bool,
    #[prop_or_default]
    pub ondragstart: Callback<DragEvent>,
}

#[function_component]
pub fn CardDisplay(props: &CardDisplayProps) -> Html {
    let card = props.card;
    let style = props.style.to_string();
    let onclick = props.onclick.clone();
    use Suite::*;
    let color = match card.suite {
        Hearts | Diamonds => "red-card",
        Clubs | Spades => "black-card",
    };
    let (face, visibility) = if !props.visible {
        ("".to_string(), "card-back")
    } else {
        (format!("{}{}", card.rank.repr(), card.suite.repr()), "")
    };
    html! {
        <div class={format!("card {} {}", color, visibility)}
            {style}
            {onclick}
            draggable={format!("{}", props.draggable)}
            ondragstart={props.ondragstart.clone()}>
            {face}
        </div>
    }
}
