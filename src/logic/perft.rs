//!Implements perft, a debug function that calculates the number of leaf nodes at a given depth. It is used to assert that the move generator is correct.

use rayon::prelude::*;

use crate::{bitboard::Board, piece::PieceTrait};

use super::{
    actions::{Action, ActionTrait},
    index::{CellIndex, INDEX_NULL},
    lookup::NEIGHBOURS2,
    rules::is_action_win,
    translate::action_to_string,
    Player,
};

impl Board {
    /// Returns the number of possible actions for a player at a given position.
    ///
    /// Is used to speed up perft at depth=1 since it only needs the number of leaf nodes, not the actions.
    #[inline]
    pub fn count_player_actions(&self, current_player: Player) -> u64 {
        self.same_colour(current_player)
            .into_iter()
            .map(|index| self.count_piece_actions(index, current_player))
            .sum()
    }

    /// Returns the number of possible actions for a specific piece at a given position.
    ///
    /// Is used to speed up perft at depth=1 since it only needs the number of leaf nodes, not the actions.
    #[inline]
    fn count_piece_actions(&self, index_start: CellIndex, current_player: Player) -> u64 {
        let mut count: u64 = 0;
        let piece_start = self.get_player_piece(index_start, current_player);

        if piece_start.is_stack() {
            // 2-range first action
            for index_mid in self.available_moves2(index_start, piece_start) {
                // 2-range move, stack or unstack
                count += (self.available_unstacks(index_mid, piece_start)
                    | self.available_stacks(index_mid, piece_start))
                .0
                .count_ones() as u64;

                // 2-range move
                count += 1;
            }

            // 1-range first action
            for index_mid in self.available_moves1(index_start, piece_start) {
                // 1-range move, stack or unstack
                count += (self.available_unstacks(index_mid, piece_start)
                    | self.available_stacks(index_mid, piece_start))
                .0
                .count_ones() as u64;

                // 1-range move, unstack on starting position
                count += 1;

                // 1-range move
                count += 1;
            }

            // stack
            for index_mid in self.available_stacks(index_start, piece_start) {
                // stack, 1-range or 2-range move
                count += (self.available_moves2(index_mid, piece_start)
                    | self.available_moves1(index_mid, piece_start))
                .0
                .count_ones() as u64;

                // stack only
                count += 1;
            }

            // unstack
            count += self
                .available_unstacks(index_start, piece_start)
                .0
                .count_ones() as u64;
        } else {
            // 1-range first action
            for index_mid in self.available_stacks(index_start, piece_start) {
                // stack, 1-range or 2-range move
                count += (self.available_moves2(index_mid, piece_start)
                    | self.available_moves1(index_mid, piece_start)
                    | (NEIGHBOURS2[index_mid] & self.available_moves1(index_start, piece_start)))
                .0
                .count_ones() as u64;

                // stack, 1-range move to starting position
                count += 1;

                // stack only
                count += 1;
            }
            // 1-range move
            count += self
                .available_moves1(index_start, piece_start)
                .0
                .count_ones() as u64;
        }
        count
    }
}

/// Debug function to measure the number of leaf nodes (possible actions) at a given depth.
///
/// Recursively counts the number of leaf nodes at the chosen depth.
///
/// Uses parallel search.
///
/// At depth 0, returns 1.
pub fn perft(board: &Board, current_player: Player, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 | 2 => perft_player_actions(board, current_player, depth),
        _ => {
            let available_actions = board.available_player_actions(0);

            available_actions
                .into_iter()
                // .par_bridge()
                .filter(|&action| !is_action_win(board, action))
                .map(|action| {
                    let mut new_board = *board;
                    new_board.play_action(action);
                    perft_player_actions(&new_board, 1 - current_player, depth - 1)
                })
                .sum()
        }
    }
}

/// Returns the number of leaf nodes (possible actions) for a player at a given depth and position after an action.
#[inline]
fn perft_count_after_action(
    board: &Board,
    action: Action,
    current_player: Player,
    depth: u64,
) -> u64 {
    if is_action_win(board, action) {
        0
    } else {
        let mut new_board = *board;
        new_board.play_action(action);
        perft_player_actions(&new_board, 1 - current_player, depth - 1)
    }
}

