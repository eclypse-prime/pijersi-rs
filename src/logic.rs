//! This module contains the necessary code to implement the game logic.

pub mod actions;
pub mod lookup;
pub mod movegen;
pub mod perft;
pub mod rules;
pub mod translate;

/// Bit width of a move index
pub const INDEX_WIDTH: usize = 8;
/// Value of a null index contained in a move
pub const INDEX_NULL: usize = 0xFFusize;
/// Mask to get the first index of a move (rightmost)
pub const INDEX_MASK: u64 = 0xFFu64;

/// Bit width of a half piece
pub const HALF_PIECE_WIDTH: usize = 4;

/// Mask to get the piece colour
pub const COLOUR_MASK: u8 = 0b0010u8;
/// Mask to get the piece type
pub const TYPE_MASK: u8 = 0b1100u8;
/// Mask to get the top piece
pub const TOP_MASK: u8 = 0b1111u8;

/// Empty cell value
pub const CELL_EMPTY: u8 = 0x00u8;
/// Cell value above which the cell contained inside is a stack
pub const STACK_THRESHOLD: u8 = 16u8;

/// White piece after applying the colour mask
pub const COLOUR_WHITE: u8 = 0b0000u8;
/// Black piece after applying the colour mask
pub const COLOUR_BLACK: u8 = 0b0010u8;

/// Scissors piece after applying the type mask
pub const TYPE_SCISSORS: u8 = 0b0000u8;
/// Paper piece after applying the type mask
pub const TYPE_PAPER: u8 = 0b0100u8;
/// Rock piece after applying the type mask
pub const TYPE_ROCK: u8 = 0b1000u8;
/// Wise piece after applying the type mask
pub const TYPE_WISE: u8 = 0b1100u8;

/// Size of the array that stores player actions
pub const MAX_PLAYER_ACTIONS: usize = 512;
