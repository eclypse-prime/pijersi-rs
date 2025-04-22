//! Implements the code to create and manipulate bitboards.

use std::ops::{BitAnd, BitOr, Index, IndexMut, Not};

use crate::{
    logic::{
        actions::{Action, ActionTrait},
        index::{CellIndex, CellIndexTrait, INDEX_NULL},
        lookup::{BLOCKER_MASKS, MAGICS, NEIGHBOURS1},
        Player, N_CELLS,
    },
    piece::{
        Piece, PieceTrait, BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, COLOUR_MASK,
        HALF_PIECE_WIDTH, TYPE_MASK, WHITE_PAPER, WHITE_ROCK, WHITE_SCISSORS, WHITE_WISE,
    },
};

const N_BITBOARDS: usize = 16;

const WHITE_WIN_MASK: u64 = 0b000000000000000000000000000000000000000111111;
const BLACK_WIN_MASK: u64 = 0b111111000000000000000000000000000000000000000;

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

    /// When used on a bitboard of blockers, this function returns a bitboard of available 2-range moves.
    pub fn get_magic(&self, index: CellIndex) -> Bitboard {
        let (magic, ref table) = MAGICS[index];
        let magic_hash = self.0.wrapping_mul(magic.0);
        let magic_index = (magic_hash >> (64 - 6)) as usize;
        table[magic_index]
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

    /// Returns a bitboard representing the wise pieces that are the same colour as the given player.
    pub fn same_wise(&self, player: Player) -> Bitboard {
        if player == 0 {
            self[3]
        } else {
            self[7]
        }
    }

    /// Returns a bitboard representing the pieces that the given piece can capture.
    pub fn victims(&self, piece: Piece) -> Bitboard {
        match piece & (COLOUR_MASK | TYPE_MASK) {
            0b000 => self[5],
            0b001 => self[6],
            0b010 => self[4],
            0b100 => self[1],
            0b101 => self[2],
            0b110 => self[0],
            _ => Bitboard(0),
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

    /// Returns a bitboard with the available range-2 moves for the piece at the given index.
    pub fn available_moves2(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let blockers = BLOCKER_MASKS[index] & !self.all();
        blockers.get_magic(index) & (!self.all() | self.victims(piece))
    }

    /// Returns a bitboard with the available range-1 moves for the piece at the given index.
    pub fn available_moves1(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        neighbours & (!self.all() | self.victims(piece))
    }

    /// Returns a bitboard with the available stacks for the piece at the given index.
    pub fn available_stacks(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        let player = piece.colour() >> 2;
        neighbours
            & (if piece.is_wise() {
                self.same_wise(player)
            } else {
                self.same_colour(player)
            } & !self.same_stacks(player))
    }

    /// Returns a bitboard with the available unstacks for the piece at the given index.
    /// An unstack actually follows the same rules as a 1-range move
    pub fn available_unstacks(&self, index: CellIndex, piece: Piece) -> Bitboard {
        self.available_moves1(index, piece)
    }

    /// Returns a bitboard with the available range-1 captures (moves or unstacks) for the piece at the given index.
    pub fn available_captures1(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        neighbours & self.victims(piece)
    }

    /// Returns a bitboard with the available range-2 captures for the piece at the given index.
    pub fn available_captures2(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let blockers = BLOCKER_MASKS[index] & !self.all();
        blockers.get_magic(index) & self.victims(piece)
    }

    /// Returns a bitboard with the available range-1 captures (moves or unstacks) for the piece at the given index.
    pub fn available_non_captures1(&self, index: CellIndex) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        neighbours & !self.all()
    }

    /// Returns a bitboard with the available range-2 captures for the piece at the given index.
    pub fn available_non_captures2(&self, index: CellIndex) -> Bitboard {
        let blockers = BLOCKER_MASKS[index] & !self.all();
        blockers.get_magic(index) & !self.all()
    }

    /// Returns true if the current position is winning for one of the players.
    pub fn is_win(&self) -> bool {
        (self.white_not_wise() & Bitboard(WHITE_WIN_MASK)).0 != 0
            || (self.black_not_wise() & Bitboard(BLACK_WIN_MASK)).0 != 0
    }

    /// Returns true if the current position is a stalemate for one of the players.
    ///
    /// This means one of the two players has no legal move left.
    pub fn is_stalemate(&self, current_player: Player) -> bool {
        self.count_player_actions(current_player) == 0
    }

    /// Returns the winning player if there is one.
    pub fn get_winner(&self) -> Option<Player> {
        if (self.white_not_wise() & Bitboard(WHITE_WIN_MASK)).0 != 0 {
            Some(0)
        } else if (self.black_not_wise() & Bitboard(BLACK_WIN_MASK)).0 != 0 {
            Some(1)
        } else {
            None
        }
    }

    /// Returns true if the chosen action leads to a win.
    ///
    /// To win, one allied piece (except wise) must reach the last row in the opposite side.
    pub fn is_action_win(&self, action: Action) -> bool {
        let (index_start, index_mid, index_end) = action.to_indices();

        let moving_piece: Piece = self.get_piece(index_start);

        !moving_piece.is_wise()
            && (index_mid != INDEX_NULL
                && ((moving_piece.is_white() && index_mid.is_black_home())
                    || (moving_piece.is_black() && index_mid.is_white_home()))
                || (moving_piece.is_white() && index_end.is_black_home())
                || (moving_piece.is_black() && index_end.is_white_home()))
    }

    /// Counts the number of pieces on the board.
    ///
    /// A stack counts as two pieces.
    pub fn count_pieces(&self) -> u64 {
        self.0
            .iter()
            .map(|bitboard| bitboard.0.count_ones() as u64)
            .sum()
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

    /// Converts the cells to a pretty formatted string.
    ///
    /// The starting position is represented as such:
    /// ```not_rust
    ///  s- p- r- s- p- r-
    /// p- r- s- ww r- s- p-
    ///  .  .  .  .  .  .  
    /// .  .  .  .  .  .  .  
    ///  .  .  .  .  .  .  
    /// P- S- R- WW S- R- P-
    ///  R- P- S- R- P- S-
    /// ```
    pub fn to_pretty_string(&self) -> String {
        let mut pretty_string = " ".to_owned();
        for i in 0..N_CELLS {
            let piece = self.get_piece(i);
            let top_piece: Piece = piece.top();
            let bottom_piece: Piece = piece.bottom();
            let char1: char = match top_piece {
                0b0000 => '.',
                0b1000 => 'S',
                0b1001 => 'P',
                0b1010 => 'R',
                0b1011 => 'W',
                0b1100 => 's',
                0b1101 => 'p',
                0b1110 => 'r',
                0b1111 => 'w',
                _ => '?',
            };
            let char2: char = if top_piece == 0 {
                ' '
            } else {
                match bottom_piece {
                    0b0000 => '-',
                    0b1000 => 'S',
                    0b1001 => 'P',
                    0b1010 => 'R',
                    0b1011 => 'W',
                    0b1100 => 's',
                    0b1101 => 'p',
                    0b1110 => 'r',
                    0b1111 => 'w',
                    _ => '?',
                }
            };
            pretty_string += &format!("{char1}{char2} ");

            if [5, 12, 18, 25, 31, 38].contains(&i) {
                pretty_string += "\n";
                if [12, 25, 38].contains(&i) {
                    pretty_string += " ";
                }
            }
        }

        pretty_string
    }
}
