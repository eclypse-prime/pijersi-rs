use pijersi_rs::logic::actions::play_action;

use crate::TEST_CELLS;

#[test]
fn test_play_action() {
    let test_array = [
        (
            2107175,
            [
                3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183,
                0, 0, 0, 0, 0, 0, 0, 89, 1, 9, 221, 1, 9, 81, 0, 5, 1, 9, 5, 0,
            ],
        ),
        (
            1769248,
            [
                3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183,
                0, 5, 0, 0, 0, 0, 0, 0, 1, 9, 221, 1, 9, 81, 9, 5, 1, 9, 5, 0,
            ],
        ),
        (
            1712167,
            [
                3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183,
                0, 89, 0, 0, 0, 0, 0, 0, 1, 9, 221, 1, 9, 81, 0, 5, 1, 9, 5, 0,
            ],
        ),
        (
            2031395,
            [
                3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183,
                0, 0, 0, 0, 0, 221, 0, 5, 1, 9, 0, 1, 9, 81, 9, 5, 1, 9, 5, 0,
            ],
        ),
        (
            1975075,
            [
                3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183,
                0, 0, 0, 0, 0, 13, 0, 5, 1, 9, 13, 1, 9, 81, 9, 5, 1, 9, 5, 0,
            ],
        ),
        (
            1448995,
            [
                3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 183,
                0, 0, 0, 13, 0, 0, 0, 5, 1, 9, 0, 1, 9, 81, 9, 5, 1, 9, 5, 0,
            ],
        ),
        (
            2041126,
            [
                3, 7, 11, 3, 0, 11, 7, 11, 3, 255, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183,
                0, 0, 0, 0, 0, 0, 145, 5, 1, 9, 221, 1, 0, 5, 9, 5, 1, 9, 5, 0,
            ],
        ),
    ];

    for (input, output) in test_array {
        let mut cells = TEST_CELLS;
        play_action(&mut cells, input);
        assert_eq!(cells, output);
    }
}
