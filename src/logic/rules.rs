//! Implements the rules to check if an action is valid or not.
use crate::{
    bitboard::Board,
    piece::{Piece, PieceTrait},
};

use super::{
    actions::{Action, ActionTrait, ACTION_MASK},
    index::{CellIndexTrait, INDEX_NULL},
    Player,
};

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
