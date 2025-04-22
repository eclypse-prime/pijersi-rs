use pijersi_rs::{bitboard::Board, hash::position::HashTrait};

use crate::{TEST_BOARD_STR, TEST_BOARD_STR2};

/// Asserts that equal positions and starting players give the same hash.
/// Asserts that different positions and/or different players give different hashes.
#[test]
fn test_to_hash() {
    let test_board = Board::try_from(TEST_BOARD_STR).unwrap();
    let test_board2 = Board::try_from(TEST_BOARD_STR2).unwrap();
    assert_eq!((&test_board, 0).hash(), (&test_board, 0).hash());
    assert_eq!((&test_board, 1).hash(), (&test_board, 1).hash());
    assert_eq!((&test_board2, 0).hash(), (&test_board2, 0).hash());
    assert_eq!((&test_board2, 1).hash(), (&test_board2, 1).hash());
    assert_ne!((&test_board, 0).hash(), (&test_board2, 0).hash());
    assert_ne!((&test_board, 1).hash(), (&test_board2, 1).hash());
    assert_ne!((&test_board, 0).hash(), (&test_board, 1).hash());
    assert_ne!((&test_board, 1).hash(), (&test_board, 0).hash());
    assert_ne!((&test_board2, 0).hash(), (&test_board2, 1).hash());
    assert_ne!((&test_board2, 1).hash(), (&test_board2, 0).hash());
}
