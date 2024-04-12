use regex::Regex;

use super::{
    movegen::concatenate_action, CELL_EMPTY, COLOUR_MASK, INDEX_MASK, INDEX_NULL, INDEX_WIDTH,
    STACK_THRESHOLD,
};

const ROW_LETTERS: [char; 7] = ['g', 'f', 'e', 'd', 'c', 'b', 'a'];

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
pub fn string_to_action(cells: &[u8; 45], action_string: &str) -> u64 {
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

    concatenate_action(index_start, index_mid, index_end)
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
