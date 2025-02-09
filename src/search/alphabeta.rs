//! This module implements the alphabeta search that chooses the best move

use std::cmp::max;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::RwLock;
use std::time::Instant;

use rayon::prelude::*;

use crate::hash::search::SearchTable;
use crate::logic::actions::Action;
use crate::logic::translate::action_to_string;
use crate::logic::{Cells, Player};
use crate::utils::{argsort, reverse_argsort};

use super::super::logic::movegen::available_player_actions;

use super::eval::{
    evaluate_action, evaluate_action_terminal, evaluate_position_with_details, MAX_SCORE,
};
use super::{AtomicScore, NodeType, Score};

/// Starting beta value for the alphabeta search (starting alpha is equal to -beta)
pub const BASE_BETA: Score = 8_192;
/// Starting alpha value for the alphabeta search (starting alpha is equal to -beta)
pub const BASE_ALPHA: Score = -BASE_BETA;

#[cfg(feature = "nps-count")]
use std::sync::atomic::AtomicU64;
#[cfg(feature = "nps-count")]
/// Counts the number of evaluated nodes during a search
pub static TOTAL_NODE_COUNT: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "nps-count")]
/// Increments the `TOTAL_NODE_COUNT` counter by the chosen amount.
pub fn increment_node_count(node_count: u64) {
    TOTAL_NODE_COUNT.fetch_add(node_count, Relaxed);
}

/// Returns the best move at a given depth
pub fn search(
    cells: &Cells,
    current_player: Player,
    depth: u64,
    end_time: Option<Instant>,
    scores: &Option<Vec<Score>>,
    transposition_table: Option<&RwLock<SearchTable>>,
) -> Option<(Action, Score, Vec<Score>)> {
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

    let scores: Vec<Score> = match depth {
        0 => return None,
        1 => {
            #[cfg(feature = "nps-count")]
            increment_node_count(n_actions as u64);
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
        // On depth 2, run the classic recursive search sequentially
        2 => {
            // Cutoffs will happen on winning moves
            let mut alpha = BASE_ALPHA;
            let beta = BASE_BETA;

            let mut scores: Vec<Score> = vec![-MAX_SCORE; n_actions];

            // Evaluate possible moves
            for k in 0..n_actions {
                let action = available_actions[order[k]];
                let eval = -evaluate_action(
                    (cells, 1 - current_player),
                    action,
                    depth - 1,
                    (-beta, -alpha),
                    end_time,
                    NodeType::PV,
                    transposition_table,
                );

                alpha = max(alpha, eval);
                scores[k] = eval;

                // Cutoff
                if eval > beta {
                    break;
                }
            }
            scores
        }
        // On depth > 2, run the classic recursive search sequentially with parallel search
        _ => {
            // Cutoffs will happen on winning actions
            let alpha = BASE_ALPHA;
            let beta = BASE_BETA;

            let mut scores: Vec<Score> = vec![-MAX_SCORE; n_actions];

            // Principal Variation Search: search the first move with the full window, search subsequent moves with a null window first then if they fail high, search them with a full window
            let first_eval = -evaluate_action(
                (cells, 1 - current_player),
                available_actions[order[0]],
                depth - 1,
                (-beta, -alpha),
                end_time,
                NodeType::PV,
                transposition_table,
            );
            scores[0] = first_eval;

            let alpha_atomic: AtomicScore = AtomicScore::new(max(alpha, first_eval));
            // This will stop iteration if there is a cutoff
            let atomic_cut: AtomicBool = AtomicBool::new(alpha_atomic.load(Relaxed) > beta);

            // Evaluate possible moves
            scores
                .iter_mut()
                .enumerate()
                .skip(1)
                .par_bridge()
                .for_each(|(k, score)| {
                    let action = available_actions[order[k]];
                    *score = {
                        if atomic_cut.load(Relaxed) {
                            Score::MIN
                        } else {
                            let eval = {
                                let alpha = alpha_atomic.load(Relaxed);
                                // Search with a null window
                                let eval_null_window = -evaluate_action(
                                    (cells, 1 - current_player),
                                    action,
                                    depth - 1,
                                    (-alpha - 1, -alpha),
                                    end_time,
                                    NodeType::Cut,
                                    transposition_table,
                                );
                                // If fail high, do the search with the full window
                                if alpha < eval_null_window && eval_null_window < beta {
                                    -evaluate_action(
                                        (cells, 1 - current_player),
                                        action,
                                        depth - 1,
                                        (-beta, -alpha),
                                        end_time,
                                        NodeType::PV,
                                        transposition_table,
                                    )
                                } else {
                                    eval_null_window
                                }
                            };

                            alpha_atomic.fetch_max(eval, Relaxed);

                            // Cutoff
                            if eval > beta {
                                atomic_cut.store(true, Relaxed);
                            }
                            eval
                        }
                    }
                });
            scores
        }
    };

    if let Some(end_time) = end_time {
        if Instant::now() > end_time {
            return None;
        }
    }

    let scores: Vec<Score> = reverse_argsort(&scores, &order);

    let res = scores
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_index, &score)| score)
        .map(|(index, &score)| (available_actions[index], score))
        .map(|(action, score)| (action, score, scores));

    res
}

/// Returns the best move by searching up to the chosen depth.
///
/// The search starts at depth 1 and the depth increases until the chosen depth is reached or a winning move is found.
/// The results at lower depths are used to sort the search order at higher depths.
pub fn search_iterative(
    cells: &Cells,
    current_player: Player,
    max_depth: u64,
    end_time: Option<Instant>,
    verbose: bool,
    transposition_table: Option<&RwLock<SearchTable>>,
) -> Option<(Action, Score)> {
    let mut best_result: Option<(Action, Score)> = None;
    let mut last_scores: Option<Vec<Score>> = None;
    let start_time = Instant::now();
    for depth in 1..=max_depth {
        if let Some(end_time) = end_time {
            if Instant::now() > end_time {
                break;
            }
        }
        let proposed_action = search(
            cells,
            current_player,
            depth,
            end_time,
            &last_scores,
            transposition_table,
        );
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
                    print!(
                        " nodes {} nps {}",
                        TOTAL_NODE_COUNT.load(Relaxed),
                        TOTAL_NODE_COUNT.load(Relaxed) as u128 * 1_000_000_000
                            / duration.as_nanos()
                    );
                    println!();
                }
                #[cfg(feature = "nps-count")]
                TOTAL_NODE_COUNT.store(0, Relaxed);
                if score < BASE_ALPHA {
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