/// Part of the perft debug function to measure the number of leaf nodes (possible actions) at a given depth. Is called by perft.
///
/// Recursively counts the number of leaf nodes at the chosen depth.
///
/// At depth 0, returns 1.
#[inline(always)]
pub fn perft_player_actions(board: &Board, current_player: Player, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 => board.count_player_actions(current_player),
        _ => board
            .same_colour(current_player)
            .into_iter()
            .map(|index| perft_piece_actions(board, index, current_player, depth))
            .sum(),
    }
}

/// Returns the number of leaf nodes (possible actions) for a player at a given depth and position.
#[inline]
fn perft_piece_actions(
    board: &Board,
    index_start: CellIndex,
    current_player: Player,
    depth: u64,
) -> u64 {
    let mut count = 0;
    let piece_start = board.get_piece(index_start);

    if piece_start.is_stack() {
        // 2-range first action

        for index_mid in board.available_moves2(index_start, piece_start) {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);

            count += perft_count_after_action(
                board,
                Action::from_indices(index_start, INDEX_NULL, index_mid),
                current_player,
                depth,
            );

            for index_end in board.available_unstacks(index_mid, piece_start)
                | board.available_stacks(index_mid, piece_start)
            {
                count += perft_count_after_action(
                    board,
                    half_action.add_last_index(index_end),
                    current_player,
                    depth,
                );
            }
        }

        for index_mid in board.available_moves1(index_start, piece_start) {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);

            for index_end in board.available_unstacks(index_mid, piece_start)
                | board.available_stacks(index_mid, piece_start)
            {
                count += perft_count_after_action(
                    board,
                    half_action.add_last_index(index_end),
                    current_player,
                    depth,
                );
            }
            // 1-range move, unstack on starting position
            count += perft_count_after_action(
                board,
                Action::from_indices(index_start, index_mid, index_start),
                current_player,
                depth,
            );

            // 1-range move
            count += perft_count_after_action(
                board,
                Action::from_indices(index_start, INDEX_NULL, index_mid),
                current_player,
                depth,
            );
        }

        for index_mid in board.available_stacks(index_start, piece_start) {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);

            for index_end in board.available_moves2(index_mid, piece_start)
                | board.available_moves1(index_mid, piece_start)
            {
                count += perft_count_after_action(
                    board,
                    half_action.add_last_index(index_end),
                    current_player,
                    depth,
                );
            }

            count += perft_count_after_action(
                board,
                Action::from_indices(index_start, index_start, index_mid),
                current_player,
                depth,
            );
        }

        for index_mid in board.available_unstacks(index_start, piece_start) {
            count += perft_count_after_action(
                board,
                Action::from_indices(index_start, index_start, index_mid),
                current_player,
                depth,
            );
        }
    } else {
        for index_mid in board.available_stacks(index_start, piece_start) {
            let half_action: Action = Action::from_indices_half(index_start, index_mid);

            for index_end in board.available_moves2(index_mid, piece_start)
                | board.available_moves1(index_mid, piece_start)
                | (NEIGHBOURS2[index_mid] & board.available_moves1(index_start, piece_start))
            {
                count += perft_count_after_action(
                    board,
                    half_action.add_last_index(index_end),
                    current_player,
                    depth,
                );
            }

            count += perft_count_after_action(
                board,
                half_action.add_last_index(index_start),
                current_player,
                depth,
            );

            count += perft_count_after_action(
                board,
                Action::from_indices(index_start, index_start, index_mid),
                current_player,
                depth,
            );
        }
        for index_mid in board.available_moves1(index_start, piece_start) {
            count += perft_count_after_action(
                board,
                Action::from_indices(index_start, INDEX_NULL, index_mid),
                current_player,
                depth,
            );
        }
    }

    count
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
    board: &Board,
    current_player: Player,
    depth: u64,
) -> Vec<(String, Action, u64)> {
    if depth == 0 {
        vec![]
    } else {
        let available_actions = board.available_player_actions(current_player);

        available_actions
            .into_iter()
            .par_bridge()
            .filter(|&action| !is_action_win(board, action))
            .map(|action| {
                let mut new_board = *board;
                new_board.play_action(action);
                (
                    action_to_string(board, action),
                    action,
                    perft_player_actions(&new_board, 1 - current_player, depth - 1),
                )
            })
            .collect()
    }
}
