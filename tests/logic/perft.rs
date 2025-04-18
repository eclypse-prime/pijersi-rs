use pijersi_rs::board::Game;
use pijersi_rs::logic::perft::count_player_actions;
use pijersi_rs::logic::perft::perft;
use pijersi_rs::logic::perft::perft_split;

#[test]
fn test_count_player_actions() {
    let mut board = Game::new();
    board.init();
    assert_eq!(
        count_player_actions(&board.cells, board.current_player, 1),
        186
    );
    assert_eq!(
        count_player_actions(&board.cells, board.current_player, 2),
        34054
    );
    assert_eq!(
        count_player_actions(&board.cells, board.current_player, 3),
        6_410_472
    );
}

#[test]
fn test_perft() {
    let mut board = Game::new();
    board.init();
    assert_eq!(perft(&board.cells, board.current_player, 1), 186);
    assert_eq!(perft(&board.cells, board.current_player, 2), 34054);
    assert_eq!(perft(&board.cells, board.current_player, 3), 6_410_472);
    assert_eq!(perft(&board.cells, board.current_player, 4), 1_181_445_032);
}

#[test]
fn test_perft_split() {
    let mut board = Game::new();
    board.init();
    assert_eq!(
        perft_split(&board.cells, board.current_player, 1)
            .iter()
            .map(|result| result.2)
            .sum::<u64>(),
        186
    );
    assert_eq!(
        perft_split(&board.cells, board.current_player, 2)
            .iter()
            .map(|result| result.2)
            .sum::<u64>(),
        34054
    );
    assert_eq!(
        perft_split(&board.cells, board.current_player, 3)
            .iter()
            .map(|result| result.2)
            .sum::<u64>(),
        6_410_472
    );
    assert_eq!(
        perft_split(&board.cells, board.current_player, 4)
            .iter()
            .map(|result| result.2)
            .sum::<u64>(),
        1_181_445_032
    );
}
