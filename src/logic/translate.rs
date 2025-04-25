//! Implements translation methods to convert the internal representation into a human-readable representation and vice versa.

use std::fmt::Display;

use regex::Regex;

use crate::{
    bitboard::{Bitboard, Board},
    errors::{
        InvalidCoordinatesKind, InvalidPlayerKind, InvalidPositionKind, ParseError, ParseErrorKind,
    },
    logic::actions::ActionTrait,
    piece::{
        Piece, PieceTrait, BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, CELL_EMPTY,
        WHITE_PAPER, WHITE_ROCK, WHITE_SCISSORS, WHITE_WISE,
    },
};

use super::{
    actions::Action,
    index::{CellIndex, CellIndexTrait, INDEX_NULL},
    Player, N_CELLS,
};

const ROW_LETTERS: [char; 7] = ['g', 'f', 'e', 'd', 'c', 'b', 'a'];

/// Converts a character to its corresponding piece (if it exists).
pub const fn char_to_piece(piece_char: char) -> Option<Piece> {
    match piece_char {
        '-' => Some(CELL_EMPTY),
        'S' => Some(WHITE_SCISSORS),
        'P' => Some(WHITE_PAPER),
        'R' => Some(WHITE_ROCK),
        'W' => Some(WHITE_WISE),
        's' => Some(BLACK_SCISSORS),
        'p' => Some(BLACK_PAPER),
        'r' => Some(BLACK_ROCK),
        'w' => Some(BLACK_WISE),
        _ => None,
    }
}

/// Converts a piece to its corresponding character (if it exists).
pub const fn piece_to_char(piece: Piece) -> Option<char> {
    match piece {
        CELL_EMPTY => Some('-'),
        WHITE_SCISSORS => Some('S'),
        WHITE_PAPER => Some('P'),
        WHITE_ROCK => Some('R'),
        WHITE_WISE => Some('W'),
        BLACK_SCISSORS => Some('s'),
        BLACK_PAPER => Some('p'),
        BLACK_ROCK => Some('r'),
        BLACK_WISE => Some('w'),
        _ => None,
    }
}

/// Converts a (i, j) coordinate set to an index.
pub const fn coords_to_index(i: CellIndex, j: CellIndex) -> CellIndex {
    if i % 2 == 0 {
        13 * i / 2 + j
    } else {
        6 + 13 * (i - 1) / 2 + j
    }
}

/// Converts an index to a (i, j) coordinate set.
pub const fn index_to_coords(index: CellIndex) -> (CellIndex, CellIndex) {
    let mut i: CellIndex = 2 * (index / 13);
    let mut j: CellIndex = index % 13;

    if j > 5 {
        j -= 6;
        i += 1;
    }
    (i, j)
}

/// Converts a "a1" style string coordinate into an index.
fn string_to_index(cell_string: &str) -> Result<CellIndex, ParseError> {
    let mut iterator = cell_string.chars();

    // Guaranteed to match regex "\w\d", no handling needed.
    let char_i: char = iterator.next().unwrap();
    let char_j: char = iterator.next().unwrap();
    let i: CellIndex = match char_i {
        'a' => 6,
        'b' => 5,
        'c' => 4,
        'd' => 3,
        'e' => 2,
        'f' => 1,
        'g' => 0,
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidCoordinates {
                    kind: InvalidCoordinatesKind::Vertical,
                    value: char_i,
                },
                value: cell_string.to_owned(),
            })
        }
    };
    let j: CellIndex = match char_j {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidCoordinates {
                    kind: InvalidCoordinatesKind::Horizontal,
                    value: char_j,
                },
                value: cell_string.to_owned(),
            })
        }
    };
    Ok(coords_to_index(i, j))
}

/// Converts a native index into a "a1" style string.
pub fn index_to_string(index: CellIndex) -> String {
    let (i, j): (CellIndex, CellIndex) = index_to_coords(index);

    ROW_LETTERS[i].to_string() + &(j + 1).to_string()
}

impl Bitboard {
    /// Converts the bitboard to a pretty formatted string.
    ///
    /// It shows how the bitboard looks on a Pijersi board.
    pub fn to_pretty_string(&self) -> String {
        let mut pretty_string = " ".to_owned();
        for i in 0..45 {
            pretty_string += if self.get(i) { "X  " } else { ".  " };
            if [5, 12, 18, 25, 31, 38].contains(&i) {
                pretty_string += "\n";
                if [12, 25, 38].contains(&i) {
                    pretty_string += " ";
                }
            }
        }

        pretty_string
    }
}

