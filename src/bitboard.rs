use crate::{
    board,
    logic::{
        actions::{Action, ActionTrait, Actions},
        index::{CellIndex, CellIndexTrait, INDEX_NULL},
        lookup::{BLOCKER_MASKS, MAGICS, NEIGHBOURS},
        translate::{coords_to_index, piece_to_char, piece_to_char2},
        Player, N_CELLS,
    },
    piece::{
        Piece, PieceColour, PieceTrait, PieceType, BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS,
        BLACK_WISE, COLOUR_MASK, HALF_PIECE_WIDTH, TOP_MASK, TYPE_MASK, WHITE_PAPER, WHITE_ROCK,
        WHITE_SCISSORS, WHITE_WISE,
    },
};

use rayon::prelude::*;

pub const NEIGHBOURS2: [Bitboard; 45] = [
    16388,
    40968,
    81937,
    163874,
    327684,
    131080,
    1048832,
    2621952,
    5243968,
    10487936,
    20975872,
    41943552,
    16778240,
    134250498,
    335609861,
    671227914,
    1342455828,
    2684387368,
    1073807376,
    8592031872,
    21479031104,
    42958586496,
    85917172992,
    171834345984,
    343601583104,
    137447344128,
    1099780079616,
    2749315981312,
    5498699071488,
    10997398142976,
    21990501318656,
    8796630024192,
    17180917760,
    34362359808,
    73019686912,
    146039373824,
    292078747648,
    34401681408,
    68736253952,
    2199157473280,
    4398382055424,
    9346519924736,
    18693039849472,
    2201707610112,
    4399120252928,
];

/// S P R W top
/// S P R W bottom
/// s p r w top
/// s p r w bottom
pub type Bitboard = u64;

pub type Board = [Bitboard; 16];

struct BitboardIterator(Bitboard);

impl Iterator for BitboardIterator {
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

pub trait BitboardTrait {
    fn get(&self, index: CellIndex) -> bool;
    fn set(&mut self, index: CellIndex);
    fn unset(&mut self, index: CellIndex);
    fn flip(&mut self, index: CellIndex);

    fn format_bits(&self) -> String;
    fn to_pretty_string(&self) -> String;
}

impl BitboardTrait for Bitboard {
    #[inline(always)]
    fn get(&self, index: CellIndex) -> bool {
        (self >> index) & 1 == 1
    }

    #[inline(always)]
    fn set(&mut self, index: CellIndex) {
        let mask = 1 << index;
        *self |= mask;
    }

    #[inline(always)]
    fn unset(&mut self, index: CellIndex) {
        let mask = !(1 << index);
        *self &= mask;
    }

    #[inline(always)]
    fn flip(&mut self, index: CellIndex) {
        let mask = 1 << index;
        *self ^= mask;
    }

    fn format_bits(&self) -> String {
        format!("{:045b}", self.reverse_bits() >> (64 - 45))
    }

