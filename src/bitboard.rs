//! Implements the code to create and manipulate bitboards.

use std::ops::{BitAnd, BitOr, Index, IndexMut, Not};

use crate::{
    logic::{index::CellIndex, Player},
    piece::{
        Piece, PieceTrait, BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, HALF_PIECE_WIDTH,
        WHITE_PAPER, WHITE_ROCK, WHITE_SCISSORS, WHITE_WISE,
    },
};

const N_BITBOARDS: usize = 16;

/// This struct represents a 64 bit (only 45 are used) bitboard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bitboard(pub u64);

/// This struct uses bitboards to represent the board and its pieces.
///
/// It contains 16 bitboards in the following order:
///
/// | Index | Position | Color | Piece    |
/// |-------|----------|-------|----------|
/// | 0     | Top      | White | Scissors |
/// | 1     | Top      | White | Paper    |
/// | 2     | Top      | White | Rock     |
/// | 3     | Top      | White | Wise     |
/// | 4     | Top      | Black | Scissors |
/// | 5     | Top      | Black | Paper    |
/// | 6     | Top      | Black | Rock     |
/// | 7     | Top      | Black | Wise     |
/// | 8     | Bottom   | White | Scissors |
/// | 9     | Bottom   | White | Paper    |
/// | 10    | Bottom   | White | Rock     |
/// | 11    | Bottom   | White | Wise     |
/// | 12    | Bottom   | Black | Scissors |
/// | 13    | Bottom   | Black | Paper    |
/// | 14    | Bottom   | Black | Rock     |
/// | 15    | Bottom   | Black | Wise     |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Board(pub [Bitboard; N_BITBOARDS]);

impl Iterator for Bitboard {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1;
            Some(index)
        }
    }
}

impl Bitboard {
    /// An null (invalid) bitboard.
    pub const NULL: Self = Self(u64::MAX);
    /// An empty bitboard.
    pub const EMPTY: Self = Self(0);

    /// Returns true if there is a set (1) bit at the given index.
    #[inline(always)]
    pub fn get(&self, index: CellIndex) -> bool {
        (self.0 >> index) & 1 == 1
    }

    /// Sets the bit to 1 at the given index.
    #[inline(always)]
    pub fn set(&mut self, index: CellIndex) {
        let mask = 1 << index;
        self.0 |= mask;
    }

    /// Sets the bit to 0 at the given index.
    #[inline(always)]
    pub fn unset(&mut self, index: CellIndex) {
        let mask = !(1 << index);
        self.0 &= mask;
    }

