//! This module implements the alphabeta search algorithm that chooses the best move and relevant evaluation functions.

pub mod alphabeta;
pub mod eval;
pub mod lookup;
pub mod openings;

/// The score is represented by a i32 value.
pub type Score = i32;
