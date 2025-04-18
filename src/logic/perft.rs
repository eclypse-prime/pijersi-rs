//!Implements perft, a debug function that calculates the number of leaf nodes at a given depth. It is used to assert that the move generator is correct.

use rayon::prelude::*;

use crate::piece::{Piece, PieceTrait};

use super::{
    actions::{play_action, Action, ActionTrait},
    index::{CellIndex, CellIndexTrait, INDEX_NULL},
    movegen::available_player_actions,
    rules::{can_move1, can_move2, can_stack, can_unstack, is_action_win},
    translate::action_to_string,
    Cells, Player,
};

/// Returns the number of possible actions for a player at a given position.
///
/// Is used to speed up perft at depth=1 since it only needs the number of leaf nodes, not the actions.
#[inline(always)]
fn count_player_actions_terminal(cells: &Cells, current_player: Player) -> u64 {
    cells
        .iter()
        .enumerate()
        .filter(|(_index, piece)| !piece.is_empty() && (piece.colour() == (current_player << 2)))
        .map(|(index, _piece)| count_piece_actions_terminal(cells, index))
        .sum::<u64>()
}

/// Returns the number of possible actions for a specific piece at a given position.
///
/// Is used to speed up perft at depth=1 since it only needs the number of leaf nodes, not the actions.
#[inline]
fn count_piece_actions_terminal(cells: &Cells, index_start: CellIndex) -> u64 {
    let mut piece_action_count: u64 = 0u64;

    let piece_start: Piece = cells[index_start];

    // If the piece is not a stack
    if piece_start.is_stack() {
        // 2 range first action
        for &index_mid in index_start.neighbours2() {
            if can_move2(cells, piece_start, index_start, index_mid) {
                // 2-range move, stack or unstack
                for &index_end in index_mid.neighbours1() {
                    // 2-range move, unstack or 2-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        piece_action_count += 1;
                    }
                }
                // 2-range move;
                piece_action_count += 1;
            }
        }
        // 1-range first action
        for &index_mid in index_start.neighbours1() {
            // 1-range move, [stack or unstack] optional
            if can_move1(cells, piece_start, index_mid) {
                // 1-range move, stack or unstack
                for &index_end in index_mid.neighbours1() {
                    // 1-range move, unstack or 1-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        piece_action_count += 1;
                    }
                }
                // 1-range move, unstack on starting position
                piece_action_count += 1;

                // 1-range move
                piece_action_count += 1;
            }
            // stack, [1/2-range move] optional
            else if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if can_move2(cells, piece_start, index_mid, index_end) {
                        piece_action_count += 1;
                    }
                }

                // stack, 1-range move
                for &index_end in index_mid.neighbours1() {
                    if can_move1(cells, piece_start, index_end) {
                        piece_action_count += 1;
                    }
                }

                // stack only
                piece_action_count += 1;
            }

            // unstack
            if can_unstack(cells, piece_start, index_mid) {
                // unstack only
                piece_action_count += 1;
            }
        }
    } else {
        // 1-range first action
        for &index_mid in index_start.neighbours1() {
            // stack, [1/2-range move] optional
            if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if can_move2(cells, piece_start, index_mid, index_end)
                        || (index_start == ((index_mid + index_end) / 2)
                            && can_move1(cells, piece_start, index_end))
                    {
                        piece_action_count += 1;
                    }
                }

                // stack, 0/1-range move
                for &index_end in index_mid.neighbours1() {
                    if can_move1(cells, piece_start, index_end) || index_start == index_end {
                        piece_action_count += 1;
                    }
                }

                // stack only
                piece_action_count += 1;
            }
            // 1-range move
            else if can_move1(cells, piece_start, index_mid) {
                piece_action_count += 1;
            }
        }
    }
    piece_action_count
}

/// Debug function to measure the number of leaf nodes (possible actions) at a given depth.
///
/// Recursively counts the number of leaf nodes at the chosen depth.
///
/// Uses parallel search.
///
/// At depth 0, returns 1.
pub fn perft(cells: &Cells, current_player: Player, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 | 2 => count_player_actions(cells, current_player, depth),
        _ => {
            let available_actions = available_player_actions(cells, current_player);

            available_actions
                .into_iter()
                .par_bridge()
                .filter(|&action| !is_action_win(cells, action))
                .map(|action| {
                    let mut new_cells: Cells = *cells;
                    play_action(&mut new_cells, action);
                    count_player_actions(&new_cells, 1 - current_player, depth - 1)
                })
                .sum()
        }
    }
}

/// Returns the number of leaf nodes (possible actions) for a player at a given depth and position after an action.
#[inline]
fn count_after_action(cells: &Cells, action: Action, current_player: Player, depth: u64) -> u64 {
    if is_action_win(cells, action) {
        0
    } else {
        let mut new_cells = *cells;
        play_action(&mut new_cells, action);
        count_player_actions(&new_cells, 1 - current_player, depth - 1)
    }
}

/// Part of the perft debug function to measure the number of leaf nodes (possible actions) at a given depth. Is called by perft.
///
/// Recursively counts the number of leaf nodes at the chosen depth.
///
/// At depth 0, returns 1.
#[inline(always)]
pub fn count_player_actions(cells: &Cells, current_player: Player, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 => count_player_actions_terminal(cells, current_player),
        _ => cells
            .iter()
            .enumerate()
            .filter(|(_index, piece)| {
                !piece.is_empty() && (piece.colour() == (current_player << 2))
            })
            .map(|(index, _piece)| count_piece_actions(cells, index, current_player, depth))
            .sum::<u64>(),
    }
}

