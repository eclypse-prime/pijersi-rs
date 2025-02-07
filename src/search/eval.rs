//! This module implements the evaluation functions: evaluates the score of a current position or evaluates the best score at a given depth.

use std::cmp::max;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32};
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
use super::Score;

/// The max score (is reached on winning position)
pub const MAX_SCORE: Score = 524_288;

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
pub fn evaluate_position_with_details(cells: &Cells) -> (Score, [Score; N_CELLS]) {
    let mut piece_scores: [Score; N_CELLS] = [0i32; N_CELLS];
    for (k, &cell) in cells.iter().enumerate() {
        piece_scores[k] = evaluate_cell(cell, k);
    }
    (piece_scores.iter().sum(), piece_scores)
}

#[inline]
fn read_transposition_table(
    cells_hash: usize,
    current_player: Player,
    transposition_table: Option<&Mutex<SearchTable>>,
) -> Option<(Action, u64)> {
    if let Some(transposition_table) = transposition_table {
        let mut transposition_table = transposition_table.lock().unwrap();
        if let Some((table_depth, table_player, table_action)) =
            transposition_table.read(cells_hash)
        {
            if table_player == current_player {
                Some((table_action, table_depth))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[inline]
fn write_transposition_table(
    cells_hash: usize,
    current_player: Player,
    action: Action,
    depth: u64,
    table_depth: Option<u64>,
    transposition_table: Option<&Mutex<SearchTable>>,
) {
    if let Some(transposition_table) = transposition_table {
        if let Some(table_depth) = table_depth {
            if depth > table_depth {
                let mut transposition_table = transposition_table.lock().unwrap();
                transposition_table.insert(cells_hash, depth, current_player, action);
            }
        } else {
            let mut transposition_table = transposition_table.lock().unwrap();
            transposition_table.insert(cells_hash, depth, current_player, action);
        }
    }
}

/// Sorts the available actions based on how good they are estimated to be (in descending order -> best actions first).
#[inline]
pub fn sort_actions(
    cells: &Cells,
    current_player: Player,
    table_action: Option<Action>,
    available_actions: &mut Actions,
) -> Option<Action> {
    let n_actions = available_actions.len();
    let mut index_sorted = 0;
    if let Some(table_action) = table_action {
        for i in 0..n_actions {
            if available_actions[i] == table_action {
                available_actions[i] = available_actions[0];
                available_actions[0] = table_action;
                index_sorted = 1;
                break;
            }
        }
    }
    let index_start = index_sorted;
    for i in index_start..n_actions {
        let action = available_actions[i];
        let (index_start, index_mid, index_end) = action.to_indices();
        if !cells[index_start].is_wise()
            && ((current_player == 0 && index_end.is_black_home())
                || (current_player == 1 && index_end.is_white_home())
                || ((current_player == 0 && !index_mid.is_null() && index_mid.is_black_home())
                    || (current_player == 1 && !index_mid.is_null() && index_mid.is_white_home())))
        {
            return Some(action);
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
    None
}

/// Evaluates the score of a given action by searching at a given depth.
///
/// Recursively calculates the best score using the alphabeta search to the chosen depth.
pub fn evaluate_action(
    cells: &Cells,
    current_player: Player,
    action: Action,
    depth: u64,
    (alpha, beta): (Score, Score),
    end_time: Option<Instant>,
    transposition_table: Option<&Mutex<SearchTable>>,
) -> Score {
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
            return -MAX_SCORE;
        }
    }

    let mut available_actions = available_player_actions(&new_cells, current_player);
    let n_actions = available_actions.len();

    if n_actions == 0 {
        return -MAX_SCORE;
    }

    let mut score = -MAX_SCORE;

    let mut alpha = alpha;
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
        let new_cells_hash = new_cells.hash();
        let (table_action, table_depth) =
            match read_transposition_table(new_cells_hash, current_player, transposition_table) {
                Some((table_action, table_depth)) => (Some(table_action), Some(table_depth)),
                None => (None, None),
            };
        let winning_action = sort_actions(
            &new_cells,
            current_player,
            table_action,
            &mut available_actions,
        );
        if let Some(winning_action) = winning_action {
            write_transposition_table(
                new_cells_hash,
                current_player,
                winning_action,
                depth,
                table_depth,
                transposition_table,
            );
            return MAX_SCORE;
        }
        let eval = -evaluate_action(
            &new_cells,
            1 - current_player,
            available_actions[0],
            depth - 1,
            (-beta, -alpha),
            end_time,
            transposition_table,
        );
        alpha = max(alpha, eval);
        if alpha > beta {
            write_transposition_table(
                new_cells_hash,
                current_player,
                available_actions[0],
                depth,
                table_depth,
                transposition_table,
            );
            return eval;
        }
        score = eval;
        if depth == 2 {
            let mut best_action = available_actions[0];
            for action in available_actions.into_iter().skip(1) {
                let eval = {
                    let eval_null_window = -evaluate_action(
                        &new_cells,
                        1 - current_player,
                        action,
                        depth - 1,
                        (-alpha - 1, -alpha),
                        end_time,
                        transposition_table,
                    );
                    if alpha < eval_null_window && eval_null_window < beta {
                        -evaluate_action(
                            &new_cells,
                            1 - current_player,
                            action,
                            depth - 1,
                            (-beta, -alpha),
                            end_time,
                            transposition_table,
                        )
                    } else {
                        eval_null_window
                    }
                };
                if eval > score {
                    score = eval;
                    best_action = action;
                }
                alpha = max(alpha, eval);
                if alpha > beta {
                    break;
                }
            }
            write_transposition_table(
                new_cells_hash,
                current_player,
                best_action,
                depth,
                table_depth,
                transposition_table,
            );
        } else {
            let alpha_atomic = AtomicI32::new(alpha);
            let score_atomic = AtomicI32::new(score);
            let best_action_atomic = AtomicU32::new(available_actions[0]);
            let cut_atomic = AtomicBool::new(false);
            available_actions
                .into_iter()
                .skip(1)
                .par_bridge()
                .for_each(|action| {
                    if !cut_atomic.load(Relaxed) {
                        let eval = {
                            let alpha = alpha_atomic.load(Relaxed);

                            let eval_null_window = -evaluate_action(
                                &new_cells,
                                1 - current_player,
                                action,
                                depth - 1,
                                (-alpha - 1, -alpha),
                                end_time,
                                transposition_table,
                            );
                            if alpha < eval_null_window
                                && eval_null_window < beta
                            {
                                -evaluate_action(
                                    &new_cells,
                                    1 - current_player,
                                    action,
                                    depth - 1,
                                    (-beta, -alpha),
                                    end_time,
                                    transposition_table,
                                )
                            } else {
                                eval_null_window
                            }
                        };
                        if eval > score_atomic.load(Relaxed) {
                            score_atomic.store(eval, Relaxed);
                            best_action_atomic.store(action, Relaxed);
                        }
                        alpha_atomic.fetch_max(eval, Relaxed);
                        if eval > beta {
                            cut_atomic.store(true, Relaxed);
                        }
                    }
                });
            score = score_atomic.load(Relaxed);
            write_transposition_table(
                new_cells_hash,
                current_player,
                best_action_atomic.load(Relaxed),
                depth,
                table_depth,
                transposition_table,
            );
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
    previous_score: Score,
    previous_piece_scores: &[Score; N_CELLS],
) -> Score {
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
