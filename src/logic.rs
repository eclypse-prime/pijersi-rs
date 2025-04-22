//! This module contains the necessary code to implement the game logic.

pub mod actions;
pub mod index;
pub mod lookup;
pub mod movegen;
pub mod perft;
pub mod rules;
pub mod translate;

/// The number of cells in a board
pub const N_CELLS: usize = 45;
/// A player is represented as a `u8`: `0u8` for White, and `1u8` for Black
pub type Player = u8;

/// Max number of half moves without capture before draw
pub const MAX_HALF_MOVES: u64 = 20;
