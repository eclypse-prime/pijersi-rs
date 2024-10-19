//! This module contains the necessary code to implement the game logic.

pub mod actions;
pub mod index;
pub mod lookup;
pub mod movegen;
pub mod perft;
pub mod rules;
pub mod translate;

/// Max number of half moves without capture before draw
pub const MAX_HALF_MOVES: u64 = 20;
