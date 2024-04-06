pub struct Board {
    pub cells: [u8; 45],
    pub current_player: u8,
}

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

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [0u8; 45],
            current_player: 0u8,
        }
    }

    pub fn init(&mut self) {
        self.cells[0] = init_piece(PieceColour::Black, None, PieceType::Scissors);
        self.cells[1] = init_piece(PieceColour::Black, None, PieceType::Paper);
        self.cells[2] = init_piece(PieceColour::Black, None, PieceType::Rock);
        self.cells[3] = init_piece(PieceColour::Black, None, PieceType::Scissors);
        self.cells[4] = init_piece(PieceColour::Black, None, PieceType::Paper);
        self.cells[5] = init_piece(PieceColour::Black, None, PieceType::Rock);
        self.cells[6] = init_piece(PieceColour::Black, None, PieceType::Paper);
        self.cells[7] = init_piece(PieceColour::Black, None, PieceType::Rock);
        self.cells[8] = init_piece(PieceColour::Black, None, PieceType::Scissors);
        self.cells[9] = init_piece(PieceColour::Black, Some(PieceType::Wise), PieceType::Wise);
        self.cells[10] = init_piece(PieceColour::Black, None, PieceType::Rock);
        self.cells[11] = init_piece(PieceColour::Black, None, PieceType::Scissors);
        self.cells[12] = init_piece(PieceColour::Black, None, PieceType::Paper);

        self.cells[44] = init_piece(PieceColour::White, None, PieceType::Scissors);
        self.cells[43] = init_piece(PieceColour::White, None, PieceType::Paper);
        self.cells[42] = init_piece(PieceColour::White, None, PieceType::Rock);
        self.cells[41] = init_piece(PieceColour::White, None, PieceType::Scissors);
        self.cells[40] = init_piece(PieceColour::White, None, PieceType::Paper);
        self.cells[39] = init_piece(PieceColour::White, None, PieceType::Rock);
        self.cells[38] = init_piece(PieceColour::White, None, PieceType::Paper);
        self.cells[37] = init_piece(PieceColour::White, None, PieceType::Rock);
        self.cells[36] = init_piece(PieceColour::White, None, PieceType::Scissors);
        self.cells[35] = init_piece(PieceColour::White, Some(PieceType::Wise), PieceType::Wise);
        self.cells[34] = init_piece(PieceColour::White, None, PieceType::Rock);
        self.cells[32] = init_piece(PieceColour::White, None, PieceType::Paper);
        self.cells[33] = init_piece(PieceColour::White, None, PieceType::Scissors);
    }

    pub fn print(&self) {
        print!(" ");
        for i in 0..45 {
            let piece: u8 = self.cells[i];
            let top_piece: u8 = piece & 0b1111;
            let bottom_piece: u8 = piece >> 4;
            let char1: char = if top_piece == 0 {
                '.'
            } else {
                match top_piece {
                    0b0001 => 'S',
                    0b0101 => 'P',
                    0b1001 => 'R',
                    0b1101 => 'W',
                    0b0011 => 's',
                    0b0111 => 'p',
                    0b1011 => 'r',
                    0b1111 => 'w',
                    _ => '?',
                }
            };
            let char2: char = if bottom_piece == 0 {
                '-'
            } else {
                match bottom_piece {
                    0b0001 => 'S',
                    0b0101 => 'P',
                    0b1001 => 'R',
                    0b1101 => 'W',
                    0b0011 => 's',
                    0b0111 => 'p',
                    0b1011 => 'r',
                    0b1111 => 'w',
                    _ => '?',
                }
            };
            print!("{char1}{char2} ");

            if [5, 12, 18, 25, 31, 38, 44].contains(&i) {
                println!();
                if [12, 25, 38].contains(&i) {
                    print!(" ");
                }
            }
        }
    }
}