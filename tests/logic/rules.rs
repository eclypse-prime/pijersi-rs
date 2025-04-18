use pijersi_rs::{
    logic::{
        rules::{
            can_move1, can_move2, can_stack, can_take, can_unstack, get_winning_player,
            is_action_legal, is_action_win, is_position_stalemate, is_position_win,
        },
        Cells, CELLS_EMPTY,
    },
    piece::{
        BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, WHITE_PAPER, WHITE_ROCK,
        WHITE_SCISSORS, WHITE_WISE,
    },
};

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  pr
/// P- S- R- WW S- RP SP
///  R- P- S- R- .  .
const TEST_CELLS2: Cells = [12, 13, 14, 12, 0, 14, 13, 14, 12, 255, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 9, 8, 10, 187, 8, 154, 152, 10, 9, 8, 10, 0, 0];

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW PS R- SP
///  R- P- S- R- .  .
const TEST_CELLS3: Cells = [12, 13, 14, 12, 0, 14, 13, 14, 12, 255, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 0, 0, 0, 0, 0, 0, 0, 9, 8, 10, 187, 137, 10, 152, 10, 9, 8, 10, 0, 0];

/// Cells state for testing
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  .
/// .  .  .  .  .  .  .
///  .  .  .  .  .  .
/// P- S- R- WW S- R- P-
///  R- P- S- R- P- S-
const TEST_CELLS_STALEMATE: Cells = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 8, 10, 187, 8, 10, 9, 10, 9, 8, 10, 9, 8];

/// Cells state for testing
///  s- p- r- s- S  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW PS R- SP
///  R- P- S- R- .  .
const TEST_CELLS_WHITE_WIN: Cells = [12, 13, 14, 12, 8, 14, 13, 14, 12, 255, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 0, 0, 0, 0, 0, 0, 0, 9, 8, 10, 187, 137, 10, 152, 10, 9, 8, 10, 0, 0];

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW PS R- SP
///  R- P- S- R- s  .
const TEST_CELLS_BLACK_WIN: Cells = [12, 13, 14, 12, 0, 14, 13, 14, 12, 255, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 0, 0, 0, 0, 0, 0, 0, 9, 8, 10, 187, 137, 10, 152, 10, 9, 8, 10, 12, 0];

#[test]
fn test_can_take() {
    let test_array = [
        (WHITE_SCISSORS, BLACK_SCISSORS, false),
        (WHITE_SCISSORS, BLACK_PAPER, true),
        (WHITE_SCISSORS, BLACK_ROCK, false),
        (WHITE_SCISSORS, BLACK_WISE, false),
        (WHITE_PAPER, BLACK_SCISSORS, false),
        (WHITE_PAPER, BLACK_PAPER, false),
        (WHITE_PAPER, BLACK_ROCK, true),
        (WHITE_PAPER, BLACK_WISE, false),
        (WHITE_ROCK, BLACK_SCISSORS, true),
        (WHITE_ROCK, BLACK_PAPER, false),
        (WHITE_ROCK, BLACK_ROCK, false),
        (WHITE_ROCK, BLACK_WISE, false),
        (WHITE_WISE, BLACK_SCISSORS, false),
        (WHITE_WISE, BLACK_PAPER, false),
        (WHITE_WISE, BLACK_ROCK, false),
        (WHITE_WISE, BLACK_WISE, false),
    ];

    for (attacker, target, output) in test_array {
        assert_eq!(can_take(attacker, target), output);
    }
}

#[test]
fn test_can_move1() {
    assert!(can_move1(&TEST_CELLS3, TEST_CELLS3[38], 31));
    assert!(!can_move1(&TEST_CELLS3, TEST_CELLS3[38], 37));
    assert!(can_move1(&TEST_CELLS2, TEST_CELLS2[38], 31));
    assert!(!can_move1(&TEST_CELLS2, TEST_CELLS3[37], 31));
}

#[test]
fn test_can_move2() {
    assert!(can_move2(&TEST_CELLS3, TEST_CELLS3[38], 38, 24));
    assert!(!can_move2(&TEST_CELLS3, TEST_CELLS3[38], 38, 36));
    assert!(!can_move2(&TEST_CELLS3, TEST_CELLS3[36], 36, 24));
    assert!(can_move2(&TEST_CELLS3, TEST_CELLS3[36], 36, 22));
    assert!(!can_move2(&TEST_CELLS3, TEST_CELLS3[9], 9, 11));
}

#[test]
fn test_can_stack() {
    assert!(!can_stack(&TEST_CELLS3, TEST_CELLS3[37], 30));
    assert!(!can_stack(&TEST_CELLS2, TEST_CELLS2[37], 31));
    assert!(!can_stack(&TEST_CELLS3, TEST_CELLS3[37], 38));
    assert!(!can_stack(&TEST_CELLS3, TEST_CELLS3[35], 34));
    assert!(can_stack(&TEST_CELLS3, TEST_CELLS3[42], 41));
}

#[test]
fn test_can_unstack() {
    assert!(can_unstack(&TEST_CELLS3, TEST_CELLS3[38], 31));
    assert!(!can_unstack(&TEST_CELLS2, TEST_CELLS2[38], 37));
    assert!(can_unstack(&TEST_CELLS2, TEST_CELLS2[38], 31));
    assert!(!can_unstack(&TEST_CELLS2, TEST_CELLS2[37], 31));
}

#[test]
fn test_is_action_win() {
    assert!(!is_action_win(&TEST_CELLS2, 1975583));
    assert!(is_action_win(&TEST_CELLS2, 2893087));
}

#[test]
fn test_is_action_legal() {
    assert!(!is_action_legal(
        &TEST_CELLS2,
        0,
        37 | (0xFF << 8) | (31 << 16)
    ));
    assert!(is_action_legal(
        &TEST_CELLS2,
        0,
        38 | (0xFF << 8) | (31 << 16)
    ));
}

#[test]
fn test_is_position_win() {
    assert!(!is_position_win(&TEST_CELLS2));
    assert!(is_position_win(&TEST_CELLS_BLACK_WIN));
    assert!(is_position_win(&TEST_CELLS_WHITE_WIN));
}

#[test]
fn test_is_position_stalemate() {
    assert!(!is_position_stalemate(&TEST_CELLS2, 0));
    assert!(is_position_stalemate(&TEST_CELLS_STALEMATE, 1));
    assert!(!is_position_stalemate(&TEST_CELLS_STALEMATE, 0));
    assert!(is_position_stalemate(&CELLS_EMPTY, 0));
    assert!(is_position_stalemate(&CELLS_EMPTY, 1));
}

#[test]
fn test_get_winning_player() {
    let test_array = [
        (TEST_CELLS2, None),
        (TEST_CELLS_WHITE_WIN, Some(0)),
        (TEST_CELLS_BLACK_WIN, Some(1)),
    ];

    for (input, output) in test_array {
        assert_eq!(get_winning_player(&input), output);
    }
}
