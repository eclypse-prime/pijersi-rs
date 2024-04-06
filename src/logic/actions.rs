use super::{HALF_PIECE_WIDTH, TOP_MASK};

pub fn do_move(index_start: usize, index_end: usize, cells: &mut [u8; 45]) {
    if index_start != index_end {
        // Move the piece to the target cell
        cells[index_end] = cells[index_start];

        // Set the starting cell as empty
        cells[index_start] = 0u8;
    }
}

pub fn do_stack(index_start: usize, index_end: usize, cells: &mut [u8; 45]) {
    let piece_start: u8 = cells[index_start];
    let piece_end: u8 = cells[index_end];

    // If the moving piece is already on top of a stack, leave the bottom piece in the starting cell
    cells[index_start] = piece_start >> HALF_PIECE_WIDTH;

    // Move the top piece to the target cell and set its new bottom piece
    cells[index_end] = (piece_start & TOP_MASK) + (piece_end << HALF_PIECE_WIDTH);
}

pub fn do_unstack(index_start: usize, index_end: usize, cells: &mut [u8; 45]) {
    let piece_start: u8 = cells[index_start];

    // Leave the bottom piece in the starting cell
    cells[index_start] = piece_start >> HALF_PIECE_WIDTH;
 
    // Remove the bottom piece from the moving piece
    // Move the top piece to the target cell
    // Will overwrite the eaten piece if there is one
    cells[index_end] = piece_start & TOP_MASK;
}
