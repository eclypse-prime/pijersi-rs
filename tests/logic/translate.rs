use pijersi_rs::{
    logic::translate::{
        action_to_indices, action_to_string, cells_to_pretty_string, cells_to_string,
        char_to_piece, coords_to_index, index_to_coords, index_to_string, piece_to_char,
        player_to_string, string_to_action, string_to_cells, string_to_player,
    },
    piece::{
        BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, CELL_EMPTY, WHITE_PAPER, WHITE_ROCK,
        WHITE_SCISSORS, WHITE_WISE,
    },
};

use crate::TEST_CELLS;

#[test]
fn test_char_to_piece() {
    let test_array = [
        ('-', Some(CELL_EMPTY)),
        ('S', Some(WHITE_SCISSORS)),
        ('P', Some(WHITE_PAPER)),
        ('R', Some(WHITE_ROCK)),
        ('W', Some(WHITE_WISE)),
        ('s', Some(BLACK_SCISSORS)),
        ('p', Some(BLACK_PAPER)),
        ('r', Some(BLACK_ROCK)),
        ('w', Some(BLACK_WISE)),
        ('?', None),
    ];

    for (input, output) in test_array {
        assert_eq!(char_to_piece(input), output);
    }
}

#[test]
fn test_piece_to_char() {
    let test_array = [
        (CELL_EMPTY, Some('-')),
        (WHITE_SCISSORS, Some('S')),
        (WHITE_PAPER, Some('P')),
        (WHITE_ROCK, Some('R')),
        (WHITE_WISE, Some('W')),
        (BLACK_SCISSORS, Some('s')),
        (BLACK_PAPER, Some('p')),
        (BLACK_ROCK, Some('r')),
        (BLACK_WISE, Some('w')),
        (u8::MAX, None),
    ];

    for (input, output) in test_array {
        assert_eq!(piece_to_char(input), output);
    }
}

#[test]
fn test_coords_to_index() {
    let test_array = [
        ((0, 0), 0),
        ((0, 1), 1),
        ((0, 2), 2),
        ((0, 3), 3),
        ((0, 4), 4),
        ((0, 5), 5),
        ((1, 0), 6),
        ((1, 1), 7),
        ((1, 2), 8),
        ((1, 3), 9),
        ((1, 4), 10),
        ((1, 5), 11),
        ((1, 6), 12),
        ((2, 0), 13),
        ((2, 1), 14),
        ((2, 2), 15),
        ((2, 3), 16),
        ((2, 4), 17),
        ((2, 5), 18),
        ((3, 0), 19),
        ((3, 1), 20),
        ((3, 2), 21),
        ((3, 3), 22),
        ((3, 4), 23),
        ((3, 5), 24),
        ((3, 6), 25),
        ((4, 0), 26),
        ((4, 1), 27),
        ((4, 2), 28),
        ((4, 3), 29),
        ((4, 4), 30),
        ((4, 5), 31),
        ((5, 0), 32),
        ((5, 1), 33),
        ((5, 2), 34),
        ((5, 3), 35),
        ((5, 4), 36),
        ((5, 5), 37),
        ((5, 6), 38),
        ((6, 0), 39),
        ((6, 1), 40),
        ((6, 2), 41),
        ((6, 3), 42),
        ((6, 4), 43),
        ((6, 5), 44),
    ];

    for ((i, j), output) in test_array {
        assert_eq!(coords_to_index(i, j), output);
    }
}

#[test]
fn test_index_to_coord() {
    let test_array = [
        (0, (0, 0)),
        (1, (0, 1)),
        (2, (0, 2)),
        (3, (0, 3)),
        (4, (0, 4)),
        (5, (0, 5)),
        (6, (1, 0)),
        (7, (1, 1)),
        (8, (1, 2)),
        (9, (1, 3)),
        (10, (1, 4)),
        (11, (1, 5)),
        (12, (1, 6)),
        (13, (2, 0)),
        (14, (2, 1)),
        (15, (2, 2)),
        (16, (2, 3)),
        (17, (2, 4)),
        (18, (2, 5)),
        (19, (3, 0)),
        (20, (3, 1)),
        (21, (3, 2)),
        (22, (3, 3)),
        (23, (3, 4)),
        (24, (3, 5)),
        (25, (3, 6)),
        (26, (4, 0)),
        (27, (4, 1)),
        (28, (4, 2)),
        (29, (4, 3)),
        (30, (4, 4)),
        (31, (4, 5)),
        (32, (5, 0)),
        (33, (5, 1)),
        (34, (5, 2)),
        (35, (5, 3)),
        (36, (5, 4)),
        (37, (5, 5)),
        (38, (5, 6)),
        (39, (6, 0)),
        (40, (6, 1)),
        (41, (6, 2)),
        (42, (6, 3)),
        (43, (6, 4)),
        (44, (6, 5)),
    ];

    for (input, output) in test_array {
        assert_eq!(index_to_coords(input), output);
    }
}

