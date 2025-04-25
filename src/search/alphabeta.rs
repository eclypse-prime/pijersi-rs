//! This module implements the alphabeta search that chooses the best move

use std::cmp::{max, min};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::RwLock;
use std::time::Instant;

use rayon::prelude::*;

use crate::bitboard::Board;
use crate::hash::position::HashTrait;
use crate::hash::search::SearchTable;
use crate::logic::actions::{Action, ActionTrait, Actions, AtomicAction};
use crate::logic::index::CellIndexTrait;
use crate::logic::rules::is_action_win;
use crate::logic::translate::action_to_string;
use crate::logic::Player;
use crate::utils::{argsort, reverse_argsort};

use super::eval::{evaluate_position_incremental, evaluate_position, quiescence_search, MAX_SCORE};
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

/// Reads the transposition table and returns its entry (action, depth, score, node type) if it exists.
#[inline]
pub fn read_transposition_table(
    cells_hash: usize,
    transposition_table: Option<&RwLock<SearchTable>>,
) -> Option<(Action, u64, Score, NodeType)> {
    if let Some(transposition_table) = transposition_table {
        let transposition_table = transposition_table.read().unwrap();
        if let Some((table_depth, table_action, table_score, table_node_type)) =
            transposition_table.read(cells_hash)
        {
            return Some((table_action, table_depth, table_score, table_node_type));
        }
    }
    None
}

/// Write the transposition table and store an entry (action, depth, score, node type).
///
/// Replaces the stored entry if the new entry has a higher depth or is the same depth and is a PV node.
#[inline]
pub fn write_transposition_table(
    cells_hash: usize,
    action: Action,
    depth: u64,
    score: Score,
    node_type: NodeType,
    transposition_table: Option<&RwLock<SearchTable>>,
) {
    if let Some(transposition_table) = transposition_table {
        let mut transposition_table = transposition_table.write().unwrap();
        transposition_table.insert(cells_hash, depth, action, score, node_type);
    }
}

/// Sorts the available actions based on how good they are estimated to be (in descending order -> best actions first).
#[inline]
pub fn sort_actions(
    board: &Board,
    current_player: Player,
    table_action: Option<Action>,
    available_actions: &mut Actions,
) -> Option<Action> {
    let n_actions = available_actions.len();
    let mut index_sorted = 0;

    // If there is a TT action and it is part of the available actions, move it first
    if let Some(table_action) = table_action {
        for i in 0..n_actions {
            if available_actions[i] == table_action {
                // Immediately returns if action is win
                if is_action_win(board, table_action) {
                    return Some(table_action);
                }
                available_actions[..].swap(0, i);
                index_sorted = 1;
                break;
            }
        }
    }

    // Skip sorting the first action if there is a TT action
    let index_start = index_sorted;
    // Find all the captures and put them at the beginning
    for i in index_start..n_actions {
        let action = available_actions[i];
        let (_index_start, index_mid, index_end) = action.to_indices();
        // Immediately return if the action is a win
        if is_action_win(board, action) {
            return Some(action);
        }
        if (!index_mid.is_null()
            && board.capturable(current_player).get(index_mid))
            || (board.capturable(current_player).get(index_end))
        {
            available_actions[i] = available_actions[index_sorted];
            available_actions[index_sorted] = action;
            index_sorted += 1;
        }
    }
    None
}

