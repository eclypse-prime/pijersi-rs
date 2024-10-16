//! Implements translation methods to convert the internal representation into a human-readable representation and vice versa.

use regex::Regex;

use crate::{
    errors::{
        InvalidCoordinatesKind, InvalidPlayerKind, InvalidPositionKind, ParseError, ParseErrorKind,
    },
    logic::actions::Action,
    piece::{
        Piece, BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, CELL_EMPTY, WHITE_PAPER,
        WHITE_ROCK, WHITE_SCISSORS, WHITE_WISE,
    },
};

use super::{actions::Index, INDEX_NULL};

const ROW_LETTERS: [char; 7] = ['g', 'f', 'e', 'd', 'c', 'b', 'a'];

/// Converts a character to its corresponding piece (if it exists).
pub fn char_to_piece(piece_char: char) -> Option<u8> {
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
pub fn piece_to_char(piece: u8) -> Option<char> {
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
pub fn coords_to_index(i: usize, j: usize) -> usize {
    if i % 2 == 0 {
        13 * i / 2 + j
    } else {
        6 + 13 * (i - 1) / 2 + j
    }
}

/// Converts an index to a (i, j) coordinate set.
pub fn index_to_coords(index: usize) -> (usize, usize) {
    let mut i: usize = 2 * (index / 13);
    let mut j: usize = index % 13;

    if j > 5 {
        j -= 6;
        i += 1;
    }
    (i, j)
}

/// Converts a "a1" style string coordinate into an index.
fn string_to_index(cell_string: &str) -> Result<usize, ParseError> {
    let mut iterator = cell_string.chars();

    // Guaranteed to match regex "\w\d", no handling needed.
    let char_i: char = iterator.next().unwrap();
    let char_j: char = iterator.next().unwrap();
    let i: usize = match char_i {
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
    let j: usize = match char_j {
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
pub fn index_to_string(index: usize) -> String {
    let (i, j): (usize, usize) = index_to_coords(index);

    ROW_LETTERS[i].to_string() + &(j + 1).to_string()
}

/// Converts a string (a1b1c1 style) move to the native triple-index format.
pub fn string_to_action(cells: &[u8; 45], action_string: &str) -> Result<u64, ParseError> {
    let action_pattern = Regex::new(r"^(\w\d)(\w\d)?(\w\d)$").unwrap();

    let action_captures = action_pattern.captures(action_string).ok_or(ParseError {
        kind: ParseErrorKind::InvalidAction,
        value: action_string.to_owned(),
    })?;

    // Guaranteed to match regex "\w\d", no handling needed.
    let index_start: usize = string_to_index(action_captures.get(1).unwrap().as_str())?;
    let mut index_mid: usize = if let Some(action_capture) = action_captures.get(2) {
        string_to_index(action_capture.as_str())?
    } else {
        INDEX_NULL
    };
    // Guaranteed to match regex "\w\d", no handling needed.
    let index_end: usize = string_to_index(action_captures.get(3).unwrap().as_str())?;

    if !cells[index_end].is_empty()
        && cells[index_start].colour() == cells[index_end].colour()
        && index_mid.is_null()
    {
        index_mid = index_start;
    }
    if index_mid == index_end {
        index_mid = INDEX_NULL;
    }

    Ok(u64::from_indices(index_start, index_mid, index_end))
}

/// Converts a native triple-index move into the string (a1b1c1 style) format.
pub fn action_to_string(cells: &[u8; 45], action: u64) -> String {
    let (index_start, index_mid, index_end) = action.to_indices();

    if index_start.is_null() {
        return String::new();
    }

    let action_string_start: String = index_to_string(index_start);
    let action_string_end: String = index_to_string(index_end);

    let action_string_mid: String = if index_mid.is_null() {
        if cells[index_start].is_stack() {
            index_to_string(index_end)
        } else {
            String::new()
        }
    } else if !index_mid.is_null() && index_start == index_mid && !cells[index_start].is_stack() {
        String::new()
    } else {
        index_to_string(index_mid)
    };

    format!("{action_string_start}{action_string_mid}{action_string_end}")
}

/// Reads a Pijersi Standard Notation string to apply its state to the cells.
pub fn string_to_cells(cells_string: &str) -> Result<[u8; 45], ParseError> {
    let cell_lines: Vec<&str> = cells_string.split('/').collect();
    if cell_lines.len() != 7 {
        Err(ParseError {
            kind: ParseErrorKind::InvalidPosition(InvalidPositionKind::WrongLineNumber(
                cell_lines.len(),
            )),
            value: cells_string.to_owned(),
        })
    } else {
        let mut cursor: usize = 0;
        let mut new_cells: [u8; 45] = [0; 45];
        for &cell_line in &cell_lines {
            let mut j: usize = 0;
            while j < cell_line.chars().count() {
                match char_to_piece(cell_line.chars().nth(j).unwrap()) {
                    Some(top_char) => {
                        if cell_line.chars().nth(j + 1).unwrap() != '-' {
                            new_cells[cursor] =
                                char_to_piece(cell_line.chars().nth(j + 1).unwrap())
                                    .unwrap()
                                    .stack_on(top_char);
                        } else {
                            new_cells[cursor] =
                                char_to_piece(cell_line.chars().nth(j).unwrap()).unwrap();
                        }
                        j += 2;
                        cursor += 1;
                    }
                    None => {
                        let jump = cell_line.chars().nth(j).unwrap().to_digit(10).unwrap() as usize;
                        j += 1;
                        cursor += jump;
                    }
                }
            }
        }
        Ok(new_cells)
    }
}

/// Converts the cells state to a Pijersi Standard Notation string.
pub fn cells_to_string(cells: &[u8; 45]) -> String {
    let mut cells_string = String::new();
    for i in 0..7usize {
        let n_columns: usize = if i % 2 == 0 { 6 } else { 7 };
        let mut counter: usize = 0;
        for j in 0..n_columns {
            let piece = cells[coords_to_index(i, j)];
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

/// Converts the cells to a pretty-formatted string.
pub fn cells_to_pretty_string(cells: &[u8; 45]) -> String {
    let mut cells_pretty_print: String = " ".to_owned();
    for (i, &piece) in cells.iter().enumerate() {
        let top_piece: u8 = piece.top();
        let bottom_piece: u8 = piece.bottom();
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
        cells_pretty_print += &format!("{char1}{char2} ");

        if [5, 12, 18, 25, 31, 38].contains(&i) {
            cells_pretty_print += "\n";
            if [12, 25, 38].contains(&i) {
                cells_pretty_print += " ";
            }
        }
    }
    cells_pretty_print
}

/// Parses the player argument: "w" -> Ok(0u8), "b" -> Ok(1u8)
pub fn string_to_player(player: &str) -> Result<u8, ParseError> {
    match player {
        "w" => Ok(0u8),
        "b" => Ok(1u8),
        _ => Err(ParseError {
            kind: ParseErrorKind::InvalidPlayer(InvalidPlayerKind::StrToPlayer(player.to_owned())),
            value: player.to_owned(),
        }),
    }
}

/// Converts the current player to its Pijersi Standard Notation form: 0 -> Ok("w".to_owned()), 1 -> Ok("b".to_owned())
pub fn player_to_string(current_player: u8) -> Result<String, ParseError> {
    match current_player {
        0u8 => Ok("w".to_owned()),
        1u8 => Ok("b".to_owned()),
        _ => Err(ParseError {
            kind: ParseErrorKind::InvalidPlayer(InvalidPlayerKind::PlayerToStr(current_player)),
            value: current_player.to_string(),
        }),
    }
}
