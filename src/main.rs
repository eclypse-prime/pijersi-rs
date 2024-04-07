use std::time::Instant;

use pijersi_rs::board::Board;
use pijersi_rs::logic::movegen::perft;

fn main() {
    let mut board: Board = Board::new();
    board.init();

    let start = Instant::now();
    for _ in 0..5 {
        let results = perft(&board.cells, board.current_player, 4);
        println!("result {results}");
    }
    let elapsed = start.elapsed();
    println!("Rust time {:.2?}", elapsed);

}
