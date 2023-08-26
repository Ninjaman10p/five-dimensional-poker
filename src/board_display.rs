use crate::game::*;
use yew::prelude::*;
use crate::hand::*;
use crate::player::*;
use crate::board::*;

const CARD_LAYOUTS: [&str; 5] = [
    "bottom: 35px; left: 30px",
    "top: 35px; left: 30px",
    "top: 15px; left: 190px",
    "top: 35px; right: 30px",
    "bottom: 35px;; right: 30px",
];

const CARD_LAYOUT_INDICES: [[usize; 5]; 6] = [
    [9, 9, 9, 9, 9], // 9 = never
    [2, 9, 9, 9, 9],
    [1, 3, 9, 9, 9],
    [1, 2, 3, 9, 9],
    [0, 1, 3, 4, 9],
    [0, 1, 2, 3, 4],
];

#[derive(PartialEq, Properties)]
pub struct BoardDisplayProps {
    pub board: Board,
    pub turn_limit: Option<usize>,
    pub coordinates: (usize, usize),
    pub active_player: usize,
    pub players: Vec<Player>,
    #[prop_or_default]
    pub ondragstart: Callback<(DragEvent, usize, usize)>, // player, card
    #[prop_or_default]
    pub ondrop: Callback<DragEvent>,
    #[prop_or_default]
    pub onbuttonclick: Callback<ButtonType>,
}

#[function_component]
pub fn BoardDisplay(props: &BoardDisplayProps) -> Html {
    let in_future = props.board.is_past(props.turn_limit);
    let is_in_future = if in_future { "disabled-board" } else { "" };
    let left = props.coordinates.0 * 550 + 25;
    let top = props.coordinates.1 * 362 + 25;
    let style = format!("left: {left}px; top: {top}px");
    let active_state = &props.board.get_turn(props.turn_limit);
    let active_hand =
        active_state.player_states[props.active_player].hand.clone();
    log::info!("{:?}", active_hand);
    let mut enemy_hands = vec![];
    let card_layout_indices =
        CARD_LAYOUT_INDICES[props.players.len() - 1].clone();
    for i in 1..active_state.player_states.len() {
        let player_number = (props.active_player + i) % props.players.len();
        let playerstate = PlayerState {

        };
        enemy_hands.push(html! {
            <Hand hand={active_state.player_states[player_number].hand.clone()}
                  visible={false}
                  {playerstate}
                  style={CARD_LAYOUTS[card_layout_indices[i - 1]]}
              />
        });
    }
    let ondragstart_player = if in_future {
        Callback::noop()
    } else {
        let active_player = props.active_player;
        props
            .ondragstart
            .reform(move |(e, i)| (e, active_player, i))
    };
    let ondragstart_global = if in_future {
        Callback::noop()
    } else {
        props.ondragstart.reform(|(e, i)| (e, usize::MAX, i))
    };

    let buttons = if in_future {
        html! {}
    } else {
        html! {
            <div class="actions">
                <button class="do-button" onclick={props.onbuttonclick.reform(|_| ButtonType::Call)}>{"Call"}</button>
                <button class="do-button" onclick={props.onbuttonclick.reform(|_| ButtonType::Raise)}>{"Raise"}</button>
                <button class="do-button" onclick={props.onbuttonclick.reform(|_| ButtonType::Fold)}>{"Fold"}</button>
            </div>
        }
    };

    html! {
        <div class={format!("table absolute {}", is_in_future)}
            {style} ondragover={|e: DragEvent| e.prevent_default()} ondrop={props.ondrop.clone()}>
            <Hand hand={active_hand}
                visible={true}
                style="bottom: 25px; left: 190px"
                draggable={!in_future}
                ondragstart={ondragstart_player} />
            {for enemy_hands}
            <Hand hand={vec![active_state.deck[0]]} visible={false} style="top: 150px; left: 150px" />
            <Hand hand={active_state.open_cards.clone()}
                visible={true}
                style="top: 150px; left: 200px"
                draggable={!in_future}
                ondragstart={ondragstart_global} />
            {buttons}
            <div class="clock">{"∞"}</div>
        </div>
    }
}