/// Returns the number of leaf nodes (possible actions) for a player at a given depth and position.
#[inline]
fn count_piece_actions(
    cells: &Cells,
    index_start: CellIndex,
    current_player: Player,
    depth: u64,
) -> u64 {
    let mut piece_action_count: u64 = 0u64;

    let piece_start: Piece = cells[index_start];

    // If the piece is not a stack
    if piece_start.is_stack() {
        // 2 range first action
        for &index_mid in index_start.neighbours2() {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);
            if can_move2(cells, piece_start, index_start, index_mid) {
                // 2-range move, stack or unstack
                for &index_end in index_mid.neighbours1() {
                    // 2-range move, unstack or 2-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        piece_action_count += count_after_action(
                            cells,
                            half_action.add_last_index(index_end),
                            current_player,
                            depth,
                        );
                    }
                }
                // 2-range move
                piece_action_count += count_after_action(
                    cells,
                    Action::from_indices(index_start, INDEX_NULL, index_mid),
                    current_player,
                    depth,
                );
            }
        }
        // 1-range first action
        for &index_mid in index_start.neighbours1() {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);
            // 1-range move, [stack or unstack] optional
            if can_move1(cells, piece_start, index_mid) {
                // 1-range move, stack or unstack
                for &index_end in index_mid.neighbours1() {
                    // 1-range move, unstack or 1-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        piece_action_count += count_after_action(
                            cells,
                            half_action.add_last_index(index_end),
                            current_player,
                            depth,
                        );
                    }
                }
                // 1-range move, unstack on starting position
                piece_action_count += count_after_action(
                    cells,
                    Action::from_indices(index_start, index_mid, index_start),
                    current_player,
                    depth,
                );

                // 1-range move
                piece_action_count += count_after_action(
                    cells,
                    Action::from_indices(index_start, INDEX_NULL, index_mid),
                    current_player,
                    depth,
                );
            }
            // stack, [1/2-range move] optional
            else if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if can_move2(cells, piece_start, index_mid, index_end) {
                        piece_action_count += count_after_action(
                            cells,
                            half_action.add_last_index(index_end),
                            current_player,
                            depth,
                        );
                    }
                }

                // stack, 1-range move
                for &index_end in index_mid.neighbours1() {
                    if can_move1(cells, piece_start, index_end) {
                        piece_action_count += count_after_action(
                            cells,
                            half_action.add_last_index(index_end),
                            current_player,
                            depth,
                        );
                    }
                }

                // stack only
                piece_action_count += count_after_action(
                    cells,
                    Action::from_indices(index_start, index_start, index_mid),
                    current_player,
                    depth,
                );
            }

            // unstack
            if can_unstack(cells, piece_start, index_mid) {
                // unstack only
                piece_action_count += count_after_action(
                    cells,
                    Action::from_indices(index_start, index_start, index_mid),
                    current_player,
                    depth,
                );
            }
        }
    } else {
        // 1-range first action
        for &index_mid in index_start.neighbours1() {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);
            // stack, [1/2-range move] optional
            if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if can_move2(cells, piece_start, index_mid, index_end)
                        || (index_start == ((index_mid + index_end) / 2)
                            && can_move1(cells, piece_start, index_end))
                    {
                        piece_action_count += count_after_action(
                            cells,
                            half_action.add_last_index(index_end),
                            current_player,
                            depth,
                        );
                    }
                }

                // stack, 0/1-range move
                for &index_end in index_mid.neighbours1() {
                    if can_move1(cells, piece_start, index_end) || index_start == index_end {
                        piece_action_count += count_after_action(
                            cells,
                            half_action.add_last_index(index_end),
                            current_player,
                            depth,
                        );
                    }
                }

                // stack only
                piece_action_count += count_after_action(
                    cells,
                    Action::from_indices(index_start, index_start, index_mid),
                    current_player,
                    depth,
                );
            }
            // 1-range move
            else if can_move1(cells, piece_start, index_mid) {
                piece_action_count += count_after_action(
                    cells,
                    Action::from_indices(index_start, INDEX_NULL, index_mid),
                    current_player,
                    depth,
                );
            }
        }
    }
    piece_action_count
}

/// Split Perft debug function to measure the number of leaf nodes (possible actions) at a given depth.
///
/// Recursively counts the number of leaf nodes at the chosen depth.
///
/// Uses parallel search.
///
/// Separates the node count between all possible depth 1 actions and returns a vector of `(action_string: String, action: Action, count: u64)`.
///
/// At depth 0, returns an empty vector.
pub fn perft_split(
    cells: &Cells,
    current_player: Player,
    depth: u64,
) -> Vec<(String, Action, u64)> {
    if depth == 0 {
        vec![]
    } else {
        let available_actions = available_player_actions(cells, current_player);

        available_actions
            .into_iter()
            .par_bridge()
            .filter(|&action| !is_action_win(cells, action))
            .map(|action| {
                let mut new_cells = *cells;
                play_action(&mut new_cells, action);
                (
                    action_to_string(cells, action),
                    action,
                    count_player_actions(&new_cells, 1 - current_player, depth - 1),
                )
            })
            .collect()
    }
}
