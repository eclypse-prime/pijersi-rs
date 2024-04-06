use regex::Regex;

use super::{movegen::concatenate_action, CELL_EMPTY, COLOUR_MASK, INDEX_NULL};

pub fn coords_to_index(i: usize, j: usize) -> usize {
    if i % 2 == 0 {
        13 * i / 2 + j
    } else {
        6 + 13 * (i - 1) / 2 + j
    }
}

pub fn string_to_index(cell_string: &str) -> usize {
    let char_i: char = cell_string.chars().nth(0).unwrap();
    let char_j: char = cell_string.chars().nth(1).unwrap();
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

pub fn string_to_action(action_string: &str, cells: &[u8; 45]) -> u64 {
    let action_pattern = Regex::new(r"(\w\d)(\w\d)?(\w\d)").unwrap();

    let Some(action_captures) = action_pattern.captures(action_string) else {
        panic!("Unknown action string '{action_string}'")
    };

    let index_start: usize = action_captures.get(1).map_or_else(
        || INDEX_NULL,
        |cell_match| string_to_index(cell_match.as_str()),
    );
    let mut index_mid: usize = action_captures.get(2).map_or_else(
        || INDEX_NULL,
        |cell_match| string_to_index(cell_match.as_str()));
    let index_end: usize = action_captures.get(3).map_or_else(
        || INDEX_NULL,
        |cell_match| string_to_index(cell_match.as_str()));
    
    if cells[index_end] != CELL_EMPTY && (cells[index_start] & COLOUR_MASK == cells[index_end] & COLOUR_MASK) && index_mid == INDEX_NULL {
        index_mid = index_start;
    }
    if cells[index_mid] == cells[index_end] {
        index_mid = INDEX_NULL;
    }

    concatenate_action(index_start, index_mid, index_end)
}
