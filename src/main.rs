pub mod board;
pub mod board_display;
mod cards;
mod game;
pub mod game_display;
mod hand;
pub mod multiverse;
mod new_game;
pub mod player;

use multiverse::*;
use game::*;
use game_display::*;
use new_game::*;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let active_game: UseStateHandle<Option<Multiverse>> = use_state(|| None);
    if let Some(game) = (*active_game).clone() {
        let ongameupdate = {
            let active_game = active_game.clone();
            move |game| {
                active_game.set(Some(game));
            }
        };
        html! {
            <GameDisplay {game} {ongameupdate}/>
        }
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
