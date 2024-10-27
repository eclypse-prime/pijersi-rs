//! Implements the actions a player can choose (move, stack, unstack...).
//!
//! An action is stored as a u64 value. Its contents are divided into the following sections:
//!
//! | Data  | Empty | Depth (optional) | Third index | Second index | First index |
//! |-------|-------|------------------|-------------|--------------|-------------|
//! | Width | 32    | 8                | 8           | 8            | 8           |

use crate::piece::Piece;

use super::{
    index::{CellIndexTrait, INDEX_MASK, INDEX_WIDTH},
    Cells,
};

/// Size of the array that stores player actions
pub const MAX_PLAYER_ACTIONS: usize = 512;

/// An action is stored as a u64 value. See [`crate::logic::actions`] for the specific data format.
pub type Action = u64;

/// Mask to get the action without additional data
pub const ACTION_MASK: u64 = 0x00FF_FFFF_u64;

/// Applies a move between chosen coordinates.
fn do_move(cells: &mut Cells, index_start: usize, index_end: usize) {
    if index_start != index_end {
        // Move the piece to the target cell
        cells[index_end] = cells[index_start];

        // Set the starting cell as empty
        cells[index_start].set_empty();
    }
}

/// Applies a stack between chosen coordinates.
fn do_stack(cells: &mut Cells, index_start: usize, index_end: usize) {
    let piece_start: u8 = cells[index_start];
    let piece_end: u8 = cells[index_end];

    // If the moving piece is already on top of a stack, leave the bottom piece in the starting cell
    cells[index_start] = piece_start.bottom();

    // Move the top piece to the target cell and set its new bottom piece
    cells[index_end] = piece_start.stack_on(piece_end);
}

/// Applies an unstack between chosen coordinates.
fn do_unstack(cells: &mut Cells, index_start: usize, index_end: usize) {
    let piece_start: u8 = cells[index_start];

    // Leave the bottom piece in the starting cell
    cells[index_start] = piece_start.bottom();

    // Remove the bottom piece from the moving piece
    // Move the top piece to the target cell
    // Will overwrite the eaten piece if there is one
    cells[index_end] = piece_start.top();
}

/// Plays the selected action.
pub fn play_action(cells: &mut Cells, action: u64) {
    let (index_start, index_mid, index_end) = action.to_indices();

    if index_start.is_null() {
        return;
    }

    let piece_start: u8 = cells[index_start];

    if !piece_start.is_empty() {
        // If there is no intermediate move
        if index_mid.is_null() {
            // Simple move
            do_move(cells, index_start, index_end);
        } else {
            let piece_mid: u8 = cells[index_mid];
            let piece_end: u8 = cells[index_end];
            // The piece at the mid coordinates is an ally : stack and move
            if !piece_mid.is_empty()
                && piece_mid.colour() == piece_start.colour()
                && (index_start != index_mid)
            {
                do_stack(cells, index_start, index_mid);
                do_move(cells, index_mid, index_end);
            }
            // The piece at the end coordinates is an ally : move and stack
            else if !piece_end.is_empty() && piece_end.colour() == piece_start.colour() {
                do_move(cells, index_start, index_mid);
                do_stack(cells, index_mid, index_end);
            }
            // The end coordinates contain an enemy or no piece : move and unstack
            else {
                do_move(cells, index_start, index_mid);
                do_unstack(cells, index_mid, index_end);
            }
        }
    }
}

/// Action trait for u64
pub trait ActionTrait: Copy {
    /// Converts an action to its indices
    fn to_indices(self) -> (usize, usize, usize);
    /// Converts a set of three indices to an action
    fn from_indices(index_start: usize, index_mid: usize, index_end: usize) -> Self;
    /// Converts a set of two starting indices (without the end index) to an action
    fn from_indices_half(index_start: usize, index_mid: usize) -> Self;
    /// Returns the search depth stored in the action data
    fn search_depth(self) -> u64;
    /// Adds the last index of an action to itself
    fn add_last_index(self, index_end: usize) -> Self;
}

impl ActionTrait for u64 {
    // TODO: can we make this even more generic by implementing From and Into for Action and Indices?
    #[inline(always)]
    fn to_indices(self) -> (usize, usize, usize) {
        let index_start: usize = (self & INDEX_MASK) as usize;
        let index_mid: usize = ((self >> INDEX_WIDTH) & INDEX_MASK) as usize;
        let index_end: usize = ((self >> (2 * INDEX_WIDTH)) & INDEX_MASK) as usize;
        (index_start, index_mid, index_end)
    }

    #[inline(always)]
    /// Concatenate three indices into a u64 action.
    /// The first index is stored in the 8 least significant bits.
    fn from_indices(index_start: usize, index_mid: usize, index_end: usize) -> Self {
        (index_start | (index_mid << INDEX_WIDTH) | (index_end << (2 * INDEX_WIDTH))) as Self
    }

    #[inline(always)]
    fn from_indices_half(index_start: usize, index_mid: usize) -> Self {
        (index_start | (index_mid << INDEX_WIDTH)) as Self
    }

    #[inline(always)]
    fn search_depth(self) -> u64 {
        (self >> (3 * INDEX_WIDTH)) & 0xFF
    }

    /// Concatenate a half action and the last index into a u64 action.
    /// The first index is stored in the 8 least significant bits.
    #[inline(always)]
    fn add_last_index(self, index_end: usize) -> Self {
        self | (index_end << (2 * INDEX_WIDTH)) as Self
    }
}

/// This struct is a fixed-length array that stores player actions
#[derive(Debug)]
pub struct Actions {
    data: [Action; MAX_PLAYER_ACTIONS],
    current_index: usize,
}

impl Actions {
    /// Store a new action
    #[inline]
    pub fn push(&mut self, value: u64) {
        self.data[self.current_index] = value;
        self.current_index += 1;
    }

    /// Return the number of stored actions
    #[inline]
    pub fn len(&self) -> usize {
        self.current_index
    }

    /// Returns whether the number of actions is zero
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<&[Action]> for Actions {
    fn from(value: &[Action]) -> Self {
        let mut data = [0; MAX_PLAYER_ACTIONS];
        let current_index = value.len();
        assert!(current_index < MAX_PLAYER_ACTIONS);
        data[..current_index].copy_from_slice(value);
        Actions {
            data,
            current_index,
        }
    }
}

impl Default for Actions {
    fn default() -> Self {
        Actions {
            data: [0; MAX_PLAYER_ACTIONS],
            current_index: 0,
        }
    }
}

impl IntoIterator for Actions {
    type Item = u64;
    type IntoIter = std::array::IntoIter<u64, 512>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl PartialEq for Actions {
    fn eq(&self, other: &Self) -> bool {
        self.current_index == other.current_index && self.data == other.data
    }
}

impl std::ops::Index<usize> for Actions {
    type Output = u64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Actions {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
