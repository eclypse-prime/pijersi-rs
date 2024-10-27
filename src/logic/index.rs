//! Implements the Index trait with helper methods for board indices.

use super::lookup::{NEIGHBOURS1, NEIGHBOURS2};

/// A cell index is represented as a usize.
pub type CellIndex = usize;

/// Bit width of a move index
pub const INDEX_WIDTH: usize = 8;
/// Value of a null index contained in a move
pub const INDEX_NULL: usize = 0xFFusize;
/// Mask to get the first index of a move (rightmost)
pub const INDEX_MASK: u64 = 0xFFu64;

/// Cell index trait for usize
pub trait CellIndexTrait: Copy {
    /// Returns true if the index if a null index (0xFF)
    fn is_null(self) -> bool;
    /// Returns true if the index is in the first row on white's side
    fn is_white_home(self) -> bool;
    /// Returns true if the index is in the first row on black's side
    fn is_black_home(self) -> bool;
    /// Returns an iterator to the 1-range neighbours of this index
    fn neighbours1(self) -> impl Iterator<Item = &'static Self>
    where
        Self: 'static;
    /// Returns an iterator to the 2-range neighbours of this index
    fn neighbours2(self) -> impl Iterator<Item = &'static Self>
    where
        Self: 'static;
}

impl CellIndexTrait for usize {
    #[inline(always)]
    fn is_null(self) -> bool {
        self == INDEX_NULL
    }

    #[inline(always)]
    fn is_white_home(self) -> bool {
        self >= 39
    }

    #[inline(always)]
    fn is_black_home(self) -> bool {
        self <= 5
    }

    #[inline(always)]
    fn neighbours1(self) -> impl Iterator<Item = &'static Self>
    where
        Self: 'static,
    {
        NEIGHBOURS1
            .iter()
            .skip(7 * self + 1)
            .take(NEIGHBOURS1[7 * self])
    }

    #[inline(always)]
    fn neighbours2(self) -> impl Iterator<Item = &'static Self>
    where
        Self: 'static,
    {
        NEIGHBOURS2
            .iter()
            .skip(7 * self + 1)
            .take(NEIGHBOURS2[7 * self])
    }
}
