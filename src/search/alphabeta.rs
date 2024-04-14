use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI64};

use rayon::prelude::*;

use super::super::logic::{movegen::available_player_actions, MAX_PLAYER_ACTIONS};

use super::eval::{evaluate_action, evaluate_action_terminal, evaluate_position_with_details};

pub const BASE_BETA: i64 = 262144;

pub fn search(cells: &[u8; 45], current_player: u8, depth: u64) -> Option<u64> {
    if depth == 0 {
        return None;
    }

    // Get an array of all the available moves for the current player, the last element of the array is the number of available moves
    let available_actions: [u64; 512] = available_player_actions(current_player, cells);
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

    scores
        .iter()
        .enumerate()
        .max_by_key(|(_index, &score)| score).map(|(index_best_move, _score)| available_actions[index_best_move])
}
