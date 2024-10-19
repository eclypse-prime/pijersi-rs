//! This module contains custom errors for this crate.

use std::{fmt::Display, num::ParseIntError};

use thiserror::Error;

use crate::logic::actions::Action;

/// General Pijersi errors
#[derive(Debug, Error)]
pub enum RuntimeError {
    /// Broken rules
    #[error("Rules error at {}:{}:{}.", file!(), line!(), column!())]
    Rules(#[from] RulesErrorKind),
    /// Failed parsing
    #[error("Parsing error at {}:{}:{}.", file!(), line!(), column!())]
    Parse(#[from] ParseError),
    /// UGI engine error
    #[error("UGI error at {}:{}:{}.", file!(), line!(), column!())]
    UGI(#[from] UgiErrorKind),
}

/// Errors returned if game rules are broken
#[derive(Debug, Error)]
pub enum RulesErrorKind {
    /// Illegal action
    #[error("This action is illegal: {0} ({} {} {}).", .0.to_indices().0, .0.to_indices().1, .0.to_indices().2)]
    IllegalAction(u64),
}

/// Errors returned if parsing fails
#[derive(Debug, Error)]
#[error("Could not parse the following value: \"{}\".", self.value)]
pub struct ParseError {
    /// The kind of parsing error
    #[source]
    pub kind: ParseErrorKind,
    /// The value that caused the error
    pub value: String,
}

/// The different kinds of parsing errors
#[derive(Debug, Error)]
pub enum ParseErrorKind {
    /// Invalid action
    #[error("Invalid action string. Expected \"a1b1c1\" or \"a1b1\" format.")]
    InvalidAction,
    /// Invalid position

    #[error("Invalid position string. See documentation at https://github.com/eclypse-prime/pijersi-rs/blob/main/UGI.md.")]
    InvalidPosition(#[from] InvalidPositionKind),
    /// Invalid PSN string
    #[error("Invalid Pijersi Standard Notation string. See documentation at https://github.com/eclypse-prime/pijersi-rs/blob/main/UGI.md.")]
    InvalidPSN,
    /// Invalid coordinates
    #[error("Invalid {kind} coordinate '{value}'.")]
    InvalidCoordinates {
        /// The kind of coordinates error (vertical or horizontal)
        kind: InvalidCoordinatesKind,
        /// The value that caused the error
        value: char,
    },
    /// Invalid player
    #[error("Invalid Player.")]
    InvalidPlayer(#[from] InvalidPlayerKind),
    /// Invalid bool
    #[error("Invalid bool string. Expected \"true\" or \"false\".")]
    InvalidBool,
    /// Invalid int
    #[error("Invalid int string.")]
    InvalidInt(#[from] ParseIntError),
}

/// The different kinds of invalid position errors
#[derive(Debug, Error)]
pub enum InvalidPositionKind {
    /// Wrong number of lines
    #[error("Invalid number of lines in board notation: {0} (expected 7)")]
    WrongLineNumber(usize),
}

/// The kind of coordinates error (vertical or horizontal)
#[derive(Debug)]
pub enum InvalidCoordinatesKind {
    /// Vertical
    Vertical,
    /// Horizontal
    Horizontal,
}

/// The different kinds of invalid player errors
#[derive(Debug, Error)]
pub enum InvalidPlayerKind {
    /// String to player
    #[error("Got {0}, expected \"w\" or \"b\".")]
    StrToPlayer(String),
    /// Player to string
    #[error("Got {0}, expected 0 or 1.")]
    PlayerToStr(u8),
}

impl Display for InvalidCoordinatesKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => write!(f, "vertical"),
            Self::Horizontal => write!(f, "horizontal"),
        }
    }
}

/// UGI engine errors
#[derive(Debug, Error)]
pub enum UgiErrorKind {
    /// Empty command
    #[error("Empty command")]
    EmptyCommand,
    /// Invalid position arguments
    #[error("Invalid position arguments: \"{0}\", expected optional \"moves [moves]\"")]
    InvalidUGIPosition(String),
    /// Clap error
    #[error("Command parsing error.")]
    ClapError(#[from] clap::Error),
}

/// Gets the error traceback as a String vector.
pub fn get_error_trace(error: &dyn std::error::Error) -> Vec<String> {
    let mut error: &dyn std::error::Error = error;
    let mut trace: Vec<String> = vec![error.to_string()];
    while let Some(source) = error.source() {
        trace.push(source.to_string());
        error = source;
    }
    trace
}
