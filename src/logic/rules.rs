//! Implements the rules to check if an action is valid or not.
use crate::piece::{Piece, COLOUR_BLACK, COLOUR_WHITE, TYPE_PAPER, TYPE_ROCK, TYPE_SCISSORS};

use super::{
    movegen::available_player_actions, perft::perft_iter, ACTION_MASK, INDEX_MASK, INDEX_WIDTH,
};

/// Returns whether an attacker piece can capture a target piece.
///
/// The capture rules are the sames as rock-paper-scissors.
/// The wise piece can neither capture or be captured.
#[inline]
pub fn can_take(attacker: u8, target: u8) -> bool {
    let attacker_type: u8 = attacker.r#type();
    let target_type: u8 = target.r#type();
    (attacker_type == TYPE_SCISSORS && target_type == TYPE_PAPER)
        || (attacker_type == TYPE_PAPER && target_type == TYPE_ROCK)
        || (attacker_type == TYPE_ROCK && target_type == TYPE_SCISSORS)
}

/// Returns whether the chosen 1-range move is possible.
#[inline]
pub fn can_move1(cells: &[u8; 45], moving_piece: u8, index_end: usize) -> bool {
    let target_piece: u8 = cells[index_end];

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
pub fn can_move2(cells: &[u8; 45], moving_piece: u8, index_start: usize, index_end: usize) -> bool {
    let target_piece: u8 = cells[index_end];

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
pub fn can_stack(cells: &[u8; 45], moving_piece: u8, index_end: usize) -> bool {
    let target_piece: u8 = cells[index_end];

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
pub fn can_unstack(cells: &[u8; 45], moving_piece: u8, index_end: usize) -> bool {
    let target_piece: u8 = cells[index_end];

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
pub fn is_action_win(cells: &[u8; 45], action: u64) -> bool {
    let index_start: usize = (action & INDEX_MASK) as usize;
    let index_end: usize = ((action >> (2 * INDEX_WIDTH)) & INDEX_MASK) as usize;

    let moving_piece: u8 = cells[index_start];

    if !moving_piece.is_wise()
        && ((moving_piece.colour() == COLOUR_WHITE && index_end <= 5)
            || (moving_piece.colour() == COLOUR_BLACK && index_end >= 39))
    {
        return true;
    }
    false
}

/// Returns true if the given action is legal.
pub fn is_action_legal(cells: &[u8; 45], current_player: u8, action: u64) -> bool {
    let action = action & ACTION_MASK;
    let (available_actions, n_actions) = available_player_actions(cells, current_player);
    available_actions
        .iter()
        .take(n_actions)
        .any(|&available_action| available_action == action)
}

/// Returns true if the current position is winning for one of the players.
pub fn is_position_win(cells: &[u8; 45]) -> bool {
    for &piece in cells.iter().take(6) {
        if !piece.is_empty() {
            // If piece is White and not Wise
            if piece.colour() == COLOUR_WHITE && !piece.is_wise() {
                return true;
            }
        }
    }
    for &piece in cells.iter().skip(39).take(6) {
        if !piece.is_empty() {
            // If piece is Black and not Wise
            if piece.colour() == COLOUR_BLACK && !piece.is_wise() {
                return true;
            }
        }
    }
    false
}

/// Returns true if the current position is a stalemate for one of the players.
///
/// This means one of the two players has no legal move left.
pub fn is_position_stalemate(cells: &[u8; 45], current_player: u8) -> bool {
    perft_iter(cells, current_player, 1) == 0
}

/// Returns the winning player if there is one.
pub fn get_winning_player(cells: &[u8; 45]) -> Option<u8> {
    for &piece in cells.iter().take(6) {
        if !piece.is_empty() {
            // If piece is White and not Wise
            if piece.colour() == COLOUR_WHITE && !piece.is_wise() {
                return Some(0);
            }
        }
    }
    for &piece in cells.iter().skip(39).take(6) {
        if !piece.is_empty() {
            // If piece is Black and not Wise
            if piece.colour() == COLOUR_BLACK && !piece.is_wise() {
                return Some(1);
            }
        }
    }
    None
}
