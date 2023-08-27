use crate::board_display::*;
use crate::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GameDisplayProps {
    pub game: Multiverse,
    #[prop_or_default]
    pub ongameupdate: Callback<Multiverse>,
}

// actual game logic goes here
#[function_component]
pub fn GameDisplay(props: &GameDisplayProps) -> Html {
    let mut boards = vec![];
    if props.game.active_player == props.game.get_active_player() {
        for (timeline_num, timeline) in props.game.timelines.iter().enumerate()
        {
            for (t, board) in timeline.boards.iter().enumerate() {
                let ondragstart = {
                    move |(e, player, card): (DragEvent, usize, usize)| {
                        let dt = e.data_transfer().unwrap();
                        dt.set_data("timeline", &timeline_num.to_string())
                            .unwrap();
                        dt.set_data("t", &t.to_string()).unwrap();
                        dt.set_data("player", &player.to_string()).unwrap();
                        dt.set_data("card", &card.to_string()).unwrap();
                    }
                };
                let ondrop = {
                    let ongameupdate = props.ongameupdate.clone();
                    let game = props.game.clone();
                    move |e: DragEvent| {
                        let mut game = game.clone();
                        let dt = e.data_transfer().unwrap();
                        let timeline_from: usize =
                            dt.get_data("timeline").unwrap().parse().unwrap();
                        let t_from: usize =
                            dt.get_data("t").unwrap().parse().unwrap();
                        let player: usize =
                            dt.get_data("player").unwrap().parse().unwrap();
                        let i: usize =
                            dt.get_data("card").unwrap().parse().unwrap();

                        let num_burn = (t_from as i64 - t as i64).abs()
                            + (timeline_from as i64 - timeline_num as i64)
                                .abs()
                            + if player < game.players.len() { 0 } else { 4 };
                        let turn_limit = game.get_turn();
                        if (timeline_from != timeline_num || t_from != t) &&
                         gloo_dialogs::confirm(&format!("Time travel? You will burn {}⏲ and have to raise in your current timeline", num_burn)) && game.try_raise_or_bet(timeline_from) {
                            let initiating_player = game.get_active_player();
                            game.players[initiating_player].chips -= num_burn;
                            let card = {
                                let from_turn = game.timelines[timeline_from].boards[t_from]
                                .get_turn_mut(
                                    Some(turn_limit).and_then(|x| Some(x + 1)),
                                );
                                if player < game.players.len() {
                                    from_turn.player_states[player].hand.remove(i)
                                } else {
                                    from_turn.open_cards.remove(i)
                                }
                            };
                            let (timeline_num, t) =
                                if game.timelines[timeline_num].boards[t]
                                    .is_past(Some(turn_limit))
                                {
                                    game.spawn_timeline(timeline_num, t)
                                } else {
                                    (timeline_num, t)
                                };
                            let to_turn = game.timelines[timeline_num].boards[t].get_turn_mut(Some(turn_limit));
                            if player < game.players.len() {
                                to_turn.player_states[player].hand.push(card);
                            } else {
                                to_turn.open_cards.push(card);
                            }
                            ongameupdate.emit(game);
                        }
                    }
                };
                let onbuttonclick = {
                    let game = props.game.clone();
                    let zeroed = game.timelines[timeline_num].boards[t]
                        .get_turn(Some(game.get_turn()))
                        .bet_amount
                        == 0;
                    let ongamechange = props.ongameupdate.clone();
                    move |b: ButtonType| {
                        let mut game = game.clone();
                        use ButtonType::*;
                        match b {
                            CallOrCheck => {
                                if zeroed {
                                    game.try_check(timeline_num);
                                } else {
                                    game.try_call(timeline_num);
                                }
                            }
                            RaiseOrBet => {
                                if zeroed {
                                    game.try_initial_bet(timeline_num);
                                } else {
                                    game.try_raise(timeline_num);
                                }
                            }
                            Fold => game.fold(timeline_num),
                            DoNothing => game.skip(timeline_num, t),
                            ToggleView => {
                                let new_view =
                                    !game.timelines[timeline_num].boards[t].1;
                                game.timelines[timeline_num].boards[t].1 =
                                    new_view;
                            }
                        }
                        ongamechange.emit(game);
                    }
                };
                let board = board.clone();
                let turn_limit = if board.1 {
                    None
                } else {
                    Some(props.game.get_turn())
                };
                boards.push(html! {
                    <BoardDisplay
                        board={board}
                        {turn_limit}
                        coordinates={(t + timeline.starting_time, timeline_num)}
                        active_player={props.game.get_active_player()}
                        {ondragstart}
                        {ondrop}
                        {onbuttonclick}
                        players={props.game.players.clone()}
                    />
                });
            }
        }
        html! {
            {for boards}
        }
    } else {
        let onclick = {
            let game = props.game.clone();
            let ongameupdate = props.ongameupdate.clone();
            move |_e: MouseEvent| {
                let mut game = game.clone();
                game.active_player = game.get_active_player();
                ongameupdate.emit(game);
            }
        };
        html! { // hide the board
            <div class="table centered">
                <div class="player-name" style="animation: none;">
                    <div style="width:200px; padding: 10px;">
                        {format!("It is {}'s turn", props.game.players[props.game.get_active_player()].name)}
                    </div>
                </div>
                <button class="start-game do-button" {onclick}>{"Start Turn"}</button>
            </div>
        }
    }
}
