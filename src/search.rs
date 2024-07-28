//! This module contains the necessary code to implement the alphabeta search algorithm that chooses the best move.
//!
//! It contains the following sub-modules:
//!
//! - alphabeta: Implements the alphabeta search that chooses the best move.
//! - eval: Implements the evaluation functions: evaluates the score of a current position or evaluates the best score at a given depth.
//! - lookup: Implements the lookup tables used for faster computations in the evaluation functions.
pub mod alphabeta;
pub mod eval;
pub mod lookup;
pub mod openings;
