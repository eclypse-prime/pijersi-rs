//! This module implements various helper functions.

use crate::errors::{ParseError, ParseErrorKind};

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

/// Parses bool arguments in string format ("true", "false"). Returns an error if the value is anything else.
pub fn parse_bool_arg(argument: &str) -> Result<bool, ParseError> {
    if argument == "true" {
        Ok(true)
    } else if argument == "false" {
        Ok(false)
    } else {
        Err(ParseError {
            kind: ParseErrorKind::InvalidBool,
            value: argument.to_owned(),
        })
    }
}
