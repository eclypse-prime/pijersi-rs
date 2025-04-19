//! Implements the move generator: returns the list of all available moves for a player at a given time.

use crate::piece::{Piece, PieceTrait};

use super::actions::{Action, ActionTrait, Actions};
use super::index::{CellIndex, CellIndexTrait, INDEX_NULL};
use super::rules::{can_move1, can_move2, can_stack, can_unstack};
use super::{Cells, Player, N_CELLS};

/// Returns the possible actions for a player.
/// The result is a `Actions` struct (fixed-length vector).
#[inline(always)]
pub fn available_player_actions(cells: &Cells, current_player: Player) -> Actions {
    let mut player_actions = Actions::default();

    // Calculate possible player_actions
    for index in 0..N_CELLS {
        if !cells[index].is_empty() {
            // Choose pieces of the current player's colour
            if (cells[index].colour()) == (current_player << 2) {
                available_piece_actions(cells, index, &mut player_actions);
            }
        }
    }
    player_actions
}

/// Calculates the possible actions for a piece.
/// The result is stored in a `Actions` struct (fixed-length vector).
/// This array is passed in parameter and modified by this function.
#[inline]
pub fn available_piece_actions(
    cells: &Cells,
    index_start: CellIndex,
    player_actions: &mut Actions,
) {
    let piece_start: Piece = cells[index_start];

    // If the piece is a stack
    if piece_start.is_stack() {
        // 2-range first action
        for &index_mid in index_start.neighbours2() {
            if can_move2(cells, piece_start, index_start, index_mid) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);
                // 2-range move, stack or unstack
                for &index_end in index_mid.neighbours1() {
                    // 2-range move, unstack or 2-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }
                // 2-range move
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
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
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }
                // 1-range move, unstack on starting position
                player_actions.push(Action::from_indices(index_start, index_mid, index_start));

                // 1-range move
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
            }
            // stack, [1/2-range move] optional
            else if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if can_move2(cells, piece_start, index_mid, index_end) {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }

                // stack, 1-range move
                for &index_end in index_mid.neighbours1() {
                    if can_move1(cells, piece_start, index_end) {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }

                // stack only
                player_actions.push(Action::from_indices(index_start, index_start, index_mid));
            }

            // unstack
            if can_unstack(cells, piece_start, index_mid) {
                // unstack only
                player_actions.push(Action::from_indices(index_start, index_start, index_mid));
            }
        }
    } else {
        // 1-range first action
        for &index_mid in index_start.neighbours1() {
            // stack, [1/2-range move] optional
            if can_stack(cells, piece_start, index_mid) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if can_move2(cells, piece_start, index_mid, index_end)
                        || (index_start == ((index_mid + index_end) / 2)
                            && can_move1(cells, piece_start, index_end))
                    {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }

                // stack, 0/1-range move
                for &index_end in index_mid.neighbours1() {
                    if can_move1(cells, piece_start, index_end) || index_start == index_end {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }

                // stack only
                player_actions.push(Action::from_indices(index_start, index_start, index_mid));
            }
            // 1-range move
            else if can_move1(cells, piece_start, index_mid) {
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
            }
        }
    }
}

/// Returns the possible captures for a player.
/// The result is a `Actions` struct (fixed-length vector).
#[inline(always)]
pub fn available_player_captures(cells: &Cells, current_player: Player) -> Actions {
    let mut player_actions = Actions::default();

    // Calculate possible player_actions
    for index in 0..N_CELLS {
        if !cells[index].is_empty() {
            if cells[index].is_wise() {
                continue;
            }
            // Choose pieces of the current player's colour
            if (cells[index].colour()) == (current_player << 2) {
                available_piece_captures(cells, index, &mut player_actions);
            }
        }
    }
    player_actions
}

#[inline]
fn is_piece_enemy(cells: &Cells, index: CellIndex, colour: u8) -> bool {
    !cells[index].is_empty() && cells[index].colour() != colour
}

