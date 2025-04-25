//! Implements the Index trait with helper methods for board indices.

use super::actions::Action;

/// A cell index is represented as a usize.
pub type CellIndex = usize;

/// Bit width of an action index
pub const INDEX_WIDTH: usize = 8;
/// Value of a null index contained in an action
pub const INDEX_NULL: CellIndex = 0xFFusize;
/// Mask to get the first index of an action (rightmost)
pub const INDEX_MASK: Action = 0xFF;

/// Cell index trait for usize
pub trait CellIndexTrait: Copy {
    /// Returns true if the index if a null index (0xFF)
    fn is_null(self) -> bool;
    /// Returns true if the index is in the first row on white's side
    fn is_white_home(self) -> bool;
    /// Returns true if the index is in the first row on black's side
    fn is_black_home(self) -> bool;
}

impl CellIndexTrait for usize {
    #[inline(always)]
    fn is_null(self) -> bool {
        self > 44
    }

    #[inline(always)]
    fn is_white_home(self) -> bool {
        self >= 39
    }

    #[inline(always)]
    fn is_black_home(self) -> bool {
        self <= 5
    }
}
