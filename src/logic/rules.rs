//! Implements the rules to check if an action is valid or not.
use crate::{
    bitboard::{Bitboard, Board},
    piece::{Piece, PieceTrait, COLOUR_MASK, TYPE_MASK},
};

use super::{
    actions::{Action, ActionTrait, ACTION_MASK},
    index::{CellIndex, CellIndexTrait, INDEX_NULL},
    lookup::{BLOCKER_MASKS, MAGICS, NEIGHBOURS1},
    Player,
};

const WHITE_WIN_MASK: Bitboard = Bitboard(0b000000000000000000000000000000000000000111111);
const BLACK_WIN_MASK: Bitboard = Bitboard(0b111111000000000000000000000000000000000000000);

/// Returns true if the given action is legal.
pub fn is_action_legal(board: &Board, current_player: Player, action: Action) -> bool {
    let action = action & ACTION_MASK;
    let available_actions = board.available_player_actions(current_player);
    available_actions
        .into_iter()
        .any(|available_action| available_action == action)
}

fn win_mask(player: Player) -> Bitboard {
    if player == 0 {
        WHITE_WIN_MASK
    } else {
        BLACK_WIN_MASK
    }
}

impl Bitboard {
    /// When used on a bitboard of blockers, this function returns a bitboard of available 2-range moves.
    pub fn get_magic(&self, index: CellIndex) -> Bitboard {
        let (magic, ref table) = MAGICS[index];
        let magic_hash = self.0.wrapping_mul(magic.0);
        let magic_index = (magic_hash >> (64 - 6)) as usize;
        table[magic_index]
    }
}

impl Board {
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

    /// Returns a bitboard representing the pieces that are capturable by the given player.
    pub fn capturable(&self, player: Player) -> Bitboard {
        if player == 0 {
            self.black_not_wise()
        } else {
            self.white_not_wise()
        }
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
    pub fn available_captures_and_win1(
        &self,
        index: CellIndex,
        piece: Piece,
        player: Player,
    ) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        neighbours & (self.victims(piece) | win_mask(player))
    }

    /// Returns a bitboard with the available range-2 captures for the piece at the given index.
    pub fn available_captures_and_win2(
        &self,
        index: CellIndex,
        piece: Piece,
        player: Player,
    ) -> Bitboard {
        let blockers = BLOCKER_MASKS[index] & !self.all();
        blockers.get_magic(index) & (self.victims(piece) | win_mask(player))
    }

    /// Returns a bitboard with the available range-1 non-capture actions for the piece at the given index.
    pub fn available_non_captures1(&self, index: CellIndex) -> Bitboard {
        let neighbours = NEIGHBOURS1[index];
        neighbours & !self.all()
    }

    /// Returns a bitboard with the available range-2 non-capture moves for the piece at the given index.
    pub fn available_non_captures2(&self, index: CellIndex) -> Bitboard {
        let blockers = BLOCKER_MASKS[index] & !self.all();
        blockers.get_magic(index) & !self.all()
    }

    /// Returns true if the current position is winning for one of the players.
    pub fn is_win(&self) -> bool {
        (self.white_not_wise() & WHITE_WIN_MASK).0 != 0
            || (self.black_not_wise() & BLACK_WIN_MASK).0 != 0
    }

    /// Returns true if the current position is a stalemate for one of the players.
    ///
    /// This means one of the two players has no legal move left.
    pub fn is_stalemate(&self, current_player: Player) -> bool {
        self.count_player_actions(current_player) == 0
    }

    /// Returns the winning player if there is one.
    pub fn get_winner(&self) -> Option<Player> {
        if (self.white_not_wise() & WHITE_WIN_MASK).0 != 0 {
            Some(0)
        } else if (self.black_not_wise() & BLACK_WIN_MASK).0 != 0 {
            Some(1)
        } else {
            None
        }
    }

    /// Returns true if the chosen action leads to a win.
    ///
    /// To win, one allied piece (except wise) must reach the last row in the opposite side.
    pub fn is_action_win(&self, action: Action, player: Player) -> bool {
        let (index_start, index_mid, index_end) = action.to_indices();

        !self.same_wise(player).get(index_start)
            && (index_mid != INDEX_NULL
                && ((player == 0 && index_mid.is_black_home())
                    || (player == 1 && index_mid.is_white_home()))
                || (player == 0 && index_end.is_black_home())
                || (player == 1 && index_end.is_white_home()))
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
}
