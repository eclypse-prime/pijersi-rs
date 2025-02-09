//! This module implements the alphabeta search algorithm that chooses the best move and relevant evaluation functions.

use std::sync::atomic::AtomicI16;

pub mod alphabeta;
pub mod eval;
pub mod lookup;
pub mod openings;

/// The score is represented by a i16 value.
pub type Score = i16;
/// The atomic score is represented by a AtomicI16 value.
pub type AtomicScore = AtomicI16;

/// The type of the node. It is used to determine if the score is exact, lower-bound, or higher bound.
/// See <https://www.chessprogramming.org/Node_Types>
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NodeType {
    /// PV (Principal Variation) node: all actions searched, returned score is exact
    #[default]
    PV,
    /// Cut node (fail-high node): at least one action searched, returned score is a lower bound
    Cut,
    /// All node (fail-low node): all actions searched, returned score is an upper-bound
    All,
}