impl Board {
    /// Converts the board to a Pijersi Standard Notation string.
    pub fn to_fen(&self) -> String {
        let mut cells_string = String::new();
        for i in 0..7usize {
            let n_columns: usize = if i % 2 == 0 { 6 } else { 7 };
            let mut counter: usize = 0;
            for j in 0..n_columns {
                let piece = self.get_piece(coords_to_index(i, j));
                if piece.is_empty() {
                    counter += 1;
                } else {
                    if counter > 0 {
                        cells_string += &counter.to_string();
                        counter = 0;
                    }
                    if piece.is_stack() {
                        cells_string += &piece_to_char(piece.bottom()).unwrap().to_string();
                        cells_string += &piece_to_char(piece.top()).unwrap().to_string();
                    } else {
                        cells_string += &piece_to_char(piece).unwrap().to_string();
                        cells_string += "-";
                    }
                }
            }
            if counter > 0 {
                cells_string += &counter.to_string();
            }
            if i < 6 {
                cells_string += "/";
            }
        }
        cells_string
    }

    /// Reads a Pijersi Standard Notation string to apply its state to the board.
    pub fn try_from_fen(board_string: &str) -> Result<Self, ParseError> {
        let cell_lines: Vec<&str> = board_string.split('/').collect();
        if cell_lines.len() == 7 {
            let mut cursor: CellIndex = 0;
            let mut new_board = Board::EMPTY;
            for &cell_line in &cell_lines {
                let mut j: usize = 0;
                while j < cell_line.chars().count() {
                    if let Some(top_char) = char_to_piece(cell_line.chars().nth(j).unwrap()) {
                        if cell_line.chars().nth(j + 1).unwrap() == '-' {
                            new_board.set_piece(
                                cursor,
                                char_to_piece(cell_line.chars().nth(j).unwrap()).unwrap(),
                            );
                        } else {
                            new_board.set_piece(
                                cursor,
                                char_to_piece(cell_line.chars().nth(j + 1).unwrap())
                                    .unwrap()
                                    .stack_on(top_char),
                            );
                        }
                        j += 2;
                        cursor += 1;
                    } else {
                        let jump =
                            cell_line.chars().nth(j).unwrap().to_digit(10).unwrap() as CellIndex;
                        j += 1;
                        cursor += jump;
                    }
                }
            }
            Ok(new_board)
        } else {
            Err(ParseError {
                kind: ParseErrorKind::InvalidPosition(InvalidPositionKind::WrongLineNumber(
                    cell_lines.len(),
                )),
                value: board_string.to_owned(),
            })
        }
    }

    /// Converts the cells to a pretty formatted string.
    ///
    /// The starting position is represented as such:
    /// ```not_rust
    ///  s- p- r- s- p- r-
    /// p- r- s- ww r- s- p-
    ///  .  .  .  .  .  .  
    /// .  .  .  .  .  .  .  
    ///  .  .  .  .  .  .  
    /// P- S- R- WW S- R- P-
    ///  R- P- S- R- P- S-
    /// ```
    pub fn to_pretty_string(&self) -> String {
        let mut pretty_string = " ".to_owned();
        for i in 0..N_CELLS {
            let piece = self.get_piece(i);
            let top_piece: Piece = piece.top();
            let bottom_piece: Piece = piece.bottom();
            let char1: char = match top_piece {
                0b0000 => '.',
                0b1000 => 'S',
                0b1001 => 'P',
                0b1010 => 'R',
                0b1011 => 'W',
                0b1100 => 's',
                0b1101 => 'p',
                0b1110 => 'r',
                0b1111 => 'w',
                _ => '?',
            };
            let char2: char = if top_piece == 0 {
                ' '
            } else {
                match bottom_piece {
                    0b0000 => '-',
                    0b1000 => 'S',
                    0b1001 => 'P',
                    0b1010 => 'R',
                    0b1011 => 'W',
                    0b1100 => 's',
                    0b1101 => 'p',
                    0b1110 => 'r',
                    0b1111 => 'w',
                    _ => '?',
                }
            };
            pretty_string += &format!("{char1}{char2} ");

            if [5, 12, 18, 25, 31, 38].contains(&i) {
                pretty_string += "\n";
                if [12, 25, 38].contains(&i) {
                    pretty_string += " ";
                }
            }
        }

        pretty_string
    }
}

// NOTE: The least significant bit (LSB) is at the right of the binary number and represents the top-left cell
impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:045b}", self.0))
    }
}