#[test]
fn test_index_to_string() {
    let test_array = [
        (0, "g1"),
        (1, "g2"),
        (2, "g3"),
        (3, "g4"),
        (4, "g5"),
        (5, "g6"),
        (6, "f1"),
        (7, "f2"),
        (8, "f3"),
        (9, "f4"),
        (10, "f5"),
        (11, "f6"),
        (12, "f7"),
        (13, "e1"),
        (14, "e2"),
        (15, "e3"),
        (16, "e4"),
        (17, "e5"),
        (18, "e6"),
        (19, "d1"),
        (20, "d2"),
        (21, "d3"),
        (22, "d4"),
        (23, "d5"),
        (24, "d6"),
        (25, "d7"),
        (26, "c1"),
        (27, "c2"),
        (28, "c3"),
        (29, "c4"),
        (30, "c5"),
        (31, "c6"),
        (32, "b1"),
        (33, "b2"),
        (34, "b3"),
        (35, "b4"),
        (36, "b5"),
        (37, "b6"),
        (38, "b7"),
        (39, "a1"),
        (40, "a2"),
        (41, "a3"),
        (42, "a4"),
        (43, "a5"),
        (44, "a6"),
    ];

    for (input, output) in test_array {
        assert!(index_to_string(input) == output);
    }
}

#[test]
fn test_string_to_action() {
    assert_eq!(string_to_action(&TEST_CELLS, "a1b1").unwrap(), 2107175);
    assert_eq!(string_to_action(&TEST_CELLS, "b1c1").unwrap(), 1769248);
    assert_eq!(string_to_action(&TEST_CELLS, "a1b1c1").unwrap(), 1712167);
    assert_eq!(string_to_action(&TEST_CELLS, "b4c5c5").unwrap(), 2031395);
    assert_eq!(string_to_action(&TEST_CELLS, "b4b4c5").unwrap(), 1975075);
    assert_eq!(string_to_action(&TEST_CELLS, "b4c3d4").unwrap(), 1448995);
    assert_eq!(string_to_action(&TEST_CELLS, "b7b6c6").unwrap(), 2041126);

    assert!(string_to_action(&TEST_CELLS, "a1b1c1d1").is_err());
    assert!(string_to_action(&TEST_CELLS, "z1a1a1").is_err());
    assert!(string_to_action(&TEST_CELLS, "a9a1a1").is_err());
    assert!(string_to_action(&TEST_CELLS, "a1").is_err());
    assert!(string_to_action(&TEST_CELLS, "??????").is_err());
}

#[test]
fn test_action_to_string() {
    let test_array = [
        (2107175, "a1b1"),
        (1769248, "b1c1"),
        (1712167, "a1b1c1"),
        (2031395, "b4c5c5"),
        (1975075, "b4b4c5"),
        (1448995, "b4c3d4"),
        (2041126, "b7b6c6"),
    ];
    for (input, output) in test_array {
        assert_eq!(action_to_string(&TEST_CELLS, input), output);
    }
}

#[test]
fn test_string_to_cells() {
    assert_eq!(
        string_to_cells("s-p-r-s-p-r-/p-r-s-wwr-s-p-/6/7/6/P-S-R-WWS-R-PS/R-P-S-R-P-1").unwrap(),
        TEST_CELLS
    );
    assert!(string_to_cells("s-p-r-s-p-r-/p-r-s-wwr-s-p-/6/7/6/P-S-R-WWS-R-PS").is_err());
    assert!(
        string_to_cells("s-p-r-s-p-r-/p-r-s-wwr-s-p-/6/7/6/P-S-R-WWS-R-PS/R-P-S-R-P-1/7").is_err()
    );
    assert!(string_to_cells("").is_err());
}

#[test]
fn test_cells_to_string() {
    assert_eq!(
        cells_to_string(&TEST_CELLS),
        "s-p-r-s-p-r-/p-r-s-wwr-s-p-/6/7/6/P-S-R-WWS-R-PS/R-P-S-R-P-1"
    )
}

#[test]
fn test_cells_to_pretty_string() {
    assert_eq!(cells_to_pretty_string(&TEST_CELLS), " s- p- r- s- p- r- \np- r- s- ww r- s- p- \n .  .  .  .  .  .  \n.  .  .  .  .  .  .  \n .  .  .  .  .  .  \nP- S- R- WW S- R- SP \n R- P- S- R- P- .  ");
}

#[test]
fn test_string_to_player() {
    assert_eq!(string_to_player("w").unwrap(), 0u8);
    assert_eq!(string_to_player("b").unwrap(), 1u8);
    assert!(string_to_player("?").is_err());
}

#[test]
fn test_player_to_string() {
    assert_eq!(player_to_string(0u8).unwrap(), "w");
    assert_eq!(player_to_string(1u8).unwrap(), "b");
    assert!(player_to_string(255u8).is_err());
}

#[test]
fn test_action_to_indices() {
    let test_array = [
        (2107175, (39, 39, 32)),
        (1769248, (32, 255, 26)),
        (1712167, (39, 32, 26)),
        (2031395, (35, 255, 30)),
        (1975075, (35, 35, 30)),
        (1448995, (35, 28, 22)),
        (2041126, (38, 37, 31)),
    ];

    for (input, output) in test_array {
        assert_eq!(action_to_indices(input), output);
    }
}
