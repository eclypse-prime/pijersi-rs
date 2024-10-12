use pijersi_rs::{
    logic::rules::{can_move1, can_move2, can_stack, can_take, can_unstack},
    piece::{
        BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, HALF_PIECE_WIDTH, WHITE_PAPER, WHITE_ROCK, WHITE_SCISSORS, WHITE_WISE
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
const TEST_CELLS2: [u8; 45] = [
    3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 183, 5, 1, 9, 221, 1, 89, 81, 9, 5, 1, 9, 0, 0,
];

/// Cells state for testing
///  s- p- r- s- .  r-
/// p- r- s- ww .  s- p-
///  .  .  .  .  .  .
/// .  .  .  .  .  pr .
///  .  .  .  .  .  .
/// P- S- R- WW PS R- SP
///  R- P- S- R- .  .
const TEST_CELLS3: [u8; 45] = [
    3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 0, 0, 0, 0,
    0, 0, 0, 5, 1, 9, 221, 21, 9, 81, 9, 5, 1, 9, 0, 0,
];

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