/// Returns the best move at a given depth
pub fn search_root(
    board: &Board,
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
    let available_actions = board.available_player_actions(current_player);
    let n_actions = available_actions.len();

    let order = match scores {
        Some(scores) => argsort(scores, true),
        None => (0..n_actions).collect(),
    };

    if n_actions == 0 {
        return None;
    }

    let scores: Vec<Score> = {
        // Cutoffs will happen on winning actions
        let alpha = BASE_ALPHA;
        let beta = BASE_BETA;

        let mut scores: Vec<Score> = vec![-MAX_SCORE; n_actions];

        let static_eval = evaluate_position(board);

        let first_action = available_actions[order[0]];
        let first_eval = if is_action_win(board, first_action) {
            MAX_SCORE
        } else {
            // Principal Variation Search: search the first move with the full window, search subsequent moves with a null window first then if they fail high, search them with a full window
            let mut new_board = *board;
            new_board.play_action(first_action);
            let new_static_eval =
                evaluate_position_incremental(board, &new_board, first_action, static_eval);
            -search_node(
                (&new_board, 1 - current_player),
                depth - 1,
                (-beta, -alpha),
                end_time,
                NodeType::PV,
                transposition_table,
                new_static_eval,
            )
        };
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
                *score = {
                    if atomic_cut.load(Relaxed) {
                        Score::MIN
                    } else {
                        let action = available_actions[order[k]];
                        let eval = if is_action_win(board, action) {
                            MAX_SCORE
                        } else {
                            let mut new_board = *board;
                            new_board.play_action(action);
                            let new_static_eval =
                                evaluate_position_incremental(board, &new_board, action, static_eval);
                            let alpha = alpha_atomic.load(Relaxed);
                            // Search with a null window
                            let eval_null_window = -search_node(
                                (&new_board, 1 - current_player),
                                depth - 1,
                                (-alpha - 1, -alpha),
                                end_time,
                                NodeType::Cut,
                                transposition_table,
                                new_static_eval,
                            );
                            // If fail high, do the search with the full window
                            if alpha < eval_null_window && eval_null_window < beta {
                                -search_node(
                                    (&new_board, 1 - current_player),
                                    depth - 1,
                                    (-beta, -alpha),
                                    end_time,
                                    NodeType::PV,
                                    transposition_table,
                                    new_static_eval,
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
    };

    if let Some(end_time) = end_time {
        if Instant::now() > end_time {
            return None;
        }
    }

    let scores: Vec<Score> = reverse_argsort(&scores, &order);

    // for i in 0..scores.len() {
    //     println!("{} {}", action_to_string(cells, available_actions[i]), scores[i])
    // }

    let res = scores
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_index, &score)| score)
        .map(|(index, &score)| (available_actions[index], score))
        .map(|(action, score)| (action, score, scores));

    res
}

/// Evaluates the score of a given action by searching at a given depth.
///
/// Recursively calculates the best score using the alphabeta search to the chosen depth.
pub fn search_node(
    (board, current_player): (&Board, Player),
    depth: u64,
    (alpha, beta): (Score, Score),
    end_time: Option<Instant>,
    node_type: NodeType,
    transposition_table: Option<&RwLock<SearchTable>>,
    static_eval: Score,
) -> Score {
    if depth == 0 {
        return quiescence_search(board, current_player, (alpha, beta), static_eval);
    }

    // Stop searching if the allocated time is up (if there are time controls)
    if let Some(end_time) = end_time {
        if Instant::now() > end_time {
            return -MAX_SCORE;
        }
    }

    let mut available_actions = board.available_player_actions(current_player);
    let n_actions = available_actions.len();

    // If there are no actions available, the player has lost
    if n_actions == 0 {
        return -MAX_SCORE;
    }

    let mut score = -MAX_SCORE;

    let mut alpha = alpha;
    let mut beta = beta;
    // Read the transposition table
    let cells_hash = (board, current_player).hash();
    let table_action = match read_transposition_table(cells_hash, transposition_table) {
        Some((table_action, table_depth, table_score, table_node_type)) => {
            // If the table has a match with the same depth, a cutoff may be possible depending on the node type
            if table_depth == depth {
                match table_node_type {
                    NodeType::PV => return table_score,
                    NodeType::Cut => {
                        if table_score > beta {
                            return table_score;
                        }
                        alpha = table_score;
                    }
                    NodeType::All => {
                        if table_score < alpha {
                            return table_score;
                        }
                        beta = table_score;
                    }
                }
            }
            Some(table_action)
        }
        None => None,
    };

    // Sort actions to improve alphabeta search
    let winning_action = sort_actions(board, current_player, table_action, &mut available_actions);

    // Return if one of the available actions is an immediate win
    if let Some(winning_action) = winning_action {
        write_transposition_table(
            cells_hash,
            winning_action,
            depth,
            MAX_SCORE,
            NodeType::PV,
            transposition_table,
        );
        return MAX_SCORE;
    }

    // Principal Variation Search: search the first move with the full window, search subsequent moves with a null window first then if they fail high, search them with a full window
    // Evaluate first action sequentially
    let mut new_board = *board;
    let first_action = available_actions[0];
    new_board.play_action(first_action);
    let new_static_eval = evaluate_position_incremental(board, &new_board, first_action, static_eval);
    let eval = -search_node(
        (&new_board, 1 - current_player),
        depth - 1,
        (-beta, -alpha),
        end_time,
        match node_type {
            NodeType::PV => NodeType::PV,
            NodeType::Cut => NodeType::All,
            NodeType::All => NodeType::Cut,
        },
        transposition_table,
        new_static_eval,
    );
    alpha = max(alpha, eval);
    // Beta-cutoff, stop the search
    if alpha > beta {
        write_transposition_table(
            cells_hash,
            available_actions[0],
            depth,
            eval,
            node_type,
            transposition_table,
        );
        return eval;
    }
    score = max(score, eval);

    // Using atomic variables for parallel search
    let alpha_atomic = AtomicScore::new(alpha);
    let score_atomic = AtomicScore::new(score);
    let best_action_atomic = AtomicAction::new(available_actions[0]);
    // This will stop iteration if there is a cutoff
    let cut_atomic = AtomicBool::new(false);

    // Evaluate the rest of the actions in parallel
    available_actions
        .into_iter()
        .skip(1)
        .par_bridge()
        .for_each(|action| {
            if !cut_atomic.load(Relaxed) {
                let eval = {
                    let alpha = alpha_atomic.load(Relaxed);

                    let mut new_board = *board;
                    new_board.play_action(action);
                    let new_static_eval =
                        evaluate_position_incremental(board, &new_board, action, static_eval);
                    // Search with a null window
                    let eval_null_window = -search_node(
                        (&new_board, 1 - current_player),
                        depth - 1,
                        (-alpha - 1, -alpha),
                        end_time,
                        match node_type {
                            NodeType::PV => NodeType::Cut,
                            NodeType::Cut => NodeType::Cut,
                            NodeType::All => NodeType::Cut,
                        },
                        transposition_table,
                        new_static_eval,
                    );

                    // If fail high, do the search with the full window
                    if alpha < eval_null_window && eval_null_window < beta {
                        -search_node(
                            (&new_board, 1 - current_player),
                            depth - 1,
                            (-beta, -alpha),
                            end_time,
                            match node_type {
                                NodeType::PV => NodeType::PV,
                                NodeType::Cut => NodeType::Cut,
                                NodeType::All => NodeType::Cut,
                            },
                            transposition_table,
                            new_static_eval,
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
                // Beta-cutoff, stop the search
                if eval > beta {
                    cut_atomic.store(true, Relaxed);
                }
            }
        });
    score = score_atomic.load(Relaxed);
    write_transposition_table(
        cells_hash,
        best_action_atomic.load(Relaxed),
        depth,
        score,
        node_type,
        transposition_table,
    );
    score
}

/// Returns the best move by searching up to the chosen depth.
///
/// The search starts at depth 1 and the depth increases until the chosen depth is reached or a winning move is found.
/// The results at lower depths are used to sort the search order at higher depths.
pub fn search_iterative(
    board: &Board,
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
        let proposed_action = search_root(
            board,
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
                let action_string = action_to_string(board, action);
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
                        println!("info loss in {}", min(1, depth / 2));
                    }
                    best_result = if let Some((last_action, _last_score)) = best_result {
                        Some((last_action, score))
                    } else {
                        Some((action, score))
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
