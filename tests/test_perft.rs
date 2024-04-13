use pijersi_rs::board::Board;
use pijersi_rs::logic::perft::perft;
use pijersi_rs::logic::perft::perft_iter;
use pijersi_rs::logic::perft::perft_split;

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

#[test]
fn test_perft_split() {
    let mut board = Board::new();
    board.init();
    assert_eq!(perft_split(&board.cells, board.current_player, 1).iter().map(|result| result.2).sum::<u64>(), 186);
    assert_eq!(perft_split(&board.cells, board.current_player, 2).iter().map(|result| result.2).sum::<u64>(), 34054);
    assert_eq!(perft_split(&board.cells, board.current_player, 3).iter().map(|result| result.2).sum::<u64>(), 6410472);
    assert_eq!(perft_split(&board.cells, board.current_player, 4).iter().map(|result| result.2).sum::<u64>(), 1181445032);
}
