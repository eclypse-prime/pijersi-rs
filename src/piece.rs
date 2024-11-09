//! Implements the code to generate pieces from their colour and type.
//!  
//! Pieces are represented by u8 numbers and have the following structure : TTCPTTCP
//!
//! It is separated in two parts: top (4 least significant bits) and bottom (4 most significant bits).
//! The bottom part can be empty.
//!
//! TT are 2 bits representing the type of the piece (Scissors, Paper, Rock, Wise)
//! C is 1 bit representing the color
//! P is 1 bit set to 1 as long as there is a piece

/// A piece is represented as a u8.
pub type Piece = u8;

/// Bit width of a half piece
pub const HALF_PIECE_WIDTH: usize = 4;

/// Mandaory bit on pieces
pub const PIECE_BIT: Piece = 0b0001;
/// Mask to get the piece colour
pub const COLOUR_MASK: Piece = 0b0010;
/// Mask to get the piece type
pub const TYPE_MASK: Piece = 0b1100;
/// Mask to get the top piece
pub const TOP_MASK: Piece = 0b1111;

/// Empty cell value
pub const CELL_EMPTY: Piece = 0x00;
/// Cell value above which the cell contained inside is a stack
const STACK_THRESHOLD: Piece = 16;

/// White piece after applying the colour mask
pub const COLOUR_WHITE: Piece = 0b0000;
/// Black piece after applying the colour mask
pub const COLOUR_BLACK: Piece = 0b0010;

/// Scissors piece after applying the type mask
pub const TYPE_SCISSORS: Piece = 0b0000;
/// Paper piece after applying the type mask
pub const TYPE_PAPER: Piece = 0b0100;
/// Rock piece after applying the type mask
pub const TYPE_ROCK: Piece = 0b1000;
/// Wise piece after applying the type mask
pub const TYPE_WISE: Piece = 0b1100;

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
pub const fn piece_to_uint(piece_colour: &PieceColour, piece_type: &PieceType) -> Piece {
    let colour_part: Piece = match piece_colour {
        PieceColour::White => COLOUR_WHITE,
        PieceColour::Black => COLOUR_BLACK,
    };
    let type_part: Piece = match piece_type {
        PieceType::Scissors => TYPE_SCISSORS,
        PieceType::Paper => TYPE_PAPER,
        PieceType::Rock => TYPE_ROCK,
        PieceType::Wise => TYPE_WISE,
    };
    PIECE_BIT | colour_part | type_part
}

/// White Scissors
pub const WHITE_SCISSORS: Piece = piece_to_uint(&PieceColour::White, &PieceType::Scissors);
/// White Paper
pub const WHITE_PAPER: Piece = piece_to_uint(&PieceColour::White, &PieceType::Paper);
/// White Rock
pub const WHITE_ROCK: Piece = piece_to_uint(&PieceColour::White, &PieceType::Rock);
/// White Wise
pub const WHITE_WISE: Piece = piece_to_uint(&PieceColour::White, &PieceType::Wise);
/// Black Scissors
pub const BLACK_SCISSORS: Piece = piece_to_uint(&PieceColour::Black, &PieceType::Scissors);
/// Black Paper
pub const BLACK_PAPER: Piece = piece_to_uint(&PieceColour::Black, &PieceType::Paper);
/// Black Rock
pub const BLACK_ROCK: Piece = piece_to_uint(&PieceColour::Black, &PieceType::Rock);
/// Black Wise
pub const BLACK_WISE: Piece = piece_to_uint(&PieceColour::Black, &PieceType::Wise);

/// PieceTrait trait for Piece
pub trait PieceTrait: Copy {
    /// Stack the piece on the provided bottom piece
    fn stack_on(self, bottom: Self) -> Self;

    /// Get the top piece of a stack
    fn top(self) -> Self;
    /// Get the bottom piece of a stack
    fn bottom(self) -> Self;

    /// Applies the colour mask to the piece to find its colour
    fn colour(self) -> Self;
    /// Applies the type mask to the piece to find its type
    fn r#type(self) -> Self;

    /// Returns true if the piece is empty
    fn is_empty(self) -> bool;
    /// Returns true if the piece is a stack
    fn is_stack(self) -> bool;

    /// Returns true if the piece is white
    fn is_white(self) -> bool;
    /// Returns true if the piece is black
    fn is_black(self) -> bool;
    /// Returns true if the piece is a wise
    fn is_wise(self) -> bool;

    /// Sets the piece to an empty value
    fn set_empty(&mut self);
}

impl PieceTrait for Piece {
    #[inline(always)]
    fn stack_on(self, bottom: Self) -> Self {
        self.top() | (bottom << HALF_PIECE_WIDTH)
    }

    #[inline(always)]
    fn bottom(self) -> Self {
        self >> HALF_PIECE_WIDTH
    }

    #[inline(always)]
    fn top(self) -> Self {
        self & TOP_MASK
    }

    #[inline(always)]
    fn colour(self) -> Self {
        self & COLOUR_MASK
    }

    #[inline(always)]
    fn r#type(self) -> Self {
        self & TYPE_MASK
    }

    #[inline(always)]
    fn is_empty(self) -> bool {
        self == CELL_EMPTY
    }

    #[inline(always)]
    fn is_stack(self) -> bool {
        self >= STACK_THRESHOLD
    }

    #[inline(always)]
    fn is_white(self) -> bool {
        self.colour() == COLOUR_WHITE
    }

    #[inline(always)]
    fn is_black(self) -> bool {
        self.colour() == COLOUR_BLACK
    }

    #[inline(always)]
    fn is_wise(self) -> bool {
        self.r#type() == TYPE_WISE
    }

    #[inline(always)]
    fn set_empty(&mut self) {
        *self = CELL_EMPTY;
    }
}
