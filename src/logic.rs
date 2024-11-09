//! This module contains the necessary code to implement the game logic.

use crate::piece::Piece;

pub mod actions;
pub mod index;
pub mod lookup;
pub mod movegen;
pub mod perft;
pub mod rules;
pub mod translate;

/// The number of cells in a board
pub const N_CELLS: usize = 45;
/// A board is represented as a `[Piece; 45]` array
pub type Cells = [Piece; N_CELLS];
/// A player is represented as a `u8`: `0u8` for White, and `1u8` for Black
pub type Player = u8;
/// An empty board
pub const CELLS_EMPTY: Cells = [0; N_CELLS];

/// Max number of half moves without capture before draw
pub const MAX_HALF_MOVES: u64 = 20;
