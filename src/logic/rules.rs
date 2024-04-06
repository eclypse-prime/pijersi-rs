use super::{COLOUR_MASK, TYPE_MASK};

pub fn can_take(attacker: u8, target: u8) -> bool {
    let attacker_type: u8 = attacker & TYPE_MASK;
    let target_type: u8 = target & TYPE_MASK;
    (attacker_type == 0 && target_type == 4)
        || (attacker_type == 4 && target_type == 8)
        || (attacker_type == 8 && target_type == 0)
}

pub fn can_move1(cells: [u8; 45], moving_piece: u8, index_end: usize) -> bool {
    if cells[index_end] != 0 {
        // If the end piece and the moving piece are the same colour
        if (cells[index_end] & COLOUR_MASK) == (moving_piece & COLOUR_MASK) {
            return false;
        }
        if !can_take(moving_piece, cells[index_end]) {
            return false;
        }
    }
    return true;
}

pub fn can_move2(cells: [u8; 45], moving_piece: u8, index_start: usize, index_end: usize) -> bool {
    // If there is a piece blocking the move (cell between the start and end positions)
    if cells[(index_end + index_start) / 2] != 0 {
        return false;
    }
    if cells[index_end] != 0 {
        // If the end piece and the moving piece are the same colour
        if (cells[index_end] & COLOUR_MASK) == (moving_piece & COLOUR_MASK) {
            return false;
        }
    }
    return true;
}

pub fn can_stack(cells: [u8; 45], moving_piece: u8, index_end: usize) -> bool
{
    // If the end cell is not empty
    // If the target piece and the moving piece are the same colour
    // If the end piece is not a stack
    if (cells[index_end] != 0) && ((cells[index_end] & 2) == (moving_piece & 2)) && (cells[index_end] < 16)
    {
        // If the upper piece is Wise and the target piece is not Wise
        if (moving_piece & 12) == 12 && (cells[index_end] & 12) != 12
        {
            return false;
        }
        return true;
    }

    return false;
}

pub fn can_unstack(cells: [u8; 45], moving_piece: u8, index_end: usize) -> bool {
    if cells[index_end] != 0
    {
        // If the cells are the same colour
        if (cells[index_end] & COLOUR_MASK) == (moving_piece & COLOUR_MASK)
        {
            return false;
        }
        if !can_take(moving_piece, cells[index_end])
        {
            return false;
        }
    }
    return true;
}