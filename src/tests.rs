use crate::board::Board;
use crate::logic::movegen::perft;

#[test]
fn test_perft() {
    let mut board = Board::new();
    board.init();
    assert_eq!(perft(&board.cells, board.current_player, 1), 186);
    assert_eq!(perft(&board.cells, board.current_player, 2), 34054);
    assert_eq!(perft(&board.cells, board.current_player, 3), 6410472);
    assert_eq!(perft(&board.cells, board.current_player, 4), 1181445032);
}