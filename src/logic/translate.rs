use regex::Regex;

use crate::errors::{IllegalActionError, StringParseError};

use super::{
    movegen::concatenate_action, CELL_EMPTY, COLOUR_MASK, HALF_PIECE_WIDTH, INDEX_MASK, INDEX_NULL,
    INDEX_WIDTH, STACK_THRESHOLD,
};

const ROW_LETTERS: [char; 7] = ['g', 'f', 'e', 'd', 'c', 'b', 'a'];

pub fn char_to_piece(piece_char: char) -> Option<u8> {
    match piece_char {
        '-' => Some(0x00),
        'S' => Some(0x01),
        'P' => Some(0x05),
        'R' => Some(0x09),
        'W' => Some(0x0D),
        's' => Some(0x03),
        'p' => Some(0x07),
        'r' => Some(0x0B),
        'w' => Some(0x0F),
        _ => None,
    }
}

pub fn piece_to_char(piece: u8) -> Option<char> {
    match piece {
        0x00 => Some('-'),
        0x01 => Some('S'),
        0x05 => Some('P'),
        0x09 => Some('R'),
        0x0D => Some('W'),
        0x03 => Some('s'),
        0x07 => Some('p'),
        0x0B => Some('r'),
        0x0F => Some('w'),
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
pub fn string_to_index(cell_string: &str) -> usize {
    let mut iterator = cell_string.chars();
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
            panic!("Unknown vertical coordinate '{char_i}' of '{cell_string}'.")
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
            panic!("Unknown horizontal coordinate '{char_j}' of '{cell_string}'")
        }
    };
    coords_to_index(i, j)
}

/// Converts a native index into a "a1" style string.
pub fn index_to_string(index: usize) -> String {
    let (i, j): (usize, usize) = index_to_coords(index);

    ROW_LETTERS[i].to_string() + &(j + 1).to_string()
}

/// Converts a string (a1b1c1 style) move to the native triple-index format.
pub fn string_to_action(cells: &[u8; 45], action_string: &str) -> Result<u64, StringParseError> {
    let action_pattern = Regex::new(r"(\w\d)(\w\d)?(\w\d)").unwrap();

    let Some(action_captures) = action_pattern.captures(action_string) else {
        return Err(StringParseError::new(&format!(
            "Unknown action string '{}'",
            action_string
        )));
    };

    let index_start: usize = action_captures.get(1).map_or_else(
        || INDEX_NULL,
        |cell_match| string_to_index(cell_match.as_str()),
    );
    let mut index_mid: usize = action_captures.get(2).map_or_else(
        || INDEX_NULL,
        |cell_match| string_to_index(cell_match.as_str()),
    );
    let index_end: usize = action_captures.get(3).map_or_else(
        || INDEX_NULL,
        |cell_match| string_to_index(cell_match.as_str()),
    );

    if cells[index_end] != CELL_EMPTY
        && (cells[index_start] & COLOUR_MASK == cells[index_end] & COLOUR_MASK)
        && index_mid == INDEX_NULL
    {
        index_mid = index_start;
    }
    if cells[index_mid] == cells[index_end] {
        index_mid = INDEX_NULL;
    }

    Ok(concatenate_action(index_start, index_mid, index_end))
}

/// Converts a native triple-index move into the string (a1b1c1 style) format.
pub fn action_to_string(cells: &[u8; 45], action: u64) -> String {
    let index_start: usize = (action & INDEX_MASK) as usize;
    let index_mid: usize = ((action >> INDEX_WIDTH) & INDEX_MASK) as usize;
    let index_end: usize = ((action >> (2 * INDEX_WIDTH)) & INDEX_MASK) as usize;

    if index_start == INDEX_NULL {
        return "".to_string();
    }

    let action_string_start: String = index_to_string(index_start);
    let action_string_end: String = index_to_string(index_end);

    let action_string_mid: String = if index_mid == INDEX_NULL {
        if cells[index_start] >= STACK_THRESHOLD {
            index_to_string(index_end)
        } else {
            "".to_string()
        }
    } else if index_mid != INDEX_NULL
        && index_start == index_mid
        && cells[index_start] < STACK_THRESHOLD
    {
        "".to_string()
    } else {
        index_to_string(index_mid)
    };

    format!("{action_string_start}{action_string_mid}{action_string_end}")
}

pub fn string_to_cells(cells: &mut [u8; 45], cells_string: &str) -> Result<(), StringParseError> {
    let cell_lines: Vec<&str> = cells_string.split('/').collect();
    if cell_lines.len() != 7 {
        Err(StringParseError::new(&format!(
            "Invalid number of lines in board notation :{} (expected 7)",
            cell_lines.len()
        )))
    } else {
        let mut cursor: usize = 0;
        let mut new_cells: [u8; 45] = [0; 45];
        for i in 0..7 {
            let mut j: usize = 0;
            while j < cell_lines[i].chars().count() {
                if char_to_piece(cell_lines[i].chars().nth(j).unwrap()).is_some() {
                    if cell_lines[i].chars().nth(j + 1).unwrap() != '-' {
                        new_cells[cursor] =
                            char_to_piece(cell_lines[i].chars().nth(j + 1).unwrap()).unwrap()
                                | (char_to_piece(cell_lines[i].chars().nth(j).unwrap()).unwrap()
                                    << HALF_PIECE_WIDTH);
                    } else {
                        new_cells[cursor] =
                            char_to_piece(cell_lines[i].chars().nth(j).unwrap()).unwrap();
                    }
                    j += 2;
                    cursor += 1;
                } else {
                    let jump = cell_lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as usize;
                    j += 1;
                    cursor += jump;
                }
            }
        }
        *cells = new_cells;
        Ok(())
    }
}
