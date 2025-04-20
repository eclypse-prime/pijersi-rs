use std::ops::{BitAnd, BitOr, Index, IndexMut, Not};

use crate::{
    logic::{
        actions::{Action, ActionTrait},
        index::{CellIndex, CellIndexTrait, INDEX_NULL},
        lookup::{BLOCKER_MASKS, MAGICS, NEIGHBOURS1, NEIGHBOURS2},
        translate::{coords_to_index, piece_to_char},
        Player, N_CELLS,
    },
    piece::{
        Piece, PieceTrait, BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, COLOUR_MASK,
        HALF_PIECE_WIDTH, TYPE_MASK, WHITE_PAPER, WHITE_ROCK, WHITE_SCISSORS, WHITE_WISE,
    },
};

const N_BITBOARDS: usize = 16;

const BLACK_WIN_MASK: u64 = 0b000000000000000000000000000000000000000111111;
const WHITE_WIN_MASK: u64 = 0b111111000000000000000000000000000000000000000;

/// S P R W top
/// S P R W bottom
/// s p r w top
/// s p r w bottom
#[derive(Clone, Copy)]
pub struct Bitboard(pub u64);

#[derive(Clone, Copy)]
pub struct Board([Bitboard; N_BITBOARDS]);

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
    pub const NULL: Self = Self(u64::MAX);
    pub const EMPTY: Self = Self(0);

    #[inline(always)]
    pub fn get(&self, index: CellIndex) -> bool {
        (self.0 >> index) & 1 == 1
    }

    #[inline(always)]
    pub fn set(&mut self, index: CellIndex) {
        let mask = 1 << index;
        self.0 |= mask;
    }

    #[inline(always)]
    pub fn unset(&mut self, index: CellIndex) {
        let mask = !(1 << index);
        self.0 &= mask;
    }

    #[inline(always)]
    pub fn flip(&mut self, index: CellIndex) {
        let mask = 1 << index;
        self.0 ^= mask;
    }

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
    pub const EMPTY: Self = Self([Bitboard::EMPTY; N_BITBOARDS]);

    pub fn all(&self) -> Bitboard {
        self.white() | self.black()
    }
    pub fn white(&self) -> Bitboard {
        self[0] | self[1] | self[2] | self[3]
    }
    pub fn black(&self) -> Bitboard {
        self[4] | self[5] | self[6] | self[7]
    }

    pub fn white_not_wise(&self) -> Bitboard {
        self[0] | self[1] | self[2]
    }
    pub fn black_not_wise(&self) -> Bitboard {
        self[4] | self[5] | self[6]
    }

    pub fn colour(&self, player: Player) -> Bitboard {
        if player == 0 {
            self.white()
        } else {
            self.black()
        }
    }

    pub fn same_bottom(&self, player: Player) -> Bitboard {
        if player == 0 {
            self[8] | self[9] | self[10] | self[11]
        } else {
            self[12] | self[13] | self[14] | self[15]
        }
    }

    pub fn same_wise(&self, player: Player) -> Bitboard {
        if player == 0 {
            self[3]
        } else {
            self[7]
        }
    }

    pub fn victim(&self, piece: Piece) -> Bitboard {
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

    pub fn set_piece(&mut self, index: CellIndex, piece: Piece) {
        self[(piece & 0b0111) as usize].set(index);
        if piece.is_stack() {
            self[(piece >> HALF_PIECE_WIDTH) as usize].set(index);
        }
    }

    pub fn unset_piece(&mut self, index: CellIndex, piece: Piece) {
        self[(piece & 0b0111) as usize].unset(index);
        if piece.is_stack() {
            self[(piece >> HALF_PIECE_WIDTH) as usize].unset(index);
        }
    }

    pub fn get_piece(&self, index: usize) -> Piece {
        let mut piece = 0u8;
        for k in 0..8 {
            if self[k].get(index) {
                piece = k as u8 | 0b1000;
                break;
            }
        }
        for k in 8..N_BITBOARDS {
            if self[k].get(index) {
                piece |= (k << 4) as u8 | 0b1000_0000;
                break;
            }
        }
        piece
    }

    pub fn remove_piece(&mut self, index: CellIndex) {
        let piece = self.get_piece(index);
        self.unset_piece(index, piece);
    }

    pub fn available_moves2(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let blockers = BLOCKER_MASKS[index] & !self.all();
        blockers.get_magic(index) & (!self.all() | self.victim(piece))
    }

    pub fn available_moves1(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        neighbours & (!self.all() | self.victim(piece))
    }

    pub fn available_stacks(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        let player = piece.colour() >> 2;
        neighbours
            & (if piece.is_wise() {
                self.same_wise(player)
            } else {
                self.colour(player)
            } & !self.same_bottom(player))
    }

    pub fn available_unstacks(&self, index: CellIndex, piece: Piece) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        neighbours & (!self.all() | self.victim(piece))
    }

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

    pub fn get_winner(&self) -> Option<Player> {
        if (self.white_not_wise() & Bitboard(WHITE_WIN_MASK)).0 != 0 {
            Some(0)
        } else if (self.black_not_wise() & Bitboard(BLACK_WIN_MASK)).0 != 0 {
            Some(1)
        } else {
            None
        }
    }

    pub fn play_action(&mut self, action: Action) {
        let (index_start, index_mid, index_end) = action.to_indices();

        if index_start.is_null() {
            return;
        }

        let piece_start: Piece = self.get_piece(index_start);

        if !piece_start.is_empty() {
            // If there is no intermediate move
            if index_mid.is_null() {
                // Simple move
                do_move_bb(self, index_start, index_end);
            } else {
                let piece_mid: Piece = self.get_piece(index_mid);
                let piece_end: Piece = self.get_piece(index_end);
                // The piece at the mid coordinates is an ally : stack and move
                if !piece_mid.is_empty()
                    && piece_mid.colour() == piece_start.colour()
                    && (index_start != index_mid)
                {
                    do_stack_bb(self, index_start, index_mid);
                    do_move_bb(self, index_mid, index_end);
                }
                // The piece at the end coordinates is an ally : move and stack
                else if !piece_end.is_empty() && piece_end.colour() == piece_start.colour() {
                    do_move_bb(self, index_start, index_mid);
                    do_stack_bb(self, index_mid, index_end);
                }
                // The end coordinates contain an enemy or no piece : move and unstack
                else {
                    do_move_bb(self, index_start, index_mid);
                    do_unstack_bb(self, index_mid, index_end);
                }
            }
        }
    }

    fn is_action_win(&self, action: Action) -> bool {
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

    pub fn to_pretty_string(&self) -> String {
        let mut pretty_string = " ".to_owned();
        for i in 0..45 {
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

pub fn do_move_bb(board: &mut Board, index_start: CellIndex, index_end: CellIndex) {
    let start_piece = board.get_piece(index_start);
    board.unset_piece(index_start, start_piece);
    board.remove_piece(index_end);
    board.set_piece(index_end, start_piece);
}

pub fn do_stack_bb(board: &mut Board, index_start: CellIndex, index_end: CellIndex) {
    let piece_start = board.get_piece(index_start);
    let piece_end = board.get_piece(index_end);

    board.unset_piece(index_start, piece_start);
    board.unset_piece(index_end, piece_end);

    if piece_start.bottom() != 0 {
        board.set_piece(index_start, piece_start.bottom());
    }
    board.set_piece(index_end, piece_start.stack_on(piece_end));
}

pub fn do_unstack_bb(board: &mut Board, index_start: CellIndex, index_end: CellIndex) {
    let piece_start: Piece = board.get_piece(index_start);

    board.unset_piece(index_start, piece_start);
    board.remove_piece(index_end);

    if piece_start.bottom() != 0 {
        board.set_piece(index_start, piece_start.bottom());
    }

    board.set_piece(index_end, piece_start.top());
}

pub fn perft_iter_bb(board: &Board, current_player: Player, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 => board.count_player_actions(current_player),
        _ => {
            let available_actions = board.available_player_actions(current_player);

            available_actions
                .into_iter()
                .filter(|&action| !board.is_action_win(action))
                .map(|action| {
                    let mut new_board = *board;
                    new_board.play_action(action);
                    perft_iter_bb(&new_board, 1 - current_player, depth - 1)
                })
                .sum()
        }
    }
}

pub fn perft_bb(board: &Board, current_player: Player, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 => board.count_player_actions(current_player),
        _ => {
            let available_actions = board.available_player_actions(current_player);

            available_actions
                .into_iter()
                // .par_bridge()
                .filter(|&action| !board.is_action_win(action))
                .map(|action| {
                    let mut new_board = *board;
                    new_board.play_action(action);
                    perft_iter_bb(&new_board, 1 - current_player, depth - 1)
                })
                .sum()
        }
    }
}
