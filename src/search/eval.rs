//! This module implements the evaluation functions: evaluates the score of a current position or evaluates the best score at a given depth.

use std::cmp::max;
use std::time::Instant;

use crate::logic::actions::play_action;
use crate::logic::lookup::PIECE_TO_INDEX;
use crate::logic::movegen::available_player_actions;
use crate::logic::{INDEX_MASK, INDEX_WIDTH};
use crate::piece::{Piece, CELL_EMPTY};
use crate::search::lookup::PIECE_SCORES;

/// The max score (is reached on winning position)
pub const MAX_SCORE: i64 = 524_288;

#[inline]
/// Returns the score of a single cell given its content and index.
///
/// Uses lookup tables for faster computations.
pub fn evaluate_cell(piece: u8, index: usize) -> i64 {
    PIECE_SCORES[PIECE_TO_INDEX[piece as usize] * 45 + index]
}

/// Returns the score of a board.
pub fn evaluate_position(cells: &[u8; 45]) -> i64 {
    cells
        .iter()
        .enumerate()
        .map(|(index, &piece)| evaluate_cell(piece, index))
        .sum()
}

/// Returns the score of a board along with its individual cell scores.
pub fn evaluate_position_with_details(cells: &[u8; 45]) -> (i64, [i64; 45]) {
    let mut piece_scores: [i64; 45] = [0i64; 45];
    for (k, &cell) in cells.iter().enumerate() {
        piece_scores[k] = evaluate_cell(cell, k);
    }
    (piece_scores.iter().sum(), piece_scores)
}

/// Evaluates the score of a given action by searching at a given depth.
///
/// Recursively calculates the best score using the alphabeta search to the chosen depth.
pub fn evaluate_action(
    cells: &[u8; 45],
    current_player: u8,
    action: u64,
    depth: u64,
    alpha: i64,
    beta: i64,
    end_time: Option<Instant>,
) -> i64 {
    let index_start: usize = (action & INDEX_MASK) as usize;
    let index_end: usize = ((action >> (2 * INDEX_WIDTH)) & INDEX_MASK) as usize;

    if !cells[index_start].is_wise()
        && ((current_player == 1 && (index_end <= 5)) || (current_player == 0 && (index_end >= 39)))
    {
        return -MAX_SCORE;
    }

    let mut alpha = alpha;

    let mut new_cells: [u8; 45] = *cells;
    play_action(&mut new_cells, action);

    if depth == 0 {
        return if current_player == 0 {
            evaluate_position(&new_cells)
        } else {
            -evaluate_position(&new_cells)
        };
    }

    if let Some(end_time) = end_time {
        if Instant::now() > end_time {
            return i64::MIN;
        }
    }

    let (available_actions, n_actions) = available_player_actions(&new_cells, current_player);

    let mut score = i64::MIN;

    if n_actions == 0 {
        return score;
    }

    if depth == 1 {
        let (previous_score, previous_piece_scores) = evaluate_position_with_details(&new_cells);
        for &action in available_actions.iter().take(n_actions) {
            score = max(
                score,
                -evaluate_action_terminal(
                    &new_cells,
                    1 - current_player,
                    action,
                    previous_score,
                    &previous_piece_scores,
                ),
            );
            alpha = max(alpha, score);
            if alpha > beta {
                break;
            }
        }
    } else {
        for (k, &action) in available_actions.iter().take(n_actions).enumerate() {
            let eval = if k == 0 {
                -evaluate_action(
                    &new_cells,
                    1 - current_player,
                    action,
                    depth - 1,
                    -beta,
                    -alpha,
                    end_time,
                )
            } else {
                let eval_null_window = -evaluate_action(
                    &new_cells,
                    1 - current_player,
                    action,
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                    end_time,
                );
                if alpha < eval_null_window && eval_null_window < beta {
                    -evaluate_action(
                        &new_cells,
                        1 - current_player,
                        action,
                        depth - 1,
                        -beta,
                        -alpha,
                        end_time,
                    )
                } else {
                    eval_null_window
                }
            };
            score = max(score, eval);
            alpha = max(alpha, score);
            if alpha > beta {
                break;
            }
        }
    }
    score
}

#[inline]
/// Evaluates the score of a given action at depth 1.
///
/// Efficient method that only calculates the scores of the cells that would change and compares it to the current score.
pub fn evaluate_action_terminal(
    cells: &[u8; 45],
    current_player: u8,
    action: u64,
    previous_score: i64,
    previous_piece_scores: &[i64; 45],
) -> i64 {
    let index_start: usize = (action & INDEX_MASK) as usize;
    let index_mid: usize = ((action >> INDEX_WIDTH) & INDEX_MASK) as usize;
    let index_end: usize = ((action >> (2 * INDEX_WIDTH)) & INDEX_MASK) as usize;

    if !cells[index_start].is_wise()
        && ((current_player == 1 && (index_end <= 5)) || (current_player == 0 && (index_end >= 39)))
    {
        return -MAX_SCORE;
    }

    let mut current_score = previous_score;

    if index_mid > 44 {
        // Starting cell
        current_score -= previous_piece_scores[index_start];

        // Ending cell
        current_score -= previous_piece_scores[index_end];
        current_score += evaluate_cell(cells[index_start], index_end);
    } else {
        let mut start_piece: u8 = cells[index_start];
        let mut mid_piece: u8 = cells[index_mid];
        let mut end_piece: u8 = cells[index_end];
        // The piece at the mid coordinates is an ally : stack and action
        if !mid_piece.is_empty()
            && mid_piece.colour() == start_piece.colour()
            && (index_mid != index_start)
        {
            end_piece = start_piece.stack_on(mid_piece);
            start_piece = start_piece.bottom();
            mid_piece = CELL_EMPTY;

            // Starting cell
            current_score -= previous_piece_scores[index_start];
            current_score += evaluate_cell(start_piece, index_start);

            // Middle cell
            current_score -= previous_piece_scores[index_mid];
            current_score += evaluate_cell(mid_piece, index_mid);

            // Ending cell
            if index_start != index_end {
                current_score -= previous_piece_scores[index_end];
            }
            current_score += evaluate_cell(end_piece, index_end);
        }
        // The piece at the end coordinates is an ally : action and stack
        else if !end_piece.is_empty()
            && end_piece.colour() == start_piece.colour()
        {
            mid_piece = start_piece;
            end_piece = mid_piece.stack_on(end_piece);
            if index_start == index_end {
                end_piece = mid_piece.top();
            }
            mid_piece = mid_piece.bottom();

            // Starting cell
            if index_start != index_mid {
                current_score -= previous_piece_scores[index_start];
            }

            // Middle cell
            current_score -= previous_piece_scores[index_mid];
            current_score += evaluate_cell(mid_piece, index_mid);

            // Ending cell
            if index_start != index_end {
                current_score -= previous_piece_scores[index_end];
            }
            current_score += evaluate_cell(end_piece, index_end);
        }
        // The end coordinates contain an enemy or no piece : action and unstack
        else {
            mid_piece = start_piece;
            end_piece = mid_piece.top();
            mid_piece = mid_piece.bottom();

            // Starting cell
            if index_start != index_mid {
                current_score -= previous_piece_scores[index_start];
            }

            // Middle cell
            current_score -= previous_piece_scores[index_mid];
            current_score += evaluate_cell(mid_piece, index_mid);

            // Ending cell
            current_score -= previous_piece_scores[index_end];
            current_score += evaluate_cell(end_piece, index_end);
        }
    }

    if current_player == 0 {
        current_score
    } else {
        -current_score
    }
}
