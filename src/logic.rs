//! This module contains the necessary code to implement the game logic.

pub mod actions;
pub mod lookup;
pub mod movegen;
pub mod perft;
pub mod rules;
pub mod translate;

pub const INDEX_WIDTH: usize = 8;
pub const INDEX_NULL: usize = 0xFFusize;
pub const INDEX_MASK: u64 = 0xFFu64;

pub const HALF_PIECE_WIDTH: usize = 4;

pub const COLOUR_MASK: u8 = 0b0010u8;
pub const TYPE_MASK: u8 = 0b1100u8;
pub const TOP_MASK: u8 = 0b1111u8;

pub const CELL_EMPTY: u8 = 0x00u8;
pub const STACK_THRESHOLD: u8 = 16u8;

pub const COLOUR_WHITE: u8 = 0b0000u8;
pub const COLOUR_BLACK: u8 = 0b0010u8;

pub const TYPE_SCISSORS: u8 = 0b0000u8;
pub const TYPE_PAPER: u8 = 0b0100u8;
pub const TYPE_ROCK: u8 = 0b1000u8;
pub const TYPE_WISE: u8 = 0b1100u8;

pub const MAX_PLAYER_ACTIONS: usize = 512;
