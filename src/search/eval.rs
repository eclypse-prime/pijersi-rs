//! This module implements the evaluation functions: evaluates the score of a current position or evaluates the best score at a given depth.

use std::cmp::max;

use crate::bitboard::Board;
use crate::logic::actions::{Action, ActionTrait, ActionsLight};
use crate::logic::index::{CellIndex, CellIndexTrait};
use crate::logic::lookup::PIECE_TO_INDEX;
use crate::logic::{Player, N_CELLS};
use crate::piece::Piece;
use crate::search::lookup::PIECE_SCORES;

#[cfg(feature = "nps-count")]
use super::alphabeta::increment_node_count;
use super::Score;

/// The max score (is reached on winning position)
pub const MAX_SCORE: Score = 16_384;

/// Returns the score of a single cell given its content and index.
///
/// Uses lookup tables for faster computations.
#[inline]
pub const fn evaluate_cell(piece: Piece, index: CellIndex) -> Score {
    PIECE_SCORES[PIECE_TO_INDEX[piece as usize] * N_CELLS + index]
}

/// Returns the score of a board.
///
/// If the score is positive, the position favours the white player.
/// If the score is negative, the position favours the black player.
pub fn evaluate_position(board: &Board) -> Score {
    #[cfg(feature = "nps-count")]
    increment_node_count(1);
    board
        .all()
        .into_iter()
        .map(|index| evaluate_cell(board.get_piece(index), index))
        .sum()
}

/// Returns the score of a board from the point of view of the chosen player.
///
/// The higher the score, the better the position.
pub fn evaluate_position_for_player(board: &Board, current_player: Player) -> Score {
    #[cfg(feature = "nps-count")]
    increment_node_count(1);
    let eval = board
        .all()
        .into_iter()
        .map(|index| evaluate_cell(board.get_piece(index), index))
        .sum();
    if current_player == 0 {
        eval
    } else {
        -eval
    }
}

#[inline]
/// Evaluates the score of a position after an action.
///
/// Efficient method that only calculates the cells of the board that changed and compares it to the current score.
pub fn evaluate_position_incremental(
    old_board: &Board,
    new_board: &Board,
    action: Action,
    previous_score: Score,
) -> Score {
    let (index_start, index_mid, index_end) = action.to_indices();

    let mut score = previous_score;

    if index_mid > 44 {
        let old_start_piece = old_board.get_piece(index_start);
        let old_end_piece = old_board.get_piece(index_end);
        let new_end_piece = new_board.get_piece(index_end);

        score -= evaluate_cell(old_start_piece, index_start);

        score -= evaluate_cell(old_end_piece, index_end);
        score += evaluate_cell(new_end_piece, index_end);
    } else {
        let old_start_piece = old_board.get_piece(index_start);
        let new_start_piece = new_board.get_piece(index_start);
        score -= evaluate_cell(old_start_piece, index_start);
        score += evaluate_cell(new_start_piece, index_start);
        if index_mid != index_start {
            let old_mid_piece = old_board.get_piece(index_mid);
            let new_mid_piece = new_board.get_piece(index_mid);
            score -= evaluate_cell(old_mid_piece, index_mid);
            score += evaluate_cell(new_mid_piece, index_mid);
        }
        if index_end != index_start {
            let old_end_piece = old_board.get_piece(index_end);
            let new_end_piece = new_board.get_piece(index_end);
            score -= evaluate_cell(old_end_piece, index_end);
            score += evaluate_cell(new_end_piece, index_end);
        }
    }

    score
}

fn sort_captures(
    board: &Board,
    current_player: Player,
    available_captures: &mut ActionsLight,
) -> Option<Action> {
    let mut index_sorted = 0;
    let n_actions = available_captures.len();
    for i in 0..n_actions {
        let action = available_captures[i];
        if board.is_action_win(action, current_player) {
            return Some(action);
        }
        let (_index_start, index_mid, index_end) = action.to_indices();
        if (!index_mid.is_null() && board.opposite_stacks(current_player).get(index_mid))
            || (board.opposite_stacks(current_player).get(index_end))
        {
            available_captures[i] = available_captures[index_sorted];
            available_captures[index_sorted] = action;
            index_sorted += 1;
        }
    }
    let index_start = index_sorted;
    for i in index_start..n_actions {
        let action = available_captures[i];
        if board.is_action_win(action, current_player) {
            return Some(action);
        }
        let (_index_start, index_mid, index_end) = action.to_indices();
        if (!index_mid.is_null() && board.capturable(current_player).get(index_mid))
            && (board.capturable(current_player).get(index_end))
        {
            available_captures[i] = available_captures[index_sorted];
            available_captures[index_sorted] = action;
            index_sorted += 1;
        }
    }

    None
}

/// Evaluates a position using quiescence search.
///
/// Resolves all capture chains before evaluating positions and returns the best score using alphabeta.
pub fn quiescence_search(
    board: &Board,
    current_player: Player,
    (alpha, beta): (Score, Score),
    static_eval: Score,
) -> Score {
    let mut available_captures = board.available_player_captures_and_wins(current_player);
    let n_actions = available_captures.len();

    // Heuristic to return early
    let stand_pat = if current_player == 0 {
        static_eval
    } else {
        -static_eval
    };

    if n_actions == 0 || stand_pat > beta {
        return stand_pat;
    }

    let winning_action = sort_captures(board, current_player, &mut available_captures);

    if winning_action.is_some() {
        return MAX_SCORE;
    }

    let mut score = stand_pat;

    let mut alpha = max(alpha, stand_pat);

    let mut new_board;
    for action in available_captures.into_iter() {
        if board.is_action_win(action, current_player) {
            return MAX_SCORE;
        }
        new_board = *board;
        new_board.play_action(action);
        let new_static_eval = evaluate_position_incremental(board, &new_board, action, static_eval);
        let eval = max(
            score,
            -quiescence_search(
                &new_board,
                1 - current_player,
                (-beta, -alpha),
                new_static_eval,
            ),
        );
        score = max(score, eval);
        alpha = max(alpha, eval);

        // Beta-cutoff, stop the search
        if alpha > beta {
            break;
        }
    }
    score
}
