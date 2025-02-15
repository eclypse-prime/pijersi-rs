//! Implements the rules to check if an action is valid or not.
use crate::piece::{Piece, PieceTrait};

use super::{
    actions::{Action, ActionTrait, ACTION_MASK},
    index::{CellIndex, CellIndexTrait},
    movegen::available_player_actions,
    perft::count_player_actions,
    Cells, Player,
};

/// Returns whether an attacker piece can capture a target piece.
///
/// The capture rules are the sames as rock-paper-scissors.
/// The wise piece can neither capture or be captured.
#[inline]
pub fn can_take(attacker: Piece, target: Piece) -> bool {
    let attacker_type: Piece = attacker.r#type();
    let target_type: Piece = target.r#type();
    // Concat has 16 possible values, we can use a truth table to quickly get the result
    let concat = attacker_type | (target_type >> 2);
    // This will optimize to a single `bt` operartion in asm
    (0b0000_0001_0100_0010 >> concat) & 1 == 1
}

/// Returns whether the chosen 1-range move is possible.
#[inline]
pub fn can_move1(cells: &Cells, moving_piece: Piece, index_end: CellIndex) -> bool {
    let target_piece: Piece = cells[index_end];

    if !target_piece.is_empty() {
        // If the end piece and the moving piece are the same colour
        if target_piece.colour() == moving_piece.colour() {
            return false;
        }
        if !can_take(moving_piece, target_piece) {
            return false;
        }
    }
    true
}

/// Returns whether the chosen 2-range move is possible.
#[inline]
pub fn can_move2(
    cells: &Cells,
    moving_piece: Piece,
    index_start: CellIndex,
    index_end: CellIndex,
) -> bool {
    let target_piece: Piece = cells[index_end];

    // If there is a piece blocking the move (cell between the start and end positions)
    if !cells[(index_end + index_start) / 2].is_empty() {
        return false;
    }
    if !target_piece.is_empty() {
        // If the end piece and the moving piece are the same colour
        if target_piece.colour() == moving_piece.colour() {
            return false;
        }
        if !can_take(moving_piece, target_piece) {
            return false;
        }
    }
    true
}

/// Returns whether the chosen stack action is possible.
#[inline]
pub fn can_stack(cells: &Cells, moving_piece: Piece, index_end: CellIndex) -> bool {
    let target_piece: Piece = cells[index_end];

    // If the end cell is not empty
    // If the target piece and the moving piece are the same colour
    // If the end piece is not a stack
    if !target_piece.is_empty()
        && target_piece.colour() == moving_piece.colour()
        && !target_piece.is_stack()
    {
        // If the upper piece is Wise and the target piece is not Wise
        if moving_piece.is_wise() && !target_piece.is_wise() {
            return false;
        }
        return true;
    }

    false
}

/// Returns whether the chosen unstack action is possible.
#[inline]
pub fn can_unstack(cells: &Cells, moving_piece: Piece, index_end: CellIndex) -> bool {
    let target_piece: Piece = cells[index_end];

    if !target_piece.is_empty() {
        // If the cells are the same colour
        if target_piece.colour() == moving_piece.colour() {
            return false;
        }
        if !can_take(moving_piece, target_piece) {
            return false;
        }
    }
    true
}

/// Returns true if the chosen action leads to a win.
///
/// To win, one allied piece (except wise) must reach the last row in the opposite side.
#[inline]
pub fn is_action_win(cells: &Cells, action: Action) -> bool {
    let (index_start, index_mid, index_end) = action.to_indices();

    let moving_piece: Piece = cells[index_start];

    !moving_piece.is_wise()
        && (index_mid != INDEX_NULL && ((moving_piece.is_white() && index_mid.is_black_home())
            || (moving_piece.is_black() && index_mid.is_white_home()))
            || (moving_piece.is_white() && index_end.is_black_home())
            || (moving_piece.is_black() && index_end.is_white_home()))
}

/// Returns true if the given action is legal.
pub fn is_action_legal(cells: &Cells, current_player: Player, action: Action) -> bool {
    let action = action & ACTION_MASK;
    let available_actions = available_player_actions(cells, current_player);
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

/// Returns true if the current position is a stalemate for one of the players.
///
/// This means one of the two players has no legal move left.
pub fn is_position_stalemate(cells: &Cells, current_player: Player) -> bool {
    count_player_actions(cells, current_player, 1) == 0
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
