//! This module implements various helper functions.

use crate::errors::StringParseError;

/// Returns a vector of sorted indices
pub fn argsort<T: Ord>(data: &[T], reverse: bool) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    if reverse {
        indices.reverse();
    }
    indices
}

/// Reverse operation of argsort, uses an array of sorted indices to create the original unsorted vector
pub fn reverse_argsort<T: Clone>(original: &[T], indices: &[usize]) -> Vec<T> {
    let mut sorted = Vec::from(original);

    for (index_original, &index) in indices.iter().enumerate() {
        sorted[index] = original[index_original].clone();
    }

    sorted
}

/// Parses bool arguments in string format ("true", "false"). Returns None if the value is anything else.
pub fn parse_bool_arg(argument: &str) -> Option<bool> {
    if argument == "true" {
        Some(true)
    } else if argument == "false" {
        Some(false)
    } else {
        None
    }
}

pub fn parse_player_arg(player: &str) -> Result<u8, StringParseError> {
    match player {
        "w" => Ok(0u8),
        "b" => Ok(1u8),
        _ => {
            Err(StringParseError::new(&format!("Unknown player {player}")))
        }
    }
}
