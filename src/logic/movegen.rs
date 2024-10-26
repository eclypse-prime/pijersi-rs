//! Implements the move generator: returns the list of all available moves for a player at a given time.

use arrayvec::ArrayVec;

use crate::piece::Piece;

use super::actions::Action;
use super::index::{Index, INDEX_NULL};
use super::rules::{can_move1, can_move2, can_stack, can_unstack};

/// Size of the array that stores player actions
pub const MAX_PLAYER_ACTIONS: usize = 511;

/// Returns the possible moves for a player.
/// The result is a size `MAX_PLAYER_ACTIONS` array of u64 and the number of actions.
#[inline]
pub fn available_player_actions(
    cells: &[u8; 45],
    current_player: u8,
) -> ArrayVec<u64, MAX_PLAYER_ACTIONS> {
    let mut player_actions: ArrayVec<u64, MAX_PLAYER_ACTIONS> = ArrayVec::new();

    // Calculate possible player_actions
    for index in 0..45 {
        if !cells[index].is_empty() {
            // Choose pieces of the current player's colour
            if (cells[index].colour()) == (current_player << 1) {
                available_piece_actions(cells, index, &mut player_actions);
            }
        }
    }
    player_actions
}

/// Calculates the possible moves for a player.
/// The result is stored in a size `MAX_PLAYER_ACTIONS`. The function returns the last used index.
/// This array is passed in parameter and modified by this function.
#[inline]
pub fn available_piece_actions(
    cells: &[u8; 45],
    index_start: usize,
    player_actions: &mut ArrayVec<u64, MAX_PLAYER_ACTIONS>,
) {
    let piece_start: u8 = cells[index_start];

    // If the piece is not a stack
    if piece_start.is_stack() {
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
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }
                // 2-range move
                player_actions.push(u64::from_indices(index_start, INDEX_NULL, index_mid));
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
                        player_actions.push(half_action.add_last_index(index_end));
                    }
                }
                // 1-range move, unstack on starting position
                player_actions.push(u64::from_indices(index_start, index_mid, index_start));

                // 1-range move
                player_actions.push(u64::from_indices(index_start, INDEX_NULL, index_mid));
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
                player_actions.push(u64::from_indices(index_start, index_start, index_mid));
            }

            // unstack
            if can_unstack(cells, piece_start, index_mid) {
                // unstack only
                player_actions.push(u64::from_indices(index_start, index_start, index_mid));
            }
        }
    } else {
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
                player_actions.push(u64::from_indices(index_start, index_start, index_mid));
            }
            // 1-range move
            else if can_move1(cells, piece_start, index_mid) {
                player_actions.push(u64::from_indices(index_start, INDEX_NULL, index_mid));
            }
        }
    }
}
