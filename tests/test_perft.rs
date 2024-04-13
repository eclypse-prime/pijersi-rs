use pijersi_rs::board::Board;
use pijersi_rs::logic::perft::perft;
use pijersi_rs::logic::perft::perft_iter;

#[test]
fn test_perft_iter() {
    let mut board = Board::new();
    board.init();
    assert_eq!(perft_iter(&board.cells, board.current_player, 1), 186);
    assert_eq!(perft_iter(&board.cells, board.current_player, 2), 34054);
    assert_eq!(perft_iter(&board.cells, board.current_player, 3), 6410472);
}

#[test]
fn test_perft() {
    let mut board = Board::new();
    board.init();
    assert_eq!(perft(&board.cells, board.current_player, 1), 186);
    assert_eq!(perft(&board.cells, board.current_player, 2), 34054);
    assert_eq!(perft(&board.cells, board.current_player, 3), 6410472);
    assert_eq!(perft(&board.cells, board.current_player, 4), 1181445032);
}