//! This module implements the evaluation functions: evaluates the score of a current position or evaluates the best score at a given depth.

use std::cmp::max;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::sync::Mutex;
use std::time::Instant;

use rayon::prelude::*;

use crate::hash::position::HashTrait;
use crate::hash::search::SearchTable;
use crate::logic::actions::{play_action, Action, ActionTrait, Actions};
use crate::logic::index::{CellIndex, CellIndexTrait};
use crate::logic::lookup::PIECE_TO_INDEX;
use crate::logic::movegen::available_player_actions;
use crate::logic::{Cells, Player, N_CELLS};
use crate::piece::{Piece, PieceTrait};
use crate::search::lookup::PIECE_SCORES;

#[cfg(feature = "nps-count")]
use super::alphabeta::increment_node_count;

/// The max score (is reached on winning position)
pub const MAX_SCORE: i32 = 524_288;

/// Returns the score of a single cell given its content and index.
///
/// Uses lookup tables for faster computations.
#[inline]
pub const fn evaluate_cell(piece: Piece, index: CellIndex) -> i32 {
    PIECE_SCORES[PIECE_TO_INDEX[piece as usize] * N_CELLS + index]
}

/// Returns the score of a board.
pub fn evaluate_position(cells: &Cells) -> i32 {
    #[cfg(feature = "nps-count")]
    unsafe {
        increment_node_count(1);
    }
    cells
        .iter()
        .enumerate()
        .map(|(index, &piece)| evaluate_cell(piece, index))
        .sum()
}

/// Returns the score of a board along with its individual cell scores.
pub fn evaluate_position_with_details(cells: &Cells) -> (i32, [i32; N_CELLS]) {
    let mut piece_scores: [i32; N_CELLS] = [0i32; N_CELLS];
    for (k, &cell) in cells.iter().enumerate() {
        piece_scores[k] = evaluate_cell(cell, k);
    }
    (piece_scores.iter().sum(), piece_scores)
}

/// Sorts the available actions based on how good they are estimated to be (in descending order -> best actions first).
#[inline]
pub fn sort_actions(
    cells: &Cells,
    current_player: Player,
    available_actions: &mut Actions,
) -> bool {
    let n_actions = available_actions.len();
    let mut index_sorted = 0;
    for i in 0..n_actions {
        let action = available_actions[i];
        let (index_start, index_mid, index_end) = action.to_indices();
        if !cells[index_start].is_wise()
            && ((current_player == 0 && index_end.is_black_home())
                || (current_player == 1 && index_end.is_white_home())
                || ((current_player == 0 && !index_mid.is_null() && index_mid.is_black_home())
                    || (current_player == 1 && !index_mid.is_null() && index_mid.is_white_home())))
        {
            return true;
        }
        if (!index_mid.is_null()
            && !cells[index_mid].is_empty()
            && cells[index_mid].colour() != current_player << 1)
            || (!cells[index_end].is_empty() && cells[index_end].colour() != current_player << 1)
        {
            available_actions[i] = available_actions[index_sorted];
            available_actions[index_sorted] = action;
            index_sorted += 1;
        }
    }
    false
}