    /// Flips the bit (from 0 to 1 or from 1 to 0) at the given index.
    #[inline(always)]
    pub fn flip(&mut self, index: CellIndex) {
        let mask = 1 << index;
        self.0 ^= mask;
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Index<CellIndex> for Board {
    type Output = Bitboard;
    fn index(&self, index: CellIndex) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<CellIndex> for Board {
    fn index_mut(&mut self, index: CellIndex) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Board {
    /// An empty board
    pub const EMPTY: Self = Self([Bitboard::EMPTY; N_BITBOARDS]);

    /// Returns a bitboard representing all pieces on the board
    pub fn all(&self) -> Bitboard {
        self.white() | self.black()
    }
    /// Returns a bitboard representing the white pieces on the board
    pub fn white(&self) -> Bitboard {
        self[0] | self[1] | self[2] | self[3]
    }
    /// Returns a bitboard representing the black pieces on the board
    pub fn black(&self) -> Bitboard {
        self[4] | self[5] | self[6] | self[7]
    }

    /// Returns a bitboard representing the white pieces that aren't wise (only scissors, paper, and rock) on the board
    pub fn white_not_wise(&self) -> Bitboard {
        self[0] | self[1] | self[2]
    }

    /// Returns a bitboard representing the black pieces that aren't wise (only scissors, paper, and rock) on the board
    pub fn black_not_wise(&self) -> Bitboard {
        self[4] | self[5] | self[6]
    }

    /// Returns a bitboard representing the stacks
    pub fn stacks(&self) -> Bitboard {
        self[8] | self[9] | self[10] | self[11] | self[12] | self[13] | self[14] | self[15]
    }

    /// Returns a bitboard representing the pieces that are the same colour as the given player.
    pub fn same_colour(&self, player: Player) -> Bitboard {
        if player == 0 {
            self.white()
        } else {
            self.black()
        }
    }

    /// Returns a bitboard representing the pieces that are the same colour as the given player.
    pub fn same_colour_not_wise(&self, player: Player) -> Bitboard {
        if player == 0 {
            self.white_not_wise()
        } else {
            self.black_not_wise()
        }
    }

    /// Returns a bitboard representing the stacks that are the same colour as the given player.
    pub fn same_stacks(&self, player: Player) -> Bitboard {
        if player == 0 {
            self[8] | self[9] | self[10] | self[11]
        } else {
            self[12] | self[13] | self[14] | self[15]
        }
    }

    /// Returns a bitboard representing the stacks that are the opposite colour to the given player.
    pub fn opposite_stacks(&self, player: Player) -> Bitboard {
        if player == 1 {
            self[8] | self[9] | self[10] | self[11]
        } else {
            self[12] | self[13] | self[14] | self[15]
        }
    }

    /// Returns a bitboard representing the wise pieces that are the same colour as the given player.
    pub fn same_wise(&self, player: Player) -> Bitboard {
        if player == 0 {
            self[3]
        } else {
            self[7]
        }
    }

    /// Puts a piece at the given index in the board.
    pub fn set_piece(&mut self, index: CellIndex, piece: Piece) {
        self[(piece & 0b0111) as usize].set(index);
        if piece.is_stack() {
            self[(piece >> HALF_PIECE_WIDTH) as usize].set(index);
        }
    }

    /// Removes the given piece from the given index in the board.
    pub fn unset_piece(&mut self, index: CellIndex, piece: Piece) {
        self[(piece & 0b0111) as usize].unset(index);
        if piece.is_stack() {
            self[(piece >> HALF_PIECE_WIDTH) as usize].unset(index);
        }
    }

    /// Returns the piece from the given index in the board.
    pub fn get_piece(&self, index: usize) -> Piece {
        let mut piece = 0;
        for k in 0..8 {
            if self[k].get(index) {
                piece = k as Piece | 0b1000;
                break;
            }
        }
        for k in 8..N_BITBOARDS {
            if self[k].get(index) {
                piece |= (k << 4) as Piece | 0b1000_0000;
                break;
            }
        }
        piece
    }

    /// Returns the piece from the given index in the board knowing its owner (player).
    ///
    /// It is a bit more efficient because we only need to check half the bitboards.
    pub fn get_player_piece(&self, index: usize, player: Player) -> Piece {
        let mut piece = 0;
        if player == 0 {
            for k in 0..4 {
                if self[k].get(index) {
                    piece = k as Piece | 0b1000;
                    break;
                }
            }
            for k in 8..12 {
                if self[k].get(index) {
                    piece |= (k << 4) as Piece | 0b1000_0000;
                    break;
                }
            }
        } else {
            for k in 4..8 {
                if self[k].get(index) {
                    piece = k as Piece | 0b1000;
                    break;
                }
            }
            for k in 12..16 {
                if self[k].get(index) {
                    piece |= (k << 4) as Piece | 0b1000_0000;
                    break;
                }
            }
        }
        piece
    }

    /// Removes any piece from the given index in the board.
    pub fn remove_piece(&mut self, index: CellIndex) {
        let piece = self.get_piece(index);
        self.unset_piece(index, piece);
    }

    /// Initializes the the board to the starting configuration.
    ///
    /// Sets the pieces to their original position.
    pub fn init(&mut self) {
        self.0 = [Bitboard::EMPTY; N_BITBOARDS];

        self.set_piece(0, BLACK_SCISSORS);
        self.set_piece(1, BLACK_PAPER);
        self.set_piece(2, BLACK_ROCK);
        self.set_piece(3, BLACK_SCISSORS);
        self.set_piece(4, BLACK_PAPER);
        self.set_piece(5, BLACK_ROCK);
        self.set_piece(6, BLACK_PAPER);
        self.set_piece(7, BLACK_ROCK);
        self.set_piece(8, BLACK_SCISSORS);
        self.set_piece(9, BLACK_WISE.stack_on(BLACK_WISE));
        self.set_piece(10, BLACK_ROCK);
        self.set_piece(11, BLACK_SCISSORS);
        self.set_piece(12, BLACK_PAPER);

        self.set_piece(44, WHITE_SCISSORS);
        self.set_piece(43, WHITE_PAPER);
        self.set_piece(42, WHITE_ROCK);
        self.set_piece(41, WHITE_SCISSORS);
        self.set_piece(40, WHITE_PAPER);
        self.set_piece(39, WHITE_ROCK);
        self.set_piece(38, WHITE_PAPER);
        self.set_piece(37, WHITE_ROCK);
        self.set_piece(36, WHITE_SCISSORS);
        self.set_piece(35, WHITE_WISE.stack_on(WHITE_WISE));
        self.set_piece(34, WHITE_ROCK);
        self.set_piece(33, WHITE_SCISSORS);
        self.set_piece(32, WHITE_PAPER);
    }
}
