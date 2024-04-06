

/// A piece is separated in two parts: top (4 least significant bits) and bottom (4 most significant bits).
/// 
/// The bottom part can be empty.
/// 
/// 

pub enum PieceColour {
    White,
    Black,
}

pub enum PieceType {
    Scissors,
    Paper,
    Rock,
    Wise,
}

/// Creates a uint representation piece from a PieceColour and PieceType.
pub fn piece_to_uint(piece_colour: &PieceColour, piece_type: &PieceType) -> u8 {
    let colour_uint: u8 = match piece_colour {
        PieceColour::White => 0b0000,
        PieceColour::Black => 0b0010,
    };
    let type_uint: u8 = match piece_type {
        PieceType::Scissors => 0b0000,
        PieceType::Paper => 0b0100,
        PieceType::Rock => 0b1000,
        PieceType::Wise => 0b1100,
    };
    0b0001 | colour_uint | type_uint
}

/// Creates a uint representation complete piece (top and bottom) from a PieceColour and the top and bottom PieceType.
pub fn init_piece(
    piece_colour: PieceColour,
    bottom_type: Option<PieceType>,
    top_type: PieceType,
) -> u8 {
    let top_uint: u8 = piece_to_uint(&piece_colour, &top_type);
    let bottom_uint: u8 = match bottom_type {
        None => 0u8,
        Some(bottom_type) => piece_to_uint(&piece_colour, &bottom_type),
    };
    top_uint | bottom_uint << 4
}