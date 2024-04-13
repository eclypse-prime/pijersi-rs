use std::cmp::max;

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
    // Cutoffs will happen on winning moves
    let mut alpha = -BASE_BETA;
    let beta = BASE_BETA;

    // This will stop iteration if there is a cutoff
    let mut cut = false;

    let mut best_action: Option<u64> = None;
    let mut best_score: i64 = i64::MIN;

    // On depth 1, run the lightweight eval, only calculating score differences on cells that changed (incremental eval)
    if depth == 1 {
        let (previous_score, previous_piece_scores) = evaluate_position_with_details(cells);
        for &action in available_actions.iter().take(n_actions) {
            let eval = -evaluate_action_terminal(
                cells,
                1 - current_player,
                action,
                previous_score,
                &previous_piece_scores,
            );
            if eval > best_score {
                best_score = eval;
                best_action = Some(action);
            }
            alpha = max(alpha, best_score);
            if alpha > beta {
                break;
            }
        }
    }
    // On depth > 1, run the classic recursive search, with the lowest depth being parallelized
    else {
        // Evaluate possible moves
        for (k, &action) in available_actions.iter().take(n_actions).enumerate() {
            if cut {
                continue;
            }

            let eval = if k == 0 {
                -evaluate_action(cells, 1 - current_player, action, depth - 1, -beta, -alpha)
            } else {
                // Search with a null window
                let eval_null_window = -evaluate_action(
                    cells,
                    1 - current_player,
                    action,
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                );
                // If fail high, do the search with the full window
                if alpha < eval_null_window && eval_null_window < beta {
                    -evaluate_action(cells, 1 - current_player, action, depth - 1, -beta, -alpha)
                } else {
                    eval_null_window
                }
            };

            if eval >= best_score {
                best_score = eval;
                best_action = Some(action);
            }

            if eval > alpha {
                alpha = eval;
            }

            // Cutoff
            if alpha > beta {
                cut = true;
            }
        }
    }

    best_action
}
