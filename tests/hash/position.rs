use pijersi_rs::hash::position::HashTrait;

use crate::{TEST_CELLS, TEST_CELLS2};

#[test]
fn test_to_hash() {
    assert_eq!(TEST_CELLS.hash(), TEST_CELLS.hash());
    assert_ne!(TEST_CELLS.hash(), TEST_CELLS2.hash());
}
