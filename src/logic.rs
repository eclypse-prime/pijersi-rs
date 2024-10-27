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
/// A board is represented as a [u8; 45] array
pub type Cells = [u8; N_CELLS];
/// An empty board
pub const CELLS_EMPTY: Cells = [0u8; N_CELLS];

/// Max number of half moves without capture before draw
pub const MAX_HALF_MOVES: u64 = 20;
