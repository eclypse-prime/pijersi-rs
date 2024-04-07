//! This module contains the necessary code to implement the game logic.
//! 
//! It contains the following sub-modules:
//! 
//! - actions: Implements the actions a player can choose (move, stack, unstack...).
//! - lookup: Implements the lookup tables used for faster computations in the move generator.
//! - movegen: Implements the move generator: returns the list of all available moves for a player at a given time.
//! - perft: Implements perft, a debug function that calculates the number of leaf nodes at a given depth. It is used to assert that the move generator is correct.
//! - rules: Implements the rules, tells if an action is valid or not.
//! - translate: Implements translation methods to convert the internal representation into a human-readable representation and vice versa.

pub mod actions;
pub mod lookup;
pub mod movegen;
pub mod perft;
pub mod rules;
pub mod translate;

const INDEX_WIDTH: usize = 8;
const INDEX_NULL: usize = 0xFFusize;
const INDEX_MASK: u64 = 0xFFu64;
const ACTION_NULL: u64 = 0x00FFFFFFu64;

const HALF_PIECE_WIDTH: usize = 4;

const COLOUR_MASK: u8 = 0b0010u8;
const TYPE_MASK: u8 = 0b1100u8;
const TOP_MASK: u8 = 0b1111u8;

const CELL_EMPTY: u8 = 0x00u8;
const STACK_THRESHOLD: u8 = 16u8;

const COLOUR_WHITE: u8 = 0b0000u8;
const COLOUR_BLACK: u8 = 0b0010u8;

const TYPE_SCISSORS: u8 = 0b0000u8;
const TYPE_PAPER: u8 = 0b0100u8;
const TYPE_ROCK: u8 = 0b1000u8;
const TYPE_WISE: u8 = 0b1100u8;

const MAX_PLAYER_ACTIONS: usize = 512;
