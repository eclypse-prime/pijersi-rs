//! Implements the actions a player can choose (move, stack, unstack...).
//!
//! An action is stored as a u32 value. Its contents are divided into the following sections:
//!
//! | Data  | Depth (optional) | Third index | Second index | First index |
//! |-------|------------------|-------------|--------------|-------------|
//! | Width | 8                | 8           | 8            | 8           |

use std::{
    ops::{Index, IndexMut, Range, RangeFull},
    sync::atomic::AtomicU32,
};

use crate::{
    bitboard::Board,
    piece::{Piece, PieceTrait},
};

use super::{
    index::{CellIndex, CellIndexTrait, INDEX_MASK, INDEX_WIDTH},
    translate::action_to_string,
};

/// Size of the array that stores player actions
pub const MAX_PLAYER_ACTIONS: usize = 512;
/// Size of the array that stores player actions
pub const MAX_PLAYER_CAPTURES: usize = 128;

/// An action is stored as a u32 value. See [`crate::logic::actions`] for the specific data format.
pub type Action = u32;
/// An atomic action is stored as a AtomicU32 value.
pub type AtomicAction = AtomicU32;

/// Mask to get the action without additional data
pub const ACTION_MASK: Action = 0x00FF_FFFF;

/// `ActionTrait` trait for `Action`
pub trait ActionTrait: Copy {
    /// Converts an action to its indices
    fn to_indices(self) -> (CellIndex, CellIndex, CellIndex);
    /// Converts a set of three indices to an action
    fn from_indices(index_start: CellIndex, index_mid: CellIndex, index_end: CellIndex) -> Self;
    /// Converts a set of two starting indices (without the end index) to an action
    fn from_indices_half(index_start: CellIndex, index_mid: CellIndex) -> Self;
    /// Returns the search depth stored in the action data
    fn search_depth(self) -> u64;
    /// Adds the last index of an action to itself
    fn add_last_index(self, index_end: CellIndex) -> Self;
}

impl ActionTrait for Action {
    // TODO: can we make this even more generic by implementing From and Into for Action and Indices?
    #[inline(always)]
    fn to_indices(self) -> (CellIndex, CellIndex, CellIndex) {
        let index_start: CellIndex = (self & INDEX_MASK) as CellIndex;
        let index_mid: CellIndex = ((self >> INDEX_WIDTH) & INDEX_MASK) as CellIndex;
        let index_end: CellIndex = ((self >> (2 * INDEX_WIDTH)) & INDEX_MASK) as CellIndex;
        (index_start, index_mid, index_end)
    }

    #[inline(always)]
    /// Concatenate three indices into a `Action`.
    /// The first index is stored in the 8 least significant bits.
    fn from_indices(index_start: CellIndex, index_mid: CellIndex, index_end: CellIndex) -> Self {
        (index_start | (index_mid << INDEX_WIDTH) | (index_end << (2 * INDEX_WIDTH))) as Self
    }

    #[inline(always)]
    fn from_indices_half(index_start: CellIndex, index_mid: CellIndex) -> Self {
        (index_start | (index_mid << INDEX_WIDTH)) as Self
    }

    #[inline(always)]
    fn search_depth(self) -> u64 {
        #[allow(clippy::unnecessary_cast)]
        {
            ((self >> (3 * INDEX_WIDTH)) & 0xFF) as u64
        }
    }

    /// Concatenate a half action and the last index into a `Action`.
    /// The first index is stored in the 8 least significant bits.
    #[inline(always)]
    fn add_last_index(self, index_end: CellIndex) -> Self {
        self | (index_end << (2 * INDEX_WIDTH)) as Self
    }
}

/// This struct is a fixed-length array that stores player actions.
/// By default, this struct contains 512 actions.
#[derive(Debug, Clone, Copy)]
pub struct Actions<const N: usize = MAX_PLAYER_ACTIONS> {
    data: [Action; N],
    current_index: usize,
}

/// This struct is an alias to a lower-size [`Actions`].
/// It is used specifically to store available captures.
pub type ActionsLight = Actions<MAX_PLAYER_CAPTURES>;

