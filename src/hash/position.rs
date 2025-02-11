//! This module implements the traits and methods used to hash a position.

use crate::logic::lookup::PIECE_TO_INDEX;
use crate::piece::PieceTrait;

use crate::logic::{Cells, Player, N_CELLS};

use super::lookup::{PLAYER_HASH, ZOBRIST_TABLE};

/// `HashTrait` trait for `Cells`
pub trait HashTrait {
    /// Converts the cells into a hash that can be used to index a transposition table.
    fn hash(&self) -> usize;
}

impl HashTrait for (&Cells, Player) {
    fn hash(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter(|(_index, piece)| !piece.is_empty())
            .map(|(index, &piece)| ZOBRIST_TABLE[PIECE_TO_INDEX[piece as usize] * N_CELLS + index])
            .fold(if self.1 == 1 { PLAYER_HASH } else { 0 }, |acc, e| acc ^ e)
    }
}
