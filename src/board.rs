use crate::logic::actions::play_action;
use crate::logic::translate::string_to_action;
use crate::piece::{init_piece, PieceColour, PieceType};
use crate::search::alphabeta::search_to_depth;

/// This struct represents a Pijersi board.
///
/// It contains all the necessary information to represent a Pijersi game at one point:
///     - Current cells
///     - Current player
pub struct Board {
    pub cells: [u8; 45],
    pub current_player: u8,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /// Board constructor: the cells are empty on initialization, the current player is white.
    pub fn new() -> Self {
        Self {
            cells: [0u8; 45],
            current_player: 0u8,
        }
    }

    /// Initializes the the board to the starting configuration.
    ///
    /// Sets the pieces to their original position and the current player to white.
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

    /// Prints the current pieces on the board.
    pub fn print(&self) {
        print!(" ");
        for i in 0..45 {
            let piece: u8 = self.cells[i];
            let top_piece: u8 = piece & 0b1111;
            let bottom_piece: u8 = piece >> 4;
            let char1: char = match top_piece {
                0b0000 => '.',
                0b0001 => 'S',
                0b0101 => 'P',
                0b1001 => 'R',
                0b1101 => 'W',
                0b0011 => 's',
                0b0111 => 'p',
                0b1011 => 'r',
                0b1111 => 'w',
                _ => '?',
            };
            let char2: char = if top_piece == 0 {
                ' '
            } else {
                match bottom_piece {
                    0b0000 => '-',
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

    /// Searches and returns the best action at a given depth
    pub fn search_to_depth(&self, depth: u64) -> Option<u64> {
        search_to_depth(&self.cells, self.current_player, depth)
    }

    /// Plays the chosen action provided in string representation.
    pub fn play_from_string(&mut self, action_string: &str) {
        let action = string_to_action(&self.cells, action_string);
        self.play(action);
    }

    /// Plays the chosen action provided in u64 representation.
    pub fn play(&mut self, action: u64) {
        play_action(&mut self.cells, action);
        self.current_player = 1 - self.current_player;
    }
}