/// Calculates the possible captures for a piece.
/// The result is stored in a `Actions` struct (fixed-length vector).
/// This array is passed in parameter and modified by this function.
#[inline]
pub fn available_piece_captures(
    cells: &Cells,
    index_start: CellIndex,
    player_actions: &mut Actions,
) {
    let piece_start: Piece = cells[index_start];
    let piece_colour = piece_start.colour();

    // If the piece is a stack
    if piece_start.is_stack() {
        // 2 range first action
        for &index_mid in index_start.neighbours2() {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);
            if can_move2(cells, piece_start, index_start, index_mid) {
                // First half-action is capture
                if is_piece_enemy(cells, index_mid, piece_colour) {
                    // 2-range move, stack or unstack
                    for &index_end in index_mid.neighbours1() {
                        // 2-range move, unstack or 2-range move, stack
                        if can_unstack(cells, piece_start, index_end)
                            || can_stack(cells, piece_start, index_end)
                        {
                            player_actions.push(half_action.add_last_index(index_end));
                        }
                    }
                    // 2-range move
                    player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
                } else {
                    // 2-range move, unstack (stack is skipped)
                    for &index_end in index_mid.neighbours1() {
                        // 2-range move, unstack
                        if is_piece_enemy(cells, index_end, piece_colour)
                            && can_unstack(cells, piece_start, index_end)
                        {
                            player_actions.push(half_action.add_last_index(index_end));
                        }
                    }
                    // 2-range move is not a capture, skip this one
                }
            }
        }
        // 1-range first action
        for &index_mid in index_start.neighbours1() {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);
            // 1-range move, [stack or unstack] optional
            if can_move1(cells, piece_start, index_mid) {
                if is_piece_enemy(cells, index_mid, piece_colour) {
                    // 1-range move, stack or unstack
                    for &index_end in index_mid.neighbours1() {
                        // 1-range move, unstack
                        if can_unstack(cells, piece_start, index_end)
                            || can_stack(cells, piece_start, index_end)
                        {
                            player_actions.push(half_action.add_last_index(index_end));
                        }
                    }
                    // 1-range move, unstack on starting position
                    player_actions.push(Action::from_indices(index_start, index_mid, index_start));

                    // 1-range move
                    player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
                } else {
                    // 1-range move and unstack (stack is skipped)
                    for &index_end in index_mid.neighbours1() {
                        // 1-range move and unstack
                        if is_piece_enemy(cells, index_end, piece_colour)
                            && can_unstack(cells, piece_start, index_end)
                        {
                            player_actions.push(half_action.add_last_index(index_end));
                        }
                    }
                    // 1-range move, unstack on starting position is not a capture, skip this one

                    // 1-range move is not a capture, skip this one
                }
            }
            // stack, 1/2-range move
            else if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if is_piece_enemy(cells, index_end, piece_colour)
                        && can_move2(cells, piece_start, index_mid, index_end)
                    {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }

                // stack, 1-range move
                for &index_end in index_mid.neighbours1() {
                    if is_piece_enemy(cells, index_end, piece_colour)
                        && can_move1(cells, piece_start, index_end)
                    {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }

                // stack only is not a capture, skip this one
            }

            // unstack
            if is_piece_enemy(cells, index_mid, piece_colour)
                && can_unstack(cells, piece_start, index_mid)
            {
                // unstack only
                player_actions.push(Action::from_indices(index_start, index_start, index_mid));
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
                    if is_piece_enemy(cells, index_end, piece_colour)
                        && (can_move2(cells, piece_start, index_mid, index_end)
                            || (index_start == ((index_mid + index_end) / 2)
                                && can_move1(cells, piece_start, index_end)))
                    {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }

                // stack, 1-range move (stack and move back is not a capture)
                for &index_end in index_mid.neighbours1() {
                    if is_piece_enemy(cells, index_end, piece_colour)
                        && can_move1(cells, piece_start, index_end)
                    {
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }

                // stack only is not a capture, skip this one
            }
            // 1-range move
            else if is_piece_enemy(cells, index_mid, piece_colour)
                && can_move1(cells, piece_start, index_mid)
            {
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
            }
        }
    }
}
