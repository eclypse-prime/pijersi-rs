//! This module implements the evaluation functions: evaluates the score of a current position or evaluates the best score at a given depth.

use crate::logic::actions::{Action, ActionTrait};
use crate::logic::index::CellIndex;
use crate::logic::lookup::PIECE_TO_INDEX;
use crate::logic::rules::is_action_win;
use crate::logic::{Cells, Player, N_CELLS};
use crate::piece::{Piece, PieceTrait};
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
pub fn evaluate_position(cells: &Cells) -> Score {
    #[cfg(feature = "nps-count")]
    increment_node_count(1);
    cells
        .iter()
        .enumerate()
        .map(|(index, &piece)| evaluate_cell(piece, index))
        .sum()
}

/// Returns the score of a board along with its individual cell scores.
pub fn evaluate_position_with_details(cells: &Cells) -> (Score, [Score; N_CELLS]) {
    let mut piece_scores: [Score; N_CELLS] = [0; N_CELLS];
    for (k, &cell) in cells.iter().enumerate() {
        piece_scores[k] = evaluate_cell(cell, k);
    }
    (piece_scores.iter().sum(), piece_scores)
}

#[inline]
/// Evaluates the score of a given action at depth 1.
///
/// Efficient method that only calculates the scores of the cells that would change and compares it to the current score.
pub fn evaluate_action_terminal(
    cells: &Cells,
    current_player: Player,
    action: Action,
    previous_score: Score,
    previous_piece_scores: &[Score; N_CELLS],
) -> Score {
    let (index_start, index_mid, index_end) = action.to_indices();

    if is_action_win(cells, action) {
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
        let mut start_piece: Piece = cells[index_start];
        let mut mid_piece: Piece = cells[index_mid];
        let mut end_piece: Piece = cells[index_end];
        // The piece at the mid coordinates is an ally : stack and action
        if !mid_piece.is_empty()
            && mid_piece.colour() == start_piece.colour()
            && (index_mid != index_start)
        {
            end_piece = start_piece.stack_on(mid_piece);
            start_piece = start_piece.bottom();
            mid_piece.set_empty();

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
        else if !end_piece.is_empty() && end_piece.colour() == start_piece.colour() {
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
