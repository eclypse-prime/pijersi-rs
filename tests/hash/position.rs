use pijersi_rs::hash::position::HashTrait;

use crate::{TEST_CELLS, TEST_CELLS2};

/// Asserts that equal positions and starting players give the same hash.
/// Asserts that different positions and/or different players give different hashes.
#[test]
fn test_to_hash() {
    assert_eq!((&TEST_CELLS, 0).hash(), (&TEST_CELLS, 0).hash());
    assert_eq!((&TEST_CELLS, 1).hash(), (&TEST_CELLS, 1).hash());
    assert_eq!((&TEST_CELLS2, 0).hash(), (&TEST_CELLS2, 0).hash());
    assert_eq!((&TEST_CELLS2, 1).hash(), (&TEST_CELLS2, 1).hash());
    assert_ne!((&TEST_CELLS, 0).hash(), (&TEST_CELLS2, 0).hash());
    assert_ne!((&TEST_CELLS, 1).hash(), (&TEST_CELLS2, 1).hash());
    assert_ne!((&TEST_CELLS, 0).hash(), (&TEST_CELLS, 1).hash());
    assert_ne!((&TEST_CELLS, 1).hash(), (&TEST_CELLS, 0).hash());
    assert_ne!((&TEST_CELLS2, 0).hash(), (&TEST_CELLS2, 1).hash());
    assert_ne!((&TEST_CELLS2, 1).hash(), (&TEST_CELLS2, 0).hash());
}
