use crate::board::*;
use crate::game::*;
use crate::hand::*;
use crate::player::*;
use yew::prelude::*;

const CARD_LAYOUTS: [&str; 5] = [
    "bottom: 35px; left: 60px",
    "top: 35px; left: 60px",
    "top: 15px; left: 250px",
    "top: 35px; left: 440px",
    "bottom: 35px; left: 440px",
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
    if let Some(turn_limit) = props.turn_limit {
        let turn = props.board.get_turn(Some(turn_limit));
        if turn_limit + 1 == props.board.0.len() {
            // only continue to fold if not at the present, and also not when the game is over
            if turn.player_states[props.active_player].folded
                || turn.completed_stage >= 4
            {
                props.onbuttonclick.emit(ButtonType::DoNothing);
            }
        }
    }
    let turn = props.board.get_turn(props.turn_limit);
    let in_future = props.board.is_past(props.turn_limit);
    let is_in_future = if in_future
        || props.board.get_turn(props.turn_limit).completed_stage >= 4
    {
        "disabled-board"
    } else {
        ""
    };
    let left = props.coordinates.0 * 550 + 25;
    let top = props.coordinates.1 * 362 + 25;
    let style = format!("left: {left}px; top: {top}px");
    let active_state = &props.board.get_turn(props.turn_limit);
    let active_hand = if active_state.player_states[props.active_player].folded
    {
        vec![]
    } else {
        active_state.player_states[props.active_player].hand.clone()
    };

    let card_layout_indices =
        CARD_LAYOUT_INDICES[props.players.len() - 1].clone();

    let mut enemy_hands = vec![];
    for i in 1..active_state.player_states.len() {
        let player_number = (props.active_player + i) % props.players.len();
        let player = &props.players[player_number];
        let playerstate = turn.player_states[player_number].clone();
        let playerstatedisplay = {
            PlayerStateDisplay {
                bank: player.chips,
                name: player.name.to_string(),
                betting: props.board.get_turn(props.turn_limit).player_states
                    [player_number]
                    .commitment(),
            }
        };
        let hand = if active_state.player_states[player_number].folded {
            vec![]
        } else {
            active_state.player_states[player_number].hand.clone()
        };
        enemy_hands.push({
            html! {
                <Hand {hand}
                      visible={turn.completed_stage >= 4 && !playerstate.folded}
                      playerstate={playerstatedisplay}
                      style={CARD_LAYOUTS[card_layout_indices[i - 1]]}
                  />
            }
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

    let playerstate = {
        let player = &props.players[props.active_player];
        PlayerStateDisplay {
            bank: player.chips,
            name: player.name.to_string(),
            betting: props.board.get_turn(props.turn_limit).player_states
                [props.active_player]
                .commitment(),
        }
    };

    let potinfo = PlayerStateDisplay {
        name: "Pot".to_string(),
        betting: props
            .board
            .get_turn(props.turn_limit)
            .player_states
            .iter()
            .map(|x| x.commitment())
            .sum(),
        bank: props.board.get_turn(props.turn_limit).bet_amount,
    };

    let buttons = if in_future
        || props.turn_limit.is_none()
        || turn.completed_stage >= 4
    {
        html! {}
    } else {
        html! {
            <div class="actions">
                <button class="do-button" onclick={props.onbuttonclick.reform(|_| ButtonType::CallOrCheck)}>
                    {if turn.bet_amount == 0 { "Check" } else { "Call" }}
                </button>
                <button class="do-button" onclick={props.onbuttonclick.reform(|_| ButtonType::RaiseOrBet)}>
                    {if turn.bet_amount == 0 { "Bet" } else { "Raise" }}
                </button>
                <button class="do-button" onclick={props.onbuttonclick.reform(|_| ButtonType::Fold)}>{"Fold"}</button>
            </div>
        }
    };

    let winning_type_display = if let Some(winning_type) = turn.winning_hand_type {
        html!{
            <div class="winning-hand-display">{format!("{}!", winning_type)}</div>
        }
    } else {
        html!{}
    };

    let ondragover = {
        let infinity = props.turn_limit.is_none();
        let onbuttonclick = props.onbuttonclick.clone();
        move |e: DragEvent| {
            e.prevent_default();
            if infinity {
                onbuttonclick.emit(ButtonType::ToggleView)
            }
        }
    };

    html! {
        <div class={format!("table absolute {}", is_in_future)}
            {style} {ondragover} ondrop={props.ondrop.clone()}>
            <Hand hand={active_hand}
                visible={true}
                style="bottom: 25px; left: 250px"
                draggable={!in_future}
                {playerstate}
                ondragstart={ondragstart_player} />
            {for enemy_hands}
            <Hand hand={vec![active_state.deck[0]]} visible={false} style="top: 150px; left: 150px; transform: none" />
            <Hand hand={active_state.open_cards.clone()} playerstate={potinfo}
                visible={true}
                style="top: 150px; left: 200px; transform: none; text-align: left"
                draggable={!in_future}
                ondragstart={ondragstart_global} />
            {buttons}
            {clock(&props.turn_limit.map(|x| (x+1).to_string()).unwrap_or("∞".to_string()), props.onbuttonclick.reform(|_| ButtonType::ToggleView))}
            {winning_type_display}
        </div>
    }
}

fn clock(label: &str, onclick: Callback<MouseEvent>) -> Html {
    let ticks = (0..12_i8).map(|x| {
        html! {
            <div class="tick-mark" style={format!("transform: rotate({}turn)", Into::<f64>::into(x) / 12_f64)}></div>
        }
    });
    html! {
        <div class="clock" {onclick}>
            {for ticks}
            <div class="hand"></div>
            <div class="label">{label}</div>
        </div>
    }
}
