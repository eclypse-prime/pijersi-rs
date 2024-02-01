mod board;

fn main() {
    let mut board: board::Board = board::Board::new();
    board.init();
    board.print();
}
