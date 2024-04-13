use std::time::Instant;

use pijersi_rs::board::Board;
use pijersi_rs::logic::actions::play_action;
use pijersi_rs::logic::rules::is_action_win;
use pijersi_rs::logic::translate::action_to_string;
use pijersi_rs::search::alphabeta::search;

fn main() {
    let mut board: Board = Board::new();
    board.init();

    loop {
        let action =
            search(&board.cells, board.current_player, 4);
        println!("{}", action_to_string(&board.cells, action));
        if is_action_win(&board.cells, action) {
            play_action(&mut board.cells, action);
            board.print();
            println!("win");
            break;
        } else {
            play_action(&mut board.cells, action);
            board.current_player = 1 - board.current_player;
            board.print();
        }
    }
}
