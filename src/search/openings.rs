//! This module contains the OpeningBook struct and its associated functions.
//!
//! It contains the opening book data in the form of a HashMap.
//! The keys are strings representing the Pijersi Standard Notation of the stored position.
//! The values are the stored actions in the native triple-index format (u64) and the expected score at the pre-calculated search depth.
//! 
//! The stored actions contain search depth values (see [`crate::logic::actions`]).

use std::collections::HashMap;

#[derive(Debug)]
/// The OpeningBook struct containing the opening book data.
pub struct OpeningBook {
    map: HashMap<String, (u64, i64)>,
}

const OPENINGS_FILE: &str = include_str!("../../data/openings.txt");

// TODO: use anyerror
/// Converts a \[psn\];\[action\];\[score\] string to a (psn, action, score) tuple
fn line_to_tuple(line: &str) -> Option<(String, (u64, i64))> {
    let words: Vec<&str> = line.split(';').collect();
    let state = words.first();
    let action_str = words.get(1);
    let score_str = words.get(2);
    if let (Some(state), Some(action_str), Some(score_str)) = (state, action_str, score_str) {
        let state = (*state).to_owned();
        let action: u64 = (*action_str).parse::<u64>().ok()?;
        let score: i64 = (*score_str).parse::<i64>().ok()?;
        Some((state, (action, score)))
    } else {
        None
    }
}

impl OpeningBook {
    /// Created a new OpeningBook.
    /// Loads the precompiled opening book.
    pub fn new() -> OpeningBook {
        let opening_lines: Vec<&str> = OPENINGS_FILE.lines().collect();
        let map: HashMap<String, (u64, i64)> = opening_lines
            .iter()
            .filter_map(|&line| line_to_tuple(line))
            .collect();
        OpeningBook { map }
    }

    /// Looks for a stored move corresponding to the provided board state and returns it if it exists.
    pub fn lookup(&self, state: &str) -> Option<&(u64, i64)> {
        self.map.get(state)
    }
}

impl Default for OpeningBook {
    fn default() -> Self {
        Self::new()
    }
}