impl<const N: usize> Actions<N> {
    /// Store a new action
    #[inline]
    pub fn push(&mut self, value: Action) {
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

    /// Print the actions
    #[inline]
    pub fn print(&self) {
        println!("{:?}", self[..].iter().collect::<Vec<&u32>>());
    }

    /// Print the actions in string format
    #[inline]
    pub fn print_str(&self, board: &Board) {
        println!(
            "{:?}",
            self[..]
                .iter()
                .map(|&action| { action_to_string(board, action) })
                .collect::<Vec<String>>()
        );
    }
}

impl<const N: usize> From<&[Action]> for Actions<N> {
    fn from(value: &[Action]) -> Self {
        let mut data = [0; N];
        let current_index = value.len();
        assert!(current_index < N);
        data[..current_index].copy_from_slice(value);
        Actions {
            data,
            current_index,
        }
    }
}

impl<const N: usize> Default for Actions<N> {
    fn default() -> Self {
        Actions {
            data: [0; N],
            current_index: 0,
        }
    }
}

impl<const N: usize> IntoIterator for Actions<N> {
    type Item = Action;
    type IntoIter = std::iter::Take<std::array::IntoIter<Action, N>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().take(self.current_index)
    }
}

impl<const N: usize> PartialEq for Actions<N> {
    fn eq(&self, other: &Self) -> bool {
        self.current_index == other.current_index && self.data == other.data
    }
}

impl<const N: usize> Index<usize> for Actions<N> {
    type Output = Action;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> IndexMut<usize> for Actions<N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> Index<Range<usize>> for Actions<N> {
    type Output = [Action];
    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> IndexMut<Range<usize>> for Actions<N> {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> Index<RangeFull> for Actions<N> {
    type Output = [Action];
    #[inline]
    fn index(&self, _index: RangeFull) -> &Self::Output {
        &self.data[0..self.current_index]
    }
}

impl<const N: usize> IndexMut<RangeFull> for Actions<N> {
    #[inline]
    fn index_mut(&mut self, _index: RangeFull) -> &mut Self::Output {
        &mut self.data[0..self.current_index]
    }
}

impl Board {
    /// Applies a move between chosen coordinates.
    pub fn do_move(&mut self, index_start: CellIndex, index_end: CellIndex) {
        let start_piece = self.get_piece(index_start);
        self.unset_piece(index_start, start_piece);
        self.remove_piece(index_end);
        self.set_piece(index_end, start_piece);
    }

    /// Applies a stack between chosen coordinates.
    pub fn do_stack(&mut self, index_start: CellIndex, index_end: CellIndex) {
        let piece_start = self.get_piece(index_start);
        let piece_end = self.get_piece(index_end);

        self.unset_piece(index_start, piece_start);
        self.unset_piece(index_end, piece_end);

        if piece_start.bottom() != 0 {
            self.set_piece(index_start, piece_start.bottom());
        }
        self.set_piece(index_end, piece_start.stack_on(piece_end));
    }

    /// Applies an unstack between chosen coordinates.
    pub fn do_unstack(&mut self, index_start: CellIndex, index_end: CellIndex) {
        let piece_start: Piece = self.get_piece(index_start);

        self.unset_piece(index_start, piece_start);
        self.remove_piece(index_end);

        if piece_start.bottom() != 0 {
            self.set_piece(index_start, piece_start.bottom());
        }

        self.set_piece(index_end, piece_start.top());
    }

    /// Plays the selected action.
    pub fn play_action(&mut self, action: Action) {
        let (index_start, index_mid, index_end) = action.to_indices();

        if index_start.is_null() {
            return;
        }

        let piece_start: Piece = self.get_piece(index_start);

        if !piece_start.is_empty() {
            // If there is no intermediate move
            if index_mid.is_null() {
                // Simple move
                self.do_move(index_start, index_end);
            } else {
                let piece_mid: Piece = self.get_piece(index_mid);
                let piece_end: Piece = self.get_piece(index_end);
                // The piece at the mid coordinates is an ally : stack and move
                if !piece_mid.is_empty()
                    && piece_mid.colour() == piece_start.colour()
                    && (index_start != index_mid)
                {
                    self.do_stack(index_start, index_mid);
                    self.do_move(index_mid, index_end);
                }
                // The piece at the end coordinates is an ally : move and stack
                else if !piece_end.is_empty() && piece_end.colour() == piece_start.colour() {
                    self.do_move(index_start, index_mid);
                    self.do_stack(index_mid, index_end);
                }
                // The end coordinates contain an enemy or no piece : move and unstack
                else {
                    self.do_move(index_start, index_mid);
                    self.do_unstack(index_mid, index_end);
                }
            }
        }
    }
}
