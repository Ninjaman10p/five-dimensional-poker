mod cards;

use cards::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
use wasm_bindgen::JsCast;
use web_sys::*;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let active_game: UseStateHandle<Option<Game>> = use_state(|| None);
    if let Some(game) = (*active_game).clone() {
        html! {}
    } else {
        let ongamecreate = {
            let active_game = active_game.clone();
            move |game| {
                active_game.set(Some(game));
            }
        };
        html! {
            <NewGame {ongamecreate}/>
        }
    }
}

#[derive(PartialEq, Properties)]
struct NewGameProps {
    #[prop_or_default]
    ongamecreate: Callback<Game>,
}

#[function_component]
fn NewGame(props: &NewGameProps) -> Html {
    let gamer_tags = vec![
        "Xx_bootyslayer_xX",
        "PokerKing",
        "LordOfTime",
        "ChipHoarder69",
        "Player5",
        "WeedDealer420",
    ];

    let inputs = (0..=5).map(|i| {
        html! {
            <div class="player-name">
                <label for={format!("PlayerName{}", i)}> {format!("Player {}", i + 1)} </label>
                <input id={format!("PlayerName{}", i)} placeholder={gamer_tags.get(i).unwrap().to_string()} />
            </div>
        }
    });
    let onclick = {
        let ongamecreate = props.ongamecreate.clone();
        move |_e: MouseEvent| {
            let document = window()
                .expect("no window")
                .document()
                .expect("no document");
            let players: Vec<String> = (0..=5)
                .map(|i| {
                    document
                        .get_element_by_id(&format!("PlayerName{}", i))
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>()
                        .value()
                        .to_string()
                })
                .filter(|a| a.len() != 0)
                .collect();
            if players.len() >= 2 {
                ongamecreate.emit(Game::from_players(players));
            }
        }
    };
    html! {
        <div class="table centered">
            {for inputs}
            <button class="start-game" {onclick}>{"Start"}</button>
        </div>
    }
}

#[derive(Clone)]
struct Game {
    players: Vec<Player>,
    boards: Vec<Timeline>,
}
#[derive(Clone)]
pub struct PlayerState {
    hand: Vec<Card>,
    bet: usize, // 0 means need to bet
    folded: bool,
}

#[derive(Clone)]
pub struct Player {
    name: String,
    chips: usize,
}

impl Player {
    fn from_name(name: String) -> Self {
        Self { name, chips: 5 }
    }
}

impl Game {
    fn from_players(players: Vec<String>) -> Self {
        let num_players = players.len();
        Self {
            players: players
                .into_iter()
                .map(|name| Player::from_name(name))
                .collect(),
            boards: vec![Timeline::genesis(num_players)],
        }
    }

    /// returns the indices of the new timeline in same order as arguments
    fn spawn_timeline(parent_index: usize, starting_time: usize) -> (usize, usize) {
        // TODO
    }
}

// TIMELINES

// represents a single branch of the tree...
#[derive(Clone)]
pub struct Timeline {
    parent_index: usize,
    starting_time: usize,
    boards: Vec<Board>,
}

#[derive(Clone)]
pub struct Board(Vec<BoardState>);

#[derive(Clone)]
pub struct BoardState {
    player_states: Vec<PlayerState>,
    deck: Vec<Card>,
}

impl Timeline {
    fn genesis(num_players: usize) -> Self {
        let mut deck = fresh_deck();
        deck.shuffle(&mut thread_rng());
        Self {
            parent_index: 0,
            starting_time: 0,
            boards: vec![Board::new(deck, num_players)],
        }
    }
}

impl Board {
    fn new(deck: Vec<Card>, num_players: usize) -> Self {
        Board(vec![BoardState::first_round(deck, num_players)])
    }
}

impl BoardState {
    fn first_round(mut deck: Vec<Card>, num_players: usize) -> Self {
        let mut player_states = vec![];
        for i in 1..=num_players {
            player_states.push(PlayerState {
                bet: 0,
                folded: false,
                hand: vec![deck.pop().unwrap(), deck.pop().unwrap(), deck.pop().unwrap(), deck.pop().unwrap()],
            });
        }
        Self {
            deck,
            player_states,
        }
    }
}