    fn to_pretty_string(&self) -> String {
        let mut pretty_string = " ".to_owned();
        for i in 0..45 {
            pretty_string += if self.get(i) { "X  " } else { ".  " };
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

pub trait BoardTrait {
    fn all(&self) -> Bitboard;
    fn colour(&self, player: Player) -> Bitboard;
    fn white(&self) -> Bitboard;
    fn black(&self) -> Bitboard;

    fn same_wise(&self, player: Player) -> Bitboard;
    fn same_bottom(&self, player: Player) -> Bitboard;

    fn victim(&self, piece: Piece) -> Bitboard;

    fn set_piece(&mut self, index: CellIndex, piece: Piece);
    fn get_piece(&self, index: usize) -> Piece;
    fn unset_piece(&mut self, index: CellIndex, piece: Piece);
    fn remove_piece(&mut self, index: CellIndex);

    fn init(&mut self);
    fn to_string(&self) -> String;
    fn to_pretty_string(&self) -> String;
}

impl BoardTrait for Board {
    fn all(&self) -> Bitboard {
        self.white() | self.black()
    }
    fn white(&self) -> Bitboard {
        self[0] | self[1] | self[2] | self[3]
    }
    fn black(&self) -> Bitboard {
        self[4] | self[5] | self[6] | self[7]
    }

    fn colour(&self, player: Player) -> Bitboard {
        if player == 0 {
            self.white()
        } else {
            self.black()
        }
    }

    fn same_bottom(&self, player: Player) -> Bitboard {
        if player == 0 {
            self[8] | self[9] | self[10] | self[11]
        } else {
            self[12] | self[13] | self[14] | self[15]
        }
    }

    fn same_wise(&self, player: Player) -> Bitboard {
        if player == 0 {
            self[3]
        } else {
            self[7]
        }
    }

    fn victim(&self, piece: Piece) -> Bitboard {
        match piece & (COLOUR_MASK | TYPE_MASK) {
            0b000 => self[5],
            0b001 => self[6],
            0b010 => self[4],
            0b100 => self[1],
            0b101 => self[2],
            0b110 => self[0],
            _ => 0,
        }
    }

    fn set_piece(&mut self, index: CellIndex, piece: Piece) {
        self[(piece & 0b0111) as usize].set(index);
        if piece.is_stack() {
            self[(piece >> HALF_PIECE_WIDTH) as usize].set(index);
        }
    }

    fn unset_piece(&mut self, index: CellIndex, piece: Piece) {
        self[(piece & 0b0111) as usize].unset(index);
        if piece.is_stack() {
            self[(piece >> HALF_PIECE_WIDTH) as usize].unset(index);
        }
    }

    fn get_piece(&self, index: usize) -> Piece {
        let mut piece = 0u8;
        for k in 0..8 {
            if self[k].get(index) {
                piece = k as u8 | 0b1000;
                break;
            }
        }
        for k in 8..16 {
            if self[k].get(index) {
                piece |= (k << 4) as u8 | 0b1000_0000;
                break;
            }
        }
        piece
    }

    fn remove_piece(&mut self, index: CellIndex) {
        let piece = self.get_piece(index);
        self.unset_piece(index, piece);
    }

    fn init(&mut self) {
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

    fn to_string(&self) -> String {
        let mut cells_string = String::new();
        for i in 0..7usize {
            let n_columns: usize = if i % 2 == 0 { 6 } else { 7 };
            let mut counter: usize = 0;
            for j in 0..n_columns {
                let piece = self.get_piece(coords_to_index(i, j));
                if piece.is_empty() {
                    counter += 1;
                } else {
                    if counter > 0 {
                        cells_string += &counter.to_string();
                        counter = 0;
                    }
                    if piece.is_stack() {
                        cells_string += &piece_to_char(piece.bottom()).unwrap().to_string();
                        cells_string += &piece_to_char(piece.top()).unwrap().to_string();
                    } else {
                        cells_string += &piece_to_char(piece).unwrap().to_string();
                        cells_string += "-";
                    }
                }
            }
            if counter > 0 {
                cells_string += &counter.to_string();
            }
            if i < 6 {
                cells_string += "/";
            }
        }
        cells_string
    }

    fn to_pretty_string(&self) -> String {
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

pub fn play_action_bb(board: &mut Board, action: Action) {
    let (index_start, index_mid, index_end) = action.to_indices();

    if index_start.is_null() {
        return;
    }

    let piece_start: Piece = board.get_piece(index_start);

    if !piece_start.is_empty() {
        // If there is no intermediate move
        if index_mid.is_null() {
            // Simple move
            do_move_bb(board, index_start, index_end);
        } else {
            let piece_mid: Piece = board.get_piece(index_mid);
            let piece_end: Piece = board.get_piece(index_end);
            // The piece at the mid coordinates is an ally : stack and move
            if !piece_mid.is_empty()
                && piece_mid.colour() == piece_start.colour()
                && (index_start != index_mid)
            {
                do_stack_bb(board, index_start, index_mid);
                do_move_bb(board, index_mid, index_end);
            }
            // The piece at the end coordinates is an ally : move and stack
            else if !piece_end.is_empty() && piece_end.colour() == piece_start.colour() {
                do_move_bb(board, index_start, index_mid);
                do_stack_bb(board, index_mid, index_end);
            }
            // The end coordinates contain an enemy or no piece : move and unstack
            else {
                do_move_bb(board, index_start, index_mid);
                do_unstack_bb(board, index_mid, index_end);
            }
        }
    }
}

fn get_magic(index: CellIndex, blockers: Bitboard) -> Bitboard {
    let (magic, ref table) = MAGICS[index];
    let magic_hash = blockers.wrapping_mul(magic);
    let magic_index = (magic_hash >> (64 - 6)) as usize;
    table[magic_index]
}

fn available_moves2(board: &Board, index: CellIndex, piece: Piece) -> Bitboard {
    let blockers = BLOCKER_MASKS[index] & !board.all();
    get_magic(index, blockers) & (!board.all() | board.victim(piece))
}

fn available_moves1(board: &Board, index: CellIndex, piece: Piece) -> Bitboard {
    let neighbours = NEIGHBOURS[index];
    neighbours & (!board.all() | board.victim(piece))
}

fn available_stacks(board: &Board, index: CellIndex, piece: Piece) -> Bitboard {
    let neighbours = NEIGHBOURS[index];
    let player = piece.colour() >> 2;
    neighbours
        & (if piece.is_wise() {
            board.same_wise(player)
        } else {
            board.colour(player)
        } & !board.same_bottom(player))
}

fn available_unstacks(board: &Board, index: CellIndex, piece: Piece) -> Bitboard {
    let neighbours = NEIGHBOURS[index];
    neighbours & (!board.all() | board.victim(piece))
}

pub fn available_player_actions_bb(board: &Board, current_player: Player) -> Actions {
    let mut player_actions = Actions::default();

    // Calculate possible player_actions
    for index in 0..N_CELLS {
        // Choose pieces of the current player's colour
        if board.colour(current_player).get(index) {
            available_piece_actions_bb(board, index, &mut player_actions);
        }
    }
    player_actions
}

fn available_piece_actions_bb(board: &Board, index_start: CellIndex, player_actions: &mut Actions) {
    let piece_start = board.get_piece(index_start);

    if piece_start.is_stack() {
        // 2-range first action

        for index_mid in BitboardIterator(available_moves2(board, index_start, piece_start)) {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);

            player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));

            for index_end in BitboardIterator(
                available_unstacks(board, index_mid, piece_start)
                    | available_stacks(board, index_mid, piece_start),
            ) {
                player_actions.push(half_action.add_last_index(index_end));
            }
        }

        for index_mid in BitboardIterator(available_moves1(board, index_start, piece_start)) {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);

            for index_end in BitboardIterator(
                available_unstacks(board, index_mid, piece_start)
                    | available_stacks(board, index_mid, piece_start),
            ) {
                player_actions.push(half_action.add_last_index(index_end));
            }
            // 1-range move, unstack on starting position
            player_actions.push(Action::from_indices(index_start, index_mid, index_start));

            // 1-range move
            player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
        }

        for index_mid in BitboardIterator(available_stacks(board, index_start, piece_start)) {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);

            for index_end in BitboardIterator(
                available_moves2(board, index_mid, piece_start)
                    | available_moves1(board, index_mid, piece_start),
            ) {
                player_actions.push(half_action.add_last_index(index_end));
            }

            player_actions.push(Action::from_indices(index_start, index_start, index_mid));
        }

        for index_mid in BitboardIterator(available_unstacks(board, index_start, piece_start)) {
            player_actions.push(Action::from_indices(index_start, index_start, index_mid));
        }
    } else {
        for index_mid in BitboardIterator(available_stacks(board, index_start, piece_start)) {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);

            for index_end in BitboardIterator(
                available_moves2(board, index_mid, piece_start)
                    | available_moves1(board, index_mid, piece_start)
                    | (NEIGHBOURS2[index_mid] & available_moves1(board, index_start, piece_start)),
            ) {
                player_actions.push(half_action.add_last_index(index_end));
            }

            player_actions.push(half_action.add_last_index(index_start));

            player_actions.push(Action::from_indices(index_start, index_start, index_mid));
        }
        for index_mid in BitboardIterator(available_moves1(board, index_start, piece_start)) {
            player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
        }
    }
}

fn count_player_actions_bb(board: &Board, current_player: Player) -> u64 {
    let mut count: u64 = 0;

    // Calculate possible player_actions
    for index in 0..N_CELLS {
        // Choose pieces of the current player's colour
        if board.colour(current_player).get(index) {
            count += count_piece_actions_bb(board, index);
        }
    }
    count
}

fn count_piece_actions_bb(board: &Board, index_start: CellIndex) -> u64 {
    let mut count: u64 = 0;
    let piece_start = board.get_piece(index_start);

    if piece_start.is_stack() {
        // 2-range first action

        for index_mid in BitboardIterator(available_moves2(board, index_start, piece_start)) {
            count += (available_unstacks(board, index_mid, piece_start)
                | available_stacks(board, index_mid, piece_start))
            .count_ones() as u64;
            count += 1;
        }

        for index_mid in BitboardIterator(available_moves1(board, index_start, piece_start)) {
            count += (available_unstacks(board, index_mid, piece_start)
                | available_stacks(board, index_mid, piece_start))
            .count_ones() as u64;
            count += 1;
        }

        for index_mid in BitboardIterator(available_stacks(board, index_start, piece_start)) {
            count += (available_moves2(board, index_mid, piece_start)
                | available_moves1(board, index_mid, piece_start))
            .count_ones() as u64;

            count += 1;
        }

        count += available_unstacks(board, index_start, piece_start).count_ones() as u64;
    } else {
        for index_mid in BitboardIterator(available_stacks(board, index_start, piece_start)) {
            count += (available_moves2(board, index_mid, piece_start)
                | available_moves1(board, index_mid, piece_start)
                | (NEIGHBOURS2[index_mid] & available_moves1(board, index_start, piece_start)))
            .count_ones() as u64;

            count += 1;
            count += 1;
        }
        count += available_moves1(board, index_start, piece_start).count_ones() as u64;
    }
    count
}

fn is_action_win_bb(board: &Board, action: Action) -> bool {
    let (index_start, index_mid, index_end) = action.to_indices();

    let moving_piece: Piece = board.get_piece(index_start);

    !moving_piece.is_wise()
        && (index_mid != INDEX_NULL
            && ((moving_piece.is_white() && index_mid.is_black_home())
                || (moving_piece.is_black() && index_mid.is_white_home()))
            || (moving_piece.is_white() && index_end.is_black_home())
            || (moving_piece.is_black() && index_end.is_white_home()))
}

pub fn perft_iter_bb(board: &Board, current_player: Player, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 => count_player_actions_bb(board, current_player),
        _ => {
            let available_actions = available_player_actions_bb(board, current_player);

            available_actions
                .into_iter()
                .filter(|&action| !is_action_win_bb(board, action))
                .map(|action| {
                    let mut new_board = *board;
                    play_action_bb(&mut new_board, action);
                    perft_iter_bb(&new_board, 1 - current_player, depth - 1)
                })
                .sum()
        }
    }
}

pub fn perft_bb(board: &Board, current_player: Player, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 => count_player_actions_bb(board, current_player),
        _ => {
            let available_actions = available_player_actions_bb(board, current_player);

            available_actions
                .into_iter()
                // .par_bridge()
                .filter(|&action| !is_action_win_bb(board, action))
                .map(|action| {
                    let mut new_board = *board;
                    play_action_bb(&mut new_board, action);
                    perft_iter_bb(&new_board, 1 - current_player, depth - 1)
                })
                .sum()
        }
    }
}
