use std::{io, process::exit, sync::Mutex};

use pijersi_rs::{
    board::Board, hash::search::SearchTable, logic::translate::action_to_string, ugi::UgiEngine,
};

/// Runs the UGI protocol engine
fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();

    // let mut ugi_engine = UgiEngine::new();
    // loop {
    //     let mut command = String::new();
    //     io::stdin()
    //         .read_line(&mut command)
    //         .expect("Failed to read command");
    //     command.truncate(command.trim_end().len());
    //     ugi_engine.get_command(&command);
    // }

    let tt = Mutex::new(SearchTable::default());

    let mut board = Board::default();
    board.init();
    // board.set_string_state("s-2s-p-r-/p-r-s-1r-s-p-/1w-1r-2/4RP2/2wp2PS/P-S-P-WWS-2/R-5 b 1 5").unwrap();
    board.options.verbose = false;

    while !board.is_win() && !board.is_draw() {
        println!("TT");
        let (action1, score1) = board.search_to_depth(5, None, Some(&tt)).unwrap();
        println!("No TT");
        let (action2, score2) = board.search_to_depth(5, None, None).unwrap();

        if action1 != action2 || score1 != score2 {
        // if action1 != action2 {
            println!("{}", board.get_string_state());
            board.print();
            println!("TT {} {score1}", action_to_string(&board.cells, action1));
            println!("NT {} {score2}", action_to_string(&board.cells, action2));
            break;
        }

        board.play(action1).unwrap();
    }

    board.options.verbose = true;
    println!("TT");
    board.search_to_depth(5, None, Some(&tt)).unwrap();
    println!("No TT");
    board.search_to_depth(5, None, None).unwrap();

    exit(0);
}
