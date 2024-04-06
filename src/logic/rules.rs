use super::{
    CELL_EMPTY, COLOUR_MASK, STACK_TRESHOLD, TYPE_MASK, TYPE_PAPER, TYPE_ROCK, TYPE_SCISSORS,
    TYPE_WISE,
};

pub fn can_take(attacker: u8, target: u8) -> bool {
    let attacker_type: u8 = attacker & TYPE_MASK;
    let target_type: u8 = target & TYPE_MASK;
    (attacker_type == TYPE_SCISSORS && target_type == TYPE_PAPER)
        || (attacker_type == TYPE_PAPER && target_type == TYPE_ROCK)
        || (attacker_type == TYPE_ROCK && target_type == TYPE_SCISSORS)
}

pub fn can_move1(cells: &[u8; 45], piece_start: u8, index_end: usize) -> bool {
    if cells[index_end] != CELL_EMPTY {
        // If the end piece and the moving piece are the same colour
        if (cells[index_end] & COLOUR_MASK) == (piece_start & COLOUR_MASK) {
            return false;
        }
        if !can_take(piece_start, cells[index_end]) {
            return false;
        }
    }
    true
}

pub fn can_move2(cells: &[u8; 45], piece_start: u8, index_start: usize, index_end: usize) -> bool {
    // If there is a piece blocking the move (cell between the start and end positions)
    if cells[(index_end + index_start) / 2] != 0 {
        return false;
    }
    if cells[index_end] != CELL_EMPTY {
        // If the end piece and the moving piece are the same colour
        if (cells[index_end] & COLOUR_MASK) == (piece_start & COLOUR_MASK) {
            return false;
        }
    }
    true
}

pub fn can_stack(cells: &[u8; 45], piece_start: u8, index_end: usize) -> bool {
    // If the end cell is not empty
    // If the target piece and the moving piece are the same colour
    // If the end piece is not a stack
    if (cells[index_end] != CELL_EMPTY)
        && ((cells[index_end] & COLOUR_MASK) == (piece_start & COLOUR_MASK))
        && (cells[index_end] < STACK_TRESHOLD)
    {
        // If the upper piece is Wise and the target piece is not Wise
        if (piece_start & TYPE_MASK) == TYPE_WISE && (cells[index_end] & TYPE_MASK) != TYPE_WISE {
            return false;
        }
        return true;
    }

    false
}

pub fn can_unstack(cells: &[u8; 45], piece_start: u8, index_end: usize) -> bool {
    if cells[index_end] != CELL_EMPTY {
        // If the cells are the same colour
        if (cells[index_end] & COLOUR_MASK) == (piece_start & COLOUR_MASK) {
            return false;
        }
        if !can_take(piece_start, cells[index_end]) {
            return false;
        }
    }
    true
}
