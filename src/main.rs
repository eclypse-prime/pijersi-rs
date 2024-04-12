use std::time::Instant;

use pijersi_rs::board::Board;
use pijersi_rs::logic::perft::perft;

fn main() {
    let mut board: Board = Board::new();
    board.init();

    let start = Instant::now();
    for _ in 0..10 {
        let results = perft(&board.cells, board.current_player, 4);
        println!("result {results}");
    }
    let elapsed = start.elapsed();
    println!("Rust time {:.3?}", elapsed);
}
