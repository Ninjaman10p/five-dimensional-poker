use yew::prelude::*;
use crate::*;
use crate::board_display::*;

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

                        let turn_limit = game.get_turn();
                        if game.try_raise(timeline_from, 1) {
                            // TODO
                            log::info!("hi");
                            let card = game.timelines[timeline_from].boards
                                [t_from]
                                .get_turn_mut(Some(turn_limit))
                                .player_states[player]
                                .hand
                                .remove(i);
                            let (timeline_num, t) =
                                if game.timelines[timeline_num].boards[t]
                                    .is_past(Some(turn_limit))
                                {
                                    log::info!("already in the future");
                                    game.spawn_timeline(timeline_num, t)
                                } else {
                                    (timeline_num, t)
                                };
                            game.timelines[timeline_num].boards[t].get_turn_mut(Some(turn_limit))
                                .player_states[player]
                                .hand
                                .push(card);
                            ongameupdate.emit(game);
                        }
                    }
                };
                let onbuttonclick = {
                    let game = props.game.clone();
                    let ongamechange = props.ongameupdate.clone();
                    move |b: ButtonType| {
                        log::info!("hi");
                        let mut game = game.clone();
                        use ButtonType::*;
                        match b {
                            Call => {
                                game.try_call(timeline_num);
                            }
                            Raise => {
                                game.try_raise(timeline_num, 1);
                            }
                            Fold => game.fold(timeline_num),
                        }
                        ongamechange.emit(game);
                    }
                };
                let board = board.clone();
                boards.push(html! {
                    <BoardDisplay
                        board={board}
                        turn_limit={Some(props.game.get_turn())}
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
                log::info!("{:?}", game);
                ongameupdate.emit(game);
            }
        };
        html! { // hide the board
            <div class="table centered">
                <div class="player-name">
                    <div style="width:200px; padding: 10px;">
                        {format!("It is {}'s turn", props.game.players[props.game.get_active_player()].name)}
                    </div>
                </div>
                <button class="start-game do-button" {onclick}>{"Start Turn"}</button>
            </div>
        }
    }
}
