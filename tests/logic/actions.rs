use pijersi_rs::bitboard::Board;

use crate::TEST_BOARD_STR;

#[test]
fn test_play_action() {
    let test_array = [
        (2107175, "s-p-r-s-..r-p-r-s-ww..s-p-......................pr..............RPS-R-WWS-R-SP..P-S-R-P-.."),
        (1769248, "s-p-r-s-..r-p-r-s-ww..s-p-......................pr..P-............S-R-WWS-R-SPR-P-S-R-P-.."),
        (1712167, "s-p-r-s-..r-p-r-s-ww..s-p-......................pr..RP............S-R-WWS-R-SP..P-S-R-P-.."),
        (2031395, "s-p-r-s-..r-p-r-s-ww..s-p-......................pr..........WW..P-S-R-..S-R-SPR-P-S-R-P-.."),
        (1975075, "s-p-r-s-..r-p-r-s-ww..s-p-......................pr..........W-..P-S-R-W-S-R-SPR-P-S-R-P-.."),
        (1448995, "s-p-r-s-..r-p-r-s-ww..s-p-..................W-..pr......W-......P-S-R-..S-R-SPR-P-S-R-P-.."),
        (2041126, "s-p-r-s-..r-p-r-s-ww..s-p-......................pr............SRP-S-R-WWS-..P-R-P-S-R-P-.."),
    ];

    let test_board = Board::try_from(TEST_BOARD_STR).unwrap();
    for (input, output) in test_array {
        let mut board = test_board;
        board.play_action(input);
        assert_eq!(board.to_string(), output);
    }
}
