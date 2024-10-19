//! This module contains the necessary code to implement the game logic.

pub mod actions;
pub mod index;
pub mod lookup;
pub mod movegen;
pub mod perft;
pub mod rules;
pub mod translate;

/// Bit width of a move index
pub const INDEX_WIDTH: usize = 8;
/// Value of a null index contained in a move
pub const INDEX_NULL: usize = 0xFFusize;
/// Mask to get the first index of a move (rightmost)
pub const INDEX_MASK: u64 = 0xFFu64;
/// Mask to get the action without additional data
pub const ACTION_MASK: u64 = 0xFFFFFFu64;

/// Size of the array that stores player actions
pub const MAX_PLAYER_ACTIONS: usize = 512;

/// Max number of half moves without capture before draw
pub const MAX_HALF_MOVES: u64 = 20;