// This also implements ToString/to_string()
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cells_string = String::new();
        for index in 0..N_CELLS {
            let piece = self.get_piece(index);
            if piece.is_empty() {
                cells_string += ".."
            } else {
                cells_string += &piece_to_char(piece.top()).unwrap().to_string();
                if piece.is_stack() {
                    cells_string += &piece_to_char(piece.bottom()).unwrap().to_string();
                } else {
                    cells_string += "-";
                }
            }
        }
        f.write_str(&cells_string)
    }
}

impl TryFrom<&str> for Board {
    type Error = ParseError;
    fn try_from(board_string: &str) -> Result<Self, Self::Error> {
        if board_string.chars().count() != 2 * N_CELLS {
            Err(ParseError {
                kind: ParseErrorKind::InvalidPosition(InvalidPositionKind::WrongCharNumber(
                    board_string.len(),
                )),
                value: board_string.to_owned(),
            })
        } else {
            let mut new_board = Board::EMPTY;
            let board_chars: Vec<char> = board_string.chars().collect();
            for (index, piece_chars) in board_chars.chunks_exact(2).enumerate() {
                let &[top_char, bottom_char] = piece_chars.try_into().unwrap();
                if top_char != '.' {
                    if bottom_char == '-' {
                        new_board.set_piece(index, char_to_piece(top_char).unwrap());
                    } else {
                        new_board.set_piece(
                            index,
                            char_to_piece(top_char)
                                .unwrap()
                                .stack_on(char_to_piece(bottom_char).unwrap()),
                        );
                    }
                }
            }
            Ok(new_board)
        }
    }
}

/// Converts a string (a1b1c1 style) move to the native triple-index format.
pub fn string_to_action(board: &Board, action_string: &str) -> Result<Action, ParseError> {
    let action_pattern = Regex::new(r"^(\w\d)(\w\d)?(\w\d)$").unwrap();

    let action_captures = action_pattern.captures(action_string).ok_or(ParseError {
        kind: ParseErrorKind::InvalidAction,
        value: action_string.to_owned(),
    })?;

    // Guaranteed to match regex "\w\d", no handling needed.
    let index_start: CellIndex = string_to_index(action_captures.get(1).unwrap().as_str())?;
    let mut index_mid: CellIndex = if let Some(action_capture) = action_captures.get(2) {
        string_to_index(action_capture.as_str())?
    } else {
        INDEX_NULL
    };
    // Guaranteed to match regex "\w\d", no handling needed.
    let index_end: CellIndex = string_to_index(action_captures.get(3).unwrap().as_str())?;

    if !board.get_piece(index_end).is_empty()
        && board.get_piece(index_start).colour() == board.get_piece(index_end).colour()
        && index_mid.is_null()
    {
        index_mid = index_start;
    }
    if index_mid == index_end {
        index_mid = INDEX_NULL;
    }

    Ok(Action::from_indices(index_start, index_mid, index_end))
}

/// Converts a native triple-index move into the string (a1b1c1 style) format.
pub fn action_to_string(board: &Board, action: Action) -> String {
    let (index_start, index_mid, index_end) = action.to_indices();

    if index_start.is_null() {
        return String::new();
    }

    let action_string_start: String = index_to_string(index_start);
    let action_string_end: String = index_to_string(index_end);

    let action_string_mid: String = if index_mid.is_null() {
        if board.get_piece(index_start).is_stack() {
            index_to_string(index_end)
        } else {
            String::new()
        }
    } else if !index_mid.is_null()
        && index_start == index_mid
        && !board.get_piece(index_start).is_stack()
    {
        String::new()
    } else {
        index_to_string(index_mid)
    };

    format!("{action_string_start}{action_string_mid}{action_string_end}")
}

/// Parses the player argument: `"w"` -> `Ok(0)`, `"b"` -> `Ok(1)`
pub fn string_to_player(player: &str) -> Result<Player, ParseError> {
    match player {
        "w" => Ok(0),
        "b" => Ok(1),
        _ => Err(ParseError {
            kind: ParseErrorKind::InvalidPlayer(InvalidPlayerKind::StrToPlayer(player.to_owned())),
            value: player.to_owned(),
        }),
    }
}

/// Converts the current player to its Pijersi Standard Notation form: `0` -> `Ok("w".to_owned())`, `1` -> `Ok("b".to_owned())`
pub fn player_to_string(current_player: Player) -> Result<String, ParseError> {
    match current_player {
        0 => Ok("w".to_owned()),
        1 => Ok("b".to_owned()),
        _ => Err(ParseError {
            kind: ParseErrorKind::InvalidPlayer(InvalidPlayerKind::PlayerToStr(current_player)),
            value: current_player.to_string(),
        }),
    }
}
