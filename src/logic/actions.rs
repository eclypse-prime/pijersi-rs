//! Implements the actions a player can choose (move, stack, unstack...).
//!
//! An action is stored as a u64 value. Its contents are divided into the following sections:
//!
//! | Data  | Empty | Depth (optional) | Third index | Second index | First index |
//! |-------|-------|------------------|-------------|--------------|-------------|
//! | Width | 32    | 8                | 8           | 8            | 8           |

use super::{
    translate::action_to_indices, CELL_EMPTY, COLOUR_MASK, HALF_PIECE_WIDTH, INDEX_MASK,
    INDEX_NULL, INDEX_WIDTH, TOP_MASK,
};

/// Applies a move between chosen coordinates.
fn do_move(cells: &mut [u8; 45], index_start: usize, index_end: usize) {
    if index_start != index_end {
        // Move the piece to the target cell
        cells[index_end] = cells[index_start];

        // Set the starting cell as empty
        cells[index_start] = CELL_EMPTY;
    }
}

/// Applies a stack between chosen coordinates.
fn do_stack(cells: &mut [u8; 45], index_start: usize, index_end: usize) {
    let piece_start: u8 = cells[index_start];
    let piece_end: u8 = cells[index_end];

    // If the moving piece is already on top of a stack, leave the bottom piece in the starting cell
    cells[index_start] = piece_start >> HALF_PIECE_WIDTH;

    // Move the top piece to the target cell and set its new bottom piece
    cells[index_end] = (piece_start & TOP_MASK) + (piece_end << HALF_PIECE_WIDTH);
}

/// Applies an unstack between chosen coordinates.
fn do_unstack(cells: &mut [u8; 45], index_start: usize, index_end: usize) {
    let piece_start: u8 = cells[index_start];

    // Leave the bottom piece in the starting cell
    cells[index_start] = piece_start >> HALF_PIECE_WIDTH;

    // Remove the bottom piece from the moving piece
    // Move the top piece to the target cell
    // Will overwrite the eaten piece if there is one
    cells[index_end] = piece_start & TOP_MASK;
}

/// Plays the selected action.
pub fn play_action(cells: &mut [u8; 45], action: u64) {
    let (index_start, index_mid, index_end) = action_to_indices(action);

    if index_start == INDEX_NULL {
        return;
    }

    let piece_start: u8 = cells[index_start];

    if piece_start != CELL_EMPTY {
        // If there is no intermediate move
        if index_mid == INDEX_NULL {
            // Simple move
            do_move(cells, index_start, index_end);
        } else {
            let piece_mid: u8 = cells[index_mid];
            let piece_end: u8 = cells[index_end];
            // The piece at the mid coordinates is an ally : stack and move
            if piece_mid != CELL_EMPTY
                && (piece_mid & COLOUR_MASK) == (piece_start & COLOUR_MASK)
                && (index_start != index_mid)
            {
                do_stack(cells, index_start, index_mid);
                do_move(cells, index_mid, index_end);
            }
            // The piece at the end coordinates is an ally : move and stack
            else if piece_end != CELL_EMPTY
                && (piece_end & COLOUR_MASK) == (piece_start & COLOUR_MASK)
            {
                do_move(cells, index_start, index_mid);
                do_stack(cells, index_mid, index_end);
            }
            // The end coordinates contain an enemy or no piece : move and unstack
            else {
                do_move(cells, index_start, index_mid);
                do_unstack(cells, index_mid, index_end);
            }
        }
    }
}
