use crate::cards::*;
use yew::prelude::*;

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub struct PlayerStateDisplay {
    pub name: String,
    pub bank: i64,
    pub betting: i64,
}

#[derive(PartialEq, Properties)]
pub struct HandProps {
    pub hand: Vec<Card>,
    #[prop_or(true)]
    pub visible: bool,
    #[prop_or_default]
    pub style: String,
    #[prop_or(false)]
    pub draggable: bool,
    #[prop_or_default]
    pub ondragstart: Callback<(DragEvent, usize)>,
    #[prop_or_default]
    pub playerstate: Option<PlayerStateDisplay>,
}

#[function_component]
pub fn Hand(props: &HandProps) -> Html {
    let cards = props.hand.iter().enumerate().map(|(i, card)| {
        html! {
            <CardDisplay card={card.clone()}
                visible={props.visible}
                draggable={props.draggable}
                ondragstart={props.ondragstart.reform(move |e| (e, i))}/>
        }
    });
    let player_state = if let Some(ps) = props.playerstate.clone() {
        html! {

            <span class="name-info">{format!("{} · {}﻿/﻿{}﻿⏲ ", ps.name, ps.betting, ps.bank)}</span>
        }
    } else {
        html! {}
    };
    html! {
        <div class="hand-of-cards" style={props.style.to_string()}>
            {for cards}<br/>
            {player_state}
        </div>
    }
}
