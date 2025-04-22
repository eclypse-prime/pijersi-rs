//! Implements the move generator: returns the list of all available moves for a player at a given time.

use crate::bitboard::Board;
use crate::piece::PieceTrait;

use super::actions::{Action, ActionTrait, Actions, ActionsLight};
use super::index::{CellIndex, INDEX_NULL};
use super::lookup::NEIGHBOURS2;
use super::{Player, N_CELLS};

impl Board {
    /// Returns the possible actions for a player.
    /// The result is a `Actions` struct (fixed-length vector).
    #[inline(always)]
    pub fn available_player_actions(&self, current_player: Player) -> Actions {
        let mut player_actions = Actions::default();

        // Calculate possible player_actions
        for index in 0..N_CELLS {
            // Choose pieces of the current player's colour
            if self.same_colour(current_player).get(index) {
                self.available_piece_actions(index, current_player, &mut player_actions);
            }
        }
        player_actions
    }

    /// Calculates the possible actions for a piece.
    /// The result is stored in a `Actions` struct (fixed-length vector).
    /// This array is passed in parameter and modified by this function.
    #[inline]
    pub fn available_piece_actions(
        &self,
        index_start: CellIndex,
        current_player: Player,
        player_actions: &mut Actions,
    ) {
        let piece_start = self.get_player_piece(index_start, current_player);

        if piece_start.is_stack() {
            // 2-range first action
            for index_mid in self.available_moves2(index_start, piece_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // 2-range move, stack or unstack
                for index_end in self.available_unstacks(index_mid, piece_start)
                    | self.available_stacks(index_mid, piece_start)
                {
                    player_actions.push(half_action.add_last_index(index_end));
                }

                // 2-range move
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
            }

            // 1-range first action
            for index_mid in self.available_moves1(index_start, piece_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // 1-range move, stack or unstack
                for index_end in self.available_unstacks(index_mid, piece_start)
                    | self.available_stacks(index_mid, piece_start)
                {
                    player_actions.push(half_action.add_last_index(index_end));
                }

                // 1-range move, unstack on starting position
                player_actions.push(Action::from_indices(index_start, index_mid, index_start));

                // 1-range move
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));

                // 1-range unstack
                player_actions.push(Action::from_indices(index_start, index_start, index_mid));
            }

            // stack
            for index_mid in self.available_stacks(index_start, piece_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // stack, 1-range or 2-range move
                for index_end in self.available_moves2(index_mid, piece_start)
                    | self.available_moves1(index_mid, piece_start)
                {
                    player_actions.push(half_action.add_last_index(index_end));
                }

                // stack only
                player_actions.push(Action::from_indices(index_start, index_start, index_mid));
            }
        } else {
            // 1-range first action
            for index_mid in self.available_stacks(index_start, piece_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // stack, 1-range or 2-range move
                for index_end in self.available_moves2(index_mid, piece_start)
                    | self.available_moves1(index_mid, piece_start)
                    | (NEIGHBOURS2[index_mid] & self.available_moves1(index_start, piece_start))
                {
                    player_actions.push(half_action.add_last_index(index_end));
                }

                // stack, 1-range move to starting position
                player_actions.push(half_action.add_last_index(index_start));

                // stack only
                player_actions.push(Action::from_indices(index_start, index_start, index_mid));
            }
            // 1-range move
            for index_mid in self.available_moves1(index_start, piece_start) {
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
            }
        }
    }

    /// Returns the possible captures for a player.
    /// The result is a `Actions` struct (fixed-length vector).
    pub fn available_player_captures(&self, current_player: Player) -> ActionsLight {
        let mut player_actions = ActionsLight::default();

        // Calculate possible player_actions
        for index in 0..N_CELLS {
            // Choose pieces of the current player's colour
            if self.same_colour_not_wise(current_player).get(index) {
                self.available_piece_captures(index, current_player, &mut player_actions);
            }
        }
        player_actions
    }

    /// Calculates the possible captures for a piece.
    /// The result is stored in a `Actions` struct (fixed-length vector).
    /// This array is passed in parameter and modified by this function.
    fn available_piece_captures(
        &self,
        index_start: CellIndex,
        current_player: Player,
        player_actions: &mut ActionsLight,
    ) {
        let piece_start = self.get_player_piece(index_start, current_player);

        if piece_start.is_stack() {
            // 2-range move, capture on unstack
            for index_mid in self.available_non_captures2(index_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // 2-range move, capture on unstack
                for index_end in self.available_captures1(index_mid, piece_start) {
                    player_actions.push(half_action.add_last_index(index_end));
                }
            }
            // 2-range capture
            for index_mid in self.available_captures2(index_start, piece_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // 2-range capture, stack or unstack
                for index_end in self.available_unstacks(index_mid, piece_start)
                    | self.available_stacks(index_mid, piece_start)
                {
                    player_actions.push(half_action.add_last_index(index_end));
                }

                // 2-range capture
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
            }

            // 1-range move, capture on unstack
            for index_mid in self.available_non_captures1(index_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // 1-range move, capture on unstack
                for index_end in self.available_captures1(index_mid, piece_start) {
                    player_actions.push(half_action.add_last_index(index_end));
                }
            }
            // 1-range capture
            for index_mid in self.available_captures1(index_start, piece_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // 1-range capture, stack or unstack
                for index_end in self.available_unstacks(index_mid, piece_start)
                    | self.available_stacks(index_mid, piece_start)
                {
                    player_actions.push(half_action.add_last_index(index_end));
                }

                // 1-range capture, unstack on starting position
                player_actions.push(Action::from_indices(index_start, index_mid, index_start));

                // 1-range capture on move
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
                
                // 1-range capture on unstack
                player_actions.push(Action::from_indices(index_start, index_start, index_mid));
            }

            // stack
            for index_mid in self.available_stacks(index_start, piece_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // stack, 1-range or 2-range capture
                for index_end in self.available_captures2(index_mid, piece_start)
                    | self.available_captures1(index_mid, piece_start)
                {
                    player_actions.push(half_action.add_last_index(index_end));
                }
            }
        } else {
            // 1-range first action
            for index_mid in self.available_stacks(index_start, piece_start) {
                let half_action: Action = Action::from_indices_half(index_start, index_mid);

                // stack, 1-range or 2-range capture
                for index_end in self.available_captures2(index_mid, piece_start)
                    | self.available_captures1(index_mid, piece_start)
                    | (NEIGHBOURS2[index_mid] & self.available_captures1(index_start, piece_start))
                {
                    player_actions.push(half_action.add_last_index(index_end));
                }
            }
            // 1-range capture
            for index_mid in self.available_captures1(index_start, piece_start) {
                player_actions.push(Action::from_indices(index_start, INDEX_NULL, index_mid));
            }
        }
    }
}
