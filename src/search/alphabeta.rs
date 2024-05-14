use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI64};
use std::time::Instant;

use rayon::prelude::*;

use crate::logic::translate::action_to_string;

use super::super::logic::{movegen::available_player_actions, MAX_PLAYER_ACTIONS};

use super::eval::{evaluate_action, evaluate_action_terminal, evaluate_position_with_details};

pub const BASE_BETA: i64 = 262_144;

/// Returns the best move at a given depth
pub fn search(
    cells: &[u8; 45],
    current_player: u8,
    depth: u64,
    end_time: Option<Instant>,
) -> Option<(u64, i64, Vec<i64>)> {
    if depth == 0 {
        return None;
    }

    if end_time.is_some() && Instant::now() > end_time.unwrap() {
        return None;
    }

    // Get an array of all the available moves for the current player, the last element of the array is the number of available moves
    let available_actions: [u64; 512] = available_player_actions(cells, current_player);
    let n_actions: usize = available_actions[MAX_PLAYER_ACTIONS - 1] as usize;

    if n_actions == 0 {
        return None;
    }

    let scores: Vec<i64> = if depth == 1 {
        // On depth 1, run the lightweight eval, only calculating score differences on cells that changed (incremental eval)
        let (previous_score, previous_piece_scores) = evaluate_position_with_details(cells);
        available_actions
            .iter()
            .take(n_actions)
            .map(|&action| {
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
        let alpha: AtomicI64 = AtomicI64::new(-BASE_BETA);
        let beta = BASE_BETA;

        // This will stop iteration if there is a cutoff
        let cut: AtomicBool = AtomicBool::new(false);

        // Evaluate possible moves
        available_actions
            .par_iter()
            .take(n_actions)
            .enumerate()
            .map(|(k, &action)| {
                if cut.load(Relaxed) {
                    i64::MIN
                } else {
                    let eval = if k == 0 {
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
            })
            .collect()
    };

    // println!("{scores:?}");

    if end_time.is_some() && Instant::now() > end_time.unwrap() {
        return None;
    }

    scores
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_index, &score)| score)
        .map(|(index_best_move, &score)| (available_actions[index_best_move], score))
        .map(|(best_action, best_score)| (best_action, best_score, scores))
}

pub fn search_iterative(
    cells: &[u8; 45],
    current_player: u8,
    max_depth: u64,
    end_time: Option<Instant>,
) -> Option<(u64, i64)> {
    let mut best_result: Option<(u64, i64)> = None;
    for depth in 1..=max_depth {
        if end_time.is_some() && Instant::now() > end_time.unwrap() {
            break;
        }
        let start_time = Instant::now();
        let proposed_action = search(cells, current_player, depth, end_time);
        let duration: f64 = start_time.elapsed().as_micros() as f64 / 1000f64;
        match proposed_action {
            None => (),
            Some((action, score, scores)) => {
                let action_string = action_to_string(cells, action);
                println!("info depth {depth} time {duration} score {score} pv {action_string}");
                best_result = Some((action, score));
            }
        }
    }
    best_result
}
