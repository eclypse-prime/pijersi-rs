//! This module contains the `OpeningBook` struct and its associated functions.
//!
//! It contains the opening book data in the form of a `HashMap`.
//! The keys are strings representing the Pijersi Standard Notation of the stored position.
//! The values are the stored actions in the native triple-index format (`Action`) and the expected score at the pre-calculated search depth.
//!
//! The stored actions contain search depth values (see [`crate::logic::actions`]).

#![allow(unused)]

use std::collections::HashMap;

use miniz_oxide::inflate::decompress_to_vec;
use serde::{Deserialize, Serialize};

use crate::{
    game::Game,
    logic::{actions::Action, Player, N_CELLS},
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
/// Represents a board's cells and current player. They are used to index the opening book.
pub struct Position {
    #[serde(with = "serde_bytes")]
    /// The current cells storing the piece data as `Piece` (see [`crate::piece`])
    pub cells: [u8; N_CELLS],
    /// The current player: 0 if white, 1 if black
    pub current_player: Player,
}

impl Position {
    /// Creates a new `Position` from a board. Copies its cells and current player.
    pub fn new(game: &Game) -> Self {
        Self {
            cells: [0; N_CELLS],
            current_player: game.current_player,
        }
    }

    const fn empty() -> Self {
        Self {
            cells: [0; N_CELLS],
            current_player: 0,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
/// Represents a pre-calculated response to a given position. It is used to generate the opening book `HashMap`.
pub struct Response {
    /// The position that is used as a key
    pub position: Position,
    /// The pre-calculated response
    pub action: u64,
    /// The predicted score of the response
    pub score: i64,
    // TODO: rewrite everything with action: u32 and score: Score
}
const RESPONSE_SIZE: usize = 70;

impl Response {
    /// Creates a new Response
    pub fn new(position: Position, action: Action, score: i64) -> Self {
        Self {
            position,
            action: action as u64,
            score,
        }
    }
    fn empty() -> Self {
        Self {
            position: Position::empty(),
            action: 0,
            score: 0,
        }
    }
}

#[derive(Debug)]
/// The `OpeningBook` struct containing the opening book data.
pub struct OpeningBook {
    map: HashMap<Position, (Action, i64)>,
}

const OPENINGS_BYTES_COMPRESSED: &[u8] = include_bytes!("../../data/openings");

fn decode_response(response_bytes: &[u8; RESPONSE_SIZE]) -> Option<Response> {
    None
}

fn decode_responses(responses_bytes: &[u8]) -> Vec<Response> {
    let n_responses = responses_bytes.len() / RESPONSE_SIZE;
    let mut responses: Vec<Response> = Vec::with_capacity(n_responses);
    let openings_bytes_chunks = responses_bytes.chunks(RESPONSE_SIZE);
    for response_bytes in openings_bytes_chunks {
        if let Ok(response_bytes) = response_bytes.try_into() {
            if let Some(response) = decode_response(&response_bytes) {
                responses.push(response);
            }
        }
    }
    responses
}

impl OpeningBook {
    /// Created a new `OpeningBook`.
    /// Loads the precompiled opening book.
    pub fn new() -> Self {
        let openings_bytes = decompress_to_vec(OPENINGS_BYTES_COMPRESSED).unwrap();
        // assert!(RESPONSE_SIZE == serialized_size(&Response::empty()).unwrap() as usize);
        assert!(openings_bytes.len() % RESPONSE_SIZE == 0);
        let responses = decode_responses(&openings_bytes);
        let map: HashMap<Position, (Action, i64)> = responses
            .iter()
            .map(|&response| {
                (
                    response.position,
                    (response.action as Action, response.score),
                )
            })
            .collect();
        Self { map }
    }

    /// Looks for a stored move corresponding to the provided board state and returns it if it exists.
    pub fn lookup(&self, board: &Game) -> Option<&(Action, i64)> {
        self.map.get(&Position::new(board))
    }
}

impl Default for OpeningBook {
    fn default() -> Self {
        Self::new()
    }
}