/// Evaluates the score of a given action by searching at a given depth.
///
/// Recursively calculates the best score using the alphabeta search to the chosen depth.
pub fn evaluate_action(
    cells: &Cells,
    current_player: Player,
    action: Action,
    depth: u64,
    alpha: i32,
    beta: i32,
    end_time: Option<Instant>,
    transposition_table: Option<&Mutex<SearchTable>>,
) -> i32 {
    let mut alpha = alpha;

    let mut new_cells: Cells = *cells;
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
            return i32::MIN;
        }
    }

    let mut available_actions = available_player_actions(&new_cells, current_player);
    let n_actions = available_actions.len();

    if n_actions == 0 {
        return i32::MIN;
    }

    let mut score = i32::MIN;

    if depth == 1 {
        #[cfg(feature = "nps-count")]
        let mut node_count: u64 = 1;
        let (previous_score, previous_piece_scores) = evaluate_position_with_details(&new_cells);
        for action in available_actions.into_iter() {
            #[cfg(feature = "nps-count")]
            {
                node_count += 1;
            }
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
        #[cfg(feature = "nps-count")]
        unsafe {
            increment_node_count(node_count);
        }
    } else {
        if depth >= 3 {
            if let Some(transposition_table) = transposition_table {
                let new_cells_hash = new_cells.hash();
                let mut transposition_table = transposition_table.lock().unwrap();
                let (table_score, table_depth, table_player) =
                    transposition_table.read(new_cells_hash).unwrap();
                if table_depth == depth && table_player == current_player {
                    return table_score as i32;
                }
            }
        }
        let is_action_win = sort_actions(&new_cells, current_player, &mut available_actions);
        if is_action_win {
            if depth >= 3 {
                if let Some(transposition_table) = transposition_table {
                    let new_cells_hash = new_cells.hash();
                    let mut transposition_table = transposition_table.lock().unwrap();
                    transposition_table.insert(
                        new_cells_hash,
                        MAX_SCORE as i32,
                        depth,
                        current_player,
                    );
                }
            }
            return MAX_SCORE;
        }
        let eval = -evaluate_action(
            &new_cells,
            1 - current_player,
            available_actions[0],
            depth - 1,
            -beta,
            -alpha,
            end_time,
            transposition_table,
        );
        score = max(score, eval);
        alpha = max(alpha, score);
        if alpha > beta {
            if depth >= 3 {
                if let Some(transposition_table) = transposition_table {
                    let new_cells_hash = new_cells.hash();
                    let mut transposition_table = transposition_table.lock().unwrap();
                    transposition_table.insert(new_cells_hash, score as i32, depth, current_player);
                }
            }
            return score;
        }
        if depth == 2 {
            for action in available_actions.into_iter().skip(1) {
                let eval = {
                    let eval_null_window = -evaluate_action(
                        &new_cells,
                        1 - current_player,
                        action,
                        depth - 1,
                        -alpha - 1,
                        -alpha,
                        end_time,
                        transposition_table,
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
                            transposition_table,
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
        } else {
            let alpha_atomic = AtomicI32::new(alpha);
            let score_atomic = AtomicI32::new(score);
            let cut_atomic = AtomicBool::new(false);
            available_actions
                .into_iter()
                .skip(1)
                .par_bridge()
                .for_each(|action| {
                    if !cut_atomic.load(Relaxed) {
                        let eval = {
                            let eval_null_window = -evaluate_action(
                                &new_cells,
                                1 - current_player,
                                action,
                                depth - 1,
                                -alpha_atomic.load(Relaxed) - 1,
                                -alpha_atomic.load(Relaxed),
                                end_time,
                                transposition_table,
                            );
                            if alpha_atomic.load(Relaxed) < eval_null_window
                                && eval_null_window < beta
                            {
                                -evaluate_action(
                                    &new_cells,
                                    1 - current_player,
                                    action,
                                    depth - 1,
                                    -beta,
                                    -alpha_atomic.load(Relaxed),
                                    end_time,
                                    transposition_table,
                                )
                            } else {
                                eval_null_window
                            }
                        };
                        score_atomic.fetch_max(eval, Relaxed);
                        alpha_atomic.fetch_max(eval, Relaxed);
                        if alpha_atomic.load(Relaxed) > beta {
                            cut_atomic.store(true, Relaxed);
                        }
                    }
                });
            score = score_atomic.load(Relaxed);
            if let Some(transposition_table) = transposition_table {
                let new_cells_hash = new_cells.hash();
                let mut transposition_table = transposition_table.lock().unwrap();
                transposition_table.insert(new_cells_hash, score as i32, depth, current_player);
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
    cells: &Cells,
    current_player: Player,
    action: Action,
    previous_score: i32,
    previous_piece_scores: &[i32; N_CELLS],
) -> i32 {
    let (index_start, index_mid, index_end) = action.to_indices();

    if !cells[index_start].is_wise()
        && ((current_player == 1 && index_end.is_black_home())
            || (current_player == 0 && index_end.is_white_home()))
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
