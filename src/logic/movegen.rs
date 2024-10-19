//! Implements the move generator: returns the list of all available moves for a player at a given time.

use crate::piece::Piece;

use super::actions::Action;
use super::index::Index;
use super::rules::{can_move1, can_move2, can_stack, can_unstack};
use super::{INDEX_NULL, MAX_PLAYER_ACTIONS};

/// Returns the possible moves for a player.
/// The result is a size `MAX_PLAYER_ACTIONS` array of u64 and the number of actions.
#[inline(always)]
pub fn available_player_actions(
    cells: &[u8; 45],
    current_player: u8,
) -> ([u64; MAX_PLAYER_ACTIONS], usize) {
    let mut player_actions: [u64; MAX_PLAYER_ACTIONS] = [0u64; MAX_PLAYER_ACTIONS];
    let mut index_actions: usize = 0;

    // Calculate possible player_actions
    for index in 0..45 {
        if !cells[index].is_empty() {
            // Choose pieces of the current player's colour
            if (cells[index].colour()) == (current_player << 1) {
                index_actions =
                    available_piece_actions(cells, index, &mut player_actions, index_actions);
            }
        }
    }
    (player_actions, index_actions)
}

/// Calculates the possible moves for a player.
/// The result is stored in a size `MAX_PLAYER_ACTIONS`. The function returns the last used index.
/// This array is passed in parameter and modified by this function.
#[inline]
pub fn available_piece_actions(
    cells: &[u8; 45],
    index_start: usize,
    player_actions: &mut [u64; MAX_PLAYER_ACTIONS],
    index_actions: usize,
) -> usize {
    let piece_start: u8 = cells[index_start];
    let mut index_actions = index_actions;

    // If the piece is not a stack
    if !piece_start.is_stack() {
        // 1-range first action
        for &index_mid in index_start.neighbours1() {
            let half_action: u64 = u64::from_indices_half(index_start, index_mid);
            // stack, [1/2-range move] optional
            if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if can_move2(cells, piece_start, index_mid, index_end)
                        || (index_start == ((index_mid + index_end) / 2)
                            && can_move1(cells, piece_start, index_end))
                    {
                        player_actions[index_actions] = half_action.add_last_index(index_end);
                        index_actions += 1;
                    }
                }

                // stack, 0/1-range move
                for &index_end in index_mid.neighbours1() {
                    if can_move1(cells, piece_start, index_end) || index_start == index_end {
                        player_actions[index_actions] = half_action.add_last_index(index_end);
                        index_actions += 1;
                    }
                }

                // stack only
                player_actions[index_actions] =
                    u64::from_indices(index_start, index_start, index_mid);
                index_actions += 1;
            }
            // 1-range move
            else if can_move1(cells, piece_start, index_mid) {
                player_actions[index_actions] =
                    u64::from_indices(index_start, INDEX_NULL, index_mid);
                index_actions += 1;
            }
        }
    } else {
        // 2 range first action
        for &index_mid in index_start.neighbours2() {
            let half_action: u64 = u64::from_indices_half(index_start, index_mid);
            if can_move2(cells, piece_start, index_start, index_mid) {
                // 2-range move, stack or unstack
                for &index_end in index_mid.neighbours1() {
                    // 2-range move, unstack or 2-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        player_actions[index_actions] = half_action.add_last_index(index_end);
                        index_actions += 1;
                    }
                }
                // 2-range move
                player_actions[index_actions] =
                    u64::from_indices(index_start, INDEX_NULL, index_mid);
                index_actions += 1;
            }
        }
        // 1-range first action
        for &index_mid in index_start.neighbours1() {
            let half_action: u64 = u64::from_indices_half(index_start, index_mid);
            // 1-range move, [stack or unstack] optional
            if can_move1(cells, piece_start, index_mid) {
                // 1-range move, stack or unstack
                for &index_end in index_mid.neighbours1() {
                    // 1-range move, unstack or 1-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        player_actions[index_actions] = half_action.add_last_index(index_end);
                        index_actions += 1;
                    }
                }
                // 1-range move, unstack on starting position
                player_actions[index_actions] =
                    u64::from_indices(index_start, index_mid, index_start);
                index_actions += 1;

                // 1-range move
                player_actions[index_actions] =
                    u64::from_indices(index_start, INDEX_NULL, index_mid);
                index_actions += 1;
            }
            // stack, [1/2-range move] optional
            else if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in index_mid.neighbours2() {
                    if can_move2(cells, piece_start, index_mid, index_end) {
                        player_actions[index_actions] = half_action.add_last_index(index_end);
                        index_actions += 1;
                    }
                }

                // stack, 1-range move
                for &index_end in index_mid.neighbours1() {
                    if can_move1(cells, piece_start, index_end) {
                        player_actions[index_actions] = half_action.add_last_index(index_end);
                        index_actions += 1;
                    }
                }

                // stack only
                player_actions[index_actions] =
                    u64::from_indices(index_start, index_start, index_mid);
                index_actions += 1;
            }

            // unstack
            if can_unstack(cells, piece_start, index_mid) {
                // unstack only
                player_actions[index_actions] =
                    u64::from_indices(index_start, index_start, index_mid);
                index_actions += 1;
            }
        }
    }
    index_actions
}
