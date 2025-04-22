use pijersi_rs::{
    bitboard::Board,
    logic::rules::{is_action_legal, is_action_win},
};

/// Cells state for testing
/// startpos > a6b7 g5f5d6 b6a5b6 d6c6c6
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  pr
/// P- S- R- WW S- RP SP
///  R- P- S- R- .  .
const TEST_BOARD_2_STR: &str =
    "s-p-r-s-..r-p-r-s-ww..s-p-....................................prP-S-R-WWS-RPSPR-P-S-R-....";

/// Cells state for testing
/// startpos > a6b7 g5f5d6 a5b5
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW PS R- SP
///  R- P- S- R- .  .
const TEST_BOARD_3_STR: &str =
    "s-p-r-s-..r-p-r-s-ww..s-p-......................pr..............P-S-R-WWPSR-SPR-P-S-R-....";

/// Cells state for testing
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  .
/// P- S- R- WW S- R- P-
///  R- P- S- R- P- S-
const TEST_BOARD_STALEMATE_STR: &str =
    "................................................................P-S-R-WWS-R-P-R-P-S-R-P-S-";

/// Cells state for testing
///  s- p- r- s- S  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW PS R- SP
///  R- P- S- R- .  .
const TEST_BOARD_WHITE_WIN_STR: &str =
    "s-p-r-s-S-r-p-r-s-ww..s-p-......................pr..............P-S-R-WWPSR-SPR-P-S-R-....";

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW PS R- SP
///  R- P- S- R- s  .
const TEST_BOARD_BLACK_WIN_STR: &str =
    "s-p-r-s-..r-p-r-s-ww..s-p-......................pr..............P-S-R-WWPSR-SPR-P-S-R-s-..";

#[test]
fn test_is_action_win() {
    let test_board_2 = Board::try_from(TEST_BOARD_2_STR).unwrap();
    assert!(!is_action_win(&test_board_2, 1975583));
    assert!(is_action_win(&test_board_2, 2893087));
}

#[test]
fn test_is_action_legal() {
    let test_board_2 = Board::try_from(TEST_BOARD_2_STR).unwrap();
    assert!(!is_action_legal(
        &test_board_2,
        0,
        37 | (0xFF << 8) | (31 << 16)
    ));
    assert!(is_action_legal(
        &test_board_2,
        0,
        38 | (0xFF << 8) | (31 << 16)
    ));
}

#[test]
fn test_is_position_win() {
    let test_board_2 = Board::try_from(TEST_BOARD_2_STR).unwrap();
    let test_board_black_win = Board::try_from(TEST_BOARD_BLACK_WIN_STR).unwrap();
    let test_board_white_win = Board::try_from(TEST_BOARD_WHITE_WIN_STR).unwrap();
    assert!(!test_board_2.is_win());
    assert!(test_board_black_win.is_win());
    assert!(test_board_white_win.is_win());
}

#[test]
fn test_is_position_stalemate() {
    let test_board_2 = Board::try_from(TEST_BOARD_2_STR).unwrap();
    let test_board_stalemate = Board::try_from(TEST_BOARD_STALEMATE_STR).unwrap();
    assert!(!test_board_2.is_stalemate(0));
    assert!(test_board_stalemate.is_stalemate(1));
    assert!(!test_board_stalemate.is_stalemate(0));
    assert!(Board::EMPTY.is_stalemate(0));
    assert!(Board::EMPTY.is_stalemate(1));
}

#[test]
fn test_get_winning_player() {
    let test_board_2 = Board::try_from(TEST_BOARD_2_STR).unwrap();
    let test_board_white_win = Board::try_from(TEST_BOARD_WHITE_WIN_STR).unwrap();
    let test_board_black_win = Board::try_from(TEST_BOARD_BLACK_WIN_STR).unwrap();
    let test_array = [
        (test_board_2, None),
        (test_board_white_win, Some(0)),
        (test_board_black_win, Some(1)),
    ];

    for (input, output) in test_array {
        assert_eq!(input.get_winner(), output);
    }
}
