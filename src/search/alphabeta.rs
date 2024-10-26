//! This module implements the alphabeta search that chooses the best move

use std::cmp::max;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI64};
use std::time::Instant;

use rayon::prelude::*;

use crate::logic::translate::action_to_string;
use crate::utils::{argsort, reverse_argsort};

use super::super::logic::movegen::available_player_actions;

use super::eval::{evaluate_action, evaluate_action_terminal, evaluate_position_with_details};

/// Starting beta value for the alphabeta search (starting alpha is equal to -beta)
pub const BASE_BETA: i64 = 262_144;
/// Starting alpha value for the alphabeta search (starting alpha is equal to -beta)
pub const BASE_ALPHA: i64 = -BASE_BETA;

#[cfg(feature = "nps-count")]
use std::sync::atomic::AtomicU64;
#[cfg(feature = "nps-count")]
/// Counts the number of evaluated nodes during a search
pub static mut TOTAL_NODE_COUNT: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "nps-count")]
/// Increments the `TOTAL_NODE_COUNT` counter by the chosen amount.
pub unsafe fn increment_node_count(node_count: u64) {
    TOTAL_NODE_COUNT.fetch_add(node_count, Relaxed);
}

/// Returns the best move at a given depth
pub fn search(
    cells: &[u8; 45],
    current_player: u8,
    depth: u64,
    end_time: Option<Instant>,
    scores: &Option<Vec<i64>>,
) -> Option<(u64, i64, Vec<i64>)> {
    if depth == 0 {
        return None;
    }

    if let Some(end_time) = end_time {
        if Instant::now() > end_time {
            return None;
        }
    }

    // Get an array of all the available moves for the current player, the last element of the array is the number of available moves
    let available_actions = available_player_actions(cells, current_player);
    let n_actions = available_actions.len();

    let order = match scores {
        Some(scores) => argsort(scores, true),
        None => (0..n_actions).collect(),
    };

    if n_actions == 0 {
        return None;
    }

    let scores: Vec<i64> = if depth == 1 {
        #[cfg(feature = "nps-count")]
        unsafe {
            increment_node_count(n_actions as u64);
        }
        // On depth 1, run the lightweight eval, only calculating score differences on cells that changed (incremental eval)
        let (previous_score, previous_piece_scores) = evaluate_position_with_details(cells);
        order
            .iter()
            .map(|&index| available_actions[index])
            .map(|action| {
                -evaluate_action_terminal(
                    cells,
                    1 - current_player,
                    action,
                    previous_score,
                    &previous_piece_scores,
                )
            })
            .collect()
    }
    // On depth > 1, run the classic recursive search, with the lowest depth being parallelized
    else {
        // Cutoffs will happen on winning moves
        let alpha = BASE_ALPHA;
        let beta = BASE_BETA;

        let mut scores: Vec<i64> = vec![0; n_actions];
        let first_eval = -evaluate_action(
            cells,
            1 - current_player,
            available_actions[order[0]],
            depth - 1,
            -beta,
            -alpha,
            end_time,
        );
        scores[0] = first_eval;

        let alpha: AtomicI64 = AtomicI64::new(max(alpha, first_eval));
        // This will stop iteration if there is a cutoff
        let cut: AtomicBool = AtomicBool::new(alpha.load(Relaxed) > beta);

        // Evaluate possible moves
        scores
            .iter_mut()
            .enumerate()
            .skip(1)
            .par_bridge()
            .for_each(|(k, score)| {
                let action = available_actions[order[k]];
                *score = {
                    if cut.load(Relaxed) {
                        i64::MIN
                    } else {
                        let eval = {
                            // Search with a null window
                            let eval_null_window = -evaluate_action(
                                cells,
                                1 - current_player,
                                action,
                                depth - 1,
                                -alpha.load(Relaxed) - 1,
                                -alpha.load(Relaxed),
                                end_time,
                            );
                            // If fail high, do the search with the full window
                            if alpha.load(Relaxed) < eval_null_window && eval_null_window < beta {
                                -evaluate_action(
                                    cells,
                                    1 - current_player,
                                    action,
                                    depth - 1,
                                    -beta,
                                    -alpha.load(Relaxed),
                                    end_time,
                                )
                            } else {
                                eval_null_window
                            }
                        };

                        alpha.fetch_max(eval, Relaxed);

                        // Cutoff
                        if alpha.load(Relaxed) > beta {
                            cut.store(true, Relaxed);
                        }
                        eval
                    }
                }
            });
        scores
    };

    if let Some(end_time) = end_time {
        if Instant::now() > end_time {
            return None;
        }
    }

    let scores: Vec<i64> = reverse_argsort(&scores, &order);

    scores
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_index, &score)| score)
        .map(|(index, &score)| (available_actions[index], score))
        .map(|(action, score)| (action, score, scores))
}

/// Returns the best move by searching up to the chosen depth.
///
/// The search starts at depth 1 and the depth increases until the chosen depth is reached or a winning move is found.
/// The results at lower depths are used to sort the search order at higher depths.
pub fn search_iterative(
    cells: &[u8; 45],
    current_player: u8,
    max_depth: u64,
    end_time: Option<Instant>,
    verbose: bool,
) -> Option<(u64, i64)> {
    let mut best_result: Option<(u64, i64)> = None;
    let mut last_scores: Option<Vec<i64>> = None;
    let start_time = Instant::now();
    for depth in 1..=max_depth {
        if let Some(end_time) = end_time {
            if Instant::now() > end_time {
                break;
            }
        }
        let proposed_action = search(cells, current_player, depth, end_time, &last_scores);
        let duration = start_time.elapsed();
        let duration_ms: u128 = duration.as_millis();
        match proposed_action {
            None => (),
            Some((action, score, scores)) => {
                let action_string = action_to_string(cells, action);
                if verbose {
                    print!(
                        "info depth {depth} time {duration_ms} score {score} pv {action_string}"
                    );
                    #[cfg(feature = "nps-count")]
                    unsafe {
                        print!(
                            " nodes {} nps {}",
                            TOTAL_NODE_COUNT.load(Relaxed),
                            TOTAL_NODE_COUNT.load(Relaxed) as u128 * 1_000_000
                                / duration.as_micros()
                        );
                    }
                    println!();
                }
                #[cfg(feature = "nps-count")]
                unsafe {
                    TOTAL_NODE_COUNT.store(0, Relaxed);
                }
                if score < -BASE_BETA {
                    if verbose {
                        println!("info loss in {}", depth / 2);
                    }
                    best_result = if let Some((last_action, _last_score)) = best_result {
                        Some((last_action, score))
                    } else {
                        None
                    };
                    break;
                }
                best_result = Some((action, score));
                last_scores = Some(scores);
                if score > BASE_BETA {
                    if verbose {
                        if depth > 1 {
                            println!("info mate in {}", depth / 2);
                        } else {
                            println!("info mate");
                        }
                    }
                    break;
                }
            }
        }
    }
    best_result
}
