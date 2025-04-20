//! Implements the rules to check if an action is valid or not.
use crate::{
    bitboard::Board,
    piece::{Piece, PieceTrait},
};

use super::{
    actions::{Action, ActionTrait, ACTION_MASK},
    index::{CellIndexTrait, INDEX_NULL},
    perft::perft_player_actions,
    Cells, Player,
};

/// Returns whether an attacker piece can capture a target piece.
///
/// The capture rules are the sames as rock-paper-scissors.
/// The wise piece can neither capture or be captured.

#[inline]
/// Returns true if the chosen actrion is a capture
pub fn is_action_capture(cells: &Cells, action: Action) -> bool {
    let (index_start, index_mid, index_end) = action.to_indices();

    let moving_piece: Piece = cells[index_start];
    let piece_colour = moving_piece.colour();

    (!index_mid.is_null()
        && !cells[index_mid].is_empty()
        && cells[index_mid].colour() != piece_colour)
        || (!cells[index_end].is_empty() && cells[index_end].colour() != piece_colour)
}

/// Returns true if the chosen action leads to a win.
///
/// To win, one allied piece (except wise) must reach the last row in the opposite side.
#[inline]
pub fn is_action_win(board: &Board, action: Action) -> bool {
    let (index_start, index_mid, index_end) = action.to_indices();

    let moving_piece: Piece = board.get_piece(index_start);

    !moving_piece.is_wise()
        && (index_mid != INDEX_NULL
            && ((moving_piece.is_white() && index_mid.is_black_home())
                || (moving_piece.is_black() && index_mid.is_white_home()))
            || (moving_piece.is_white() && index_end.is_black_home())
            || (moving_piece.is_black() && index_end.is_white_home()))
}

/// Returns true if the given action is legal.
pub fn is_action_legal(board: &Board, current_player: Player, action: Action) -> bool {
    let action = action & ACTION_MASK;
    let available_actions = board.available_player_actions(current_player);
    available_actions
        .into_iter()
        .any(|available_action| available_action == action)
}

/// Returns true if the current position is winning for one of the players.
pub fn is_position_win(cells: &Cells) -> bool {
    for &piece in cells[0..6].iter() {
        if !piece.is_empty() && piece.is_white() && !piece.is_wise() {
            return true;
        }
    }
    for &piece in cells[39..45].iter() {
        if !piece.is_empty() && piece.is_black() && !piece.is_wise() {
            return true;
        }
    }
    false
}

/// Returns the winning player if there is one.
pub fn get_winning_player(cells: &Cells) -> Option<Piece> {
    for &piece in cells[0..6].iter() {
        if !piece.is_empty() && piece.is_white() && !piece.is_wise() {
            return Some(0);
        }
    }
    for &piece in cells[39..45].iter() {
        if !piece.is_empty() && piece.is_black() && !piece.is_wise() {
            return Some(1);
        }
    }
    None
}
