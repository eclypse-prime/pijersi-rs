//! This module implements the code to generate pieces from their colour and type.
//!  
//! Pieces are represented by u8 numbers and have the following structure : TTCPTTCP
//!
//! It is separated in two parts: top (4 least significant bits) and bottom (4 most significant bits).
//! The bottom part can be empty.
//!
//! TT are 2 bits representing the type of the piece (Scissors, Paper, Rock, Wise)
//! C is 1 bit representing the color
//! P is 1 bit set to 1 as long as there is a piece

/// Bit width of a half piece
pub const HALF_PIECE_WIDTH: usize = 4;

/// Mandaory bit on pieces
pub const PIECE_BIT: u8 = 0b0001u8;
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

/// Represents the colour of a piece
pub enum PieceColour {
    /// White
    White,
    /// Black
    Black,
}

/// Represents the type of a piece
pub enum PieceType {
    /// Scissors
    Scissors,
    /// Paper
    Paper,
    /// Rock
    Rock,
    /// Wise
    Wise,
}

/// Creates a uint representation piece from a `PieceColour` and `PieceType`.
pub const fn piece_to_uint(piece_colour: &PieceColour, piece_type: &PieceType) -> u8 {
    let colour_uint: u8 = match piece_colour {
        PieceColour::White => COLOUR_WHITE,
        PieceColour::Black => COLOUR_BLACK,
    };
    let type_uint: u8 = match piece_type {
        PieceType::Scissors => TYPE_SCISSORS,
        PieceType::Paper => TYPE_PAPER,
        PieceType::Rock => TYPE_ROCK,
        PieceType::Wise => TYPE_WISE,
    };
    PIECE_BIT | colour_uint | type_uint
}

/// White Scissors
pub const WHITE_SCISSORS: u8 = piece_to_uint(&PieceColour::White, &PieceType::Scissors);
/// White Paper
pub const WHITE_PAPER: u8 = piece_to_uint(&PieceColour::White, &PieceType::Paper);
/// White Rock
pub const WHITE_ROCK: u8 = piece_to_uint(&PieceColour::White, &PieceType::Rock);
/// White Wise
pub const WHITE_WISE: u8 = piece_to_uint(&PieceColour::White, &PieceType::Wise);
/// Black Scissors
pub const BLACK_SCISSORS: u8 = piece_to_uint(&PieceColour::Black, &PieceType::Scissors);
/// Black Paper
pub const BLACK_PAPER: u8 = piece_to_uint(&PieceColour::Black, &PieceType::Paper);
/// Black Rock
pub const BLACK_ROCK: u8 = piece_to_uint(&PieceColour::Black, &PieceType::Rock);
/// Black Wise
pub const BLACK_WISE: u8 = piece_to_uint(&PieceColour::Black, &PieceType::Wise);

/// Creates a uint representation complete piece (top and bottom) from a `PieceColour` and the top and bottom `PieceType`.
pub fn init_piece(
    piece_colour: PieceColour,
    bottom_type: Option<PieceType>,
    top_type: PieceType,
) -> u8 {
    let top_uint: u8 = piece_to_uint(&piece_colour, &top_type);
    let bottom_uint: u8 = match bottom_type {
        None => CELL_EMPTY,
        Some(bottom_type) => piece_to_uint(&piece_colour, &bottom_type),
    };
    top_uint | bottom_uint << HALF_PIECE_WIDTH
}
