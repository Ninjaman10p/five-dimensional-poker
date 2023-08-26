use wasm_bindgen::JsCast;
use crate::multiverse::*;
use web_sys::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NewGameProps {
    #[prop_or_default]
    pub ongamecreate: Callback<Multiverse>,
}

static GAMER_TAGS: [&str; 6] = [
    "Xx_bootyslayer_xX",
    "PokerKing",
    "LordOfTime",
    "ChipHoarder69",
    "Player5",
    "WeedDealer420",
];

#[function_component]
pub fn NewGame(props: &NewGameProps) -> Html {
    let inputs = (0..=5).map(|i| {
        html! {
            <div class="player-name">
                <label for={format!("PlayerName{}", i)}> {format!("Player {}", i + 1)} </label>
                <input id={format!("PlayerName{}", i)} placeholder={GAMER_TAGS.get(i).unwrap().to_string()} />
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
                ongamecreate.emit(Multiverse::from_players(players));
            }
        }
    };
    html! {
        <div class="table centered">
            {for inputs}
            <button class="start-game do-button" {onclick}>{"Start"}</button>
        </div>
    }
}
