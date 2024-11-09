//! This module implements the traits and methods used to hash a position.

use crate::logic::lookup::PIECE_TO_INDEX;
use crate::piece::PieceTrait;

use crate::logic::{Cells, N_CELLS};

use super::lookup::ZOBRIST_TABLE;

/// `HashTrait` trait for `Cells`
pub trait HashTrait {
    /// Converts the cells into a hash that can be used to index a transposition table.
    fn hash(&self) -> usize;
}

impl HashTrait for Cells {
    fn hash(&self) -> usize {
        self.iter()
            .enumerate()
            .filter(|(_index, piece)| !piece.is_empty())
            .map(|(index, &piece)| ZOBRIST_TABLE[PIECE_TO_INDEX[piece as usize] * N_CELLS + index])
            .reduce(|acc, e| acc ^ e)
            .unwrap()
    }
}
