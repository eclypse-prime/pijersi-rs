pub mod actions;
pub mod rules;

const INDEX_WIDTH: usize = 8;
const INDEX_MASK: u64 = 0xFFu64;
const NULL_MOVE: u64 = 0x00FFFFFFu64;

const HALF_PIECE_WIDTH: usize = 4;

const COLOUR_MASK: u8 = 0b0010u8;
const TYPE_MASK: u8 = 0b1100u8;
const TOP_MASK: u8 = 0b1111u8;

const COLOUR_WHITE: u8 = 0b0000u8;
const COLOUR_BLACK: u8  = 0b0010u8;

const TYPE_SCISSORS: u8  = 0b0000u8;
const TYPE_PAPER: u8  = 0b0100u8;
const TYPE_ROCK: u8  = 0b1000u8;
const TYPE_WISE: u8  = 0b1100u8;
