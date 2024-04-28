use super::{
    movegen::available_player_actions, CELL_EMPTY, COLOUR_BLACK, COLOUR_MASK, COLOUR_WHITE,
    INDEX_MASK, INDEX_WIDTH, MAX_PLAYER_ACTIONS, STACK_THRESHOLD, TYPE_MASK, TYPE_PAPER, TYPE_ROCK,
    TYPE_SCISSORS, TYPE_WISE,
};

/// Returns whether an attacker piece can capture a target piece.
///
/// The capture rules are the sames as rock-paper-scissors.
/// The wise piece can neither capture or be captured.
#[inline]
pub fn can_take(attacker: u8, target: u8) -> bool {
    let attacker_type: u8 = attacker & TYPE_MASK;
    let target_type: u8 = target & TYPE_MASK;
    (attacker_type == TYPE_SCISSORS && target_type == TYPE_PAPER)
        || (attacker_type == TYPE_PAPER && target_type == TYPE_ROCK)
        || (attacker_type == TYPE_ROCK && target_type == TYPE_SCISSORS)
}

/// Returns whether the chosen 1-range move is possible.
#[inline]
pub fn can_move1(cells: &[u8; 45], moving_piece: u8, index_end: usize) -> bool {
    let target_piece: u8 = cells[index_end];

    if target_piece != CELL_EMPTY {
        // If the end piece and the moving piece are the same colour
        if (target_piece & COLOUR_MASK) == (moving_piece & COLOUR_MASK) {
            return false;
        }
        if !can_take(moving_piece, target_piece) {
            return false;
        }
    }
    true
}

/// Returns whether the chosen 2-range move is possible.
#[inline]
pub fn can_move2(cells: &[u8; 45], moving_piece: u8, index_start: usize, index_end: usize) -> bool {
    let target_piece: u8 = cells[index_end];

    // If there is a piece blocking the move (cell between the start and end positions)
    if cells[(index_end + index_start) / 2] != 0 {
        return false;
    }
    if target_piece != CELL_EMPTY {
        // If the end piece and the moving piece are the same colour
        if (target_piece & COLOUR_MASK) == (moving_piece & COLOUR_MASK) {
            return false;
        }
        if !can_take(moving_piece, target_piece) {
            return false;
        }
    }
    true
}

/// Returns whether the chosen stack action is possible.
#[inline]
pub fn can_stack(cells: &[u8; 45], moving_piece: u8, index_end: usize) -> bool {
    let target_piece: u8 = cells[index_end];

    // If the end cell is not empty
    // If the target piece and the moving piece are the same colour
    // If the end piece is not a stack
    if (target_piece != CELL_EMPTY)
        && (target_piece & COLOUR_MASK) == (moving_piece & COLOUR_MASK)
        && (target_piece < STACK_THRESHOLD)
    {
        // If the upper piece is Wise and the target piece is not Wise
        if (moving_piece & TYPE_MASK) == TYPE_WISE && (target_piece & TYPE_MASK) != TYPE_WISE {
            return false;
        }
        return true;
    }

    false
}

/// Returns whether the chosen unstack action is possible.
#[inline]
pub fn can_unstack(cells: &[u8; 45], moving_piece: u8, index_end: usize) -> bool {
    let target_piece: u8 = cells[index_end];

    if target_piece != CELL_EMPTY {
        // If the cells are the same colour
        if (target_piece & COLOUR_MASK) == (moving_piece & COLOUR_MASK) {
            return false;
        }
        if !can_take(moving_piece, target_piece) {
            return false;
        }
    }
    true
}

/// Returns true if the chosen action leads to a win.
///
/// To win, one allied piece (except wise) must reach the last row in the opposite side.
#[inline]
pub fn is_action_win(cells: &[u8; 45], action: u64) -> bool {
    let index_start: usize = (action & INDEX_MASK) as usize;
    let index_end: usize = ((action >> (2 * INDEX_WIDTH)) & INDEX_MASK) as usize;

    let moving_piece: u8 = cells[index_start];

    if (moving_piece & TYPE_MASK) != TYPE_WISE
        && (((moving_piece & COLOUR_MASK) == COLOUR_WHITE && index_end <= 5)
            || ((moving_piece & COLOUR_MASK) == COLOUR_BLACK && index_end >= 39))
    {
        return true;
    }
    false
}

/// Returns true if the given action is legal
pub fn is_action_legal(cells: &[u8; 45], current_player: u8, action: u64) -> bool {
    let available_actions: [u64; 512] = available_player_actions(cells, current_player);
    let n_actions: usize = available_actions[MAX_PLAYER_ACTIONS - 1] as usize;
    available_actions
        .iter()
        .take(n_actions)
        .any(|&available_action| available_action == action)
}

pub fn is_position_win(cells: &[u8; 45]) -> bool {
    for k in 0..6 {
        let piece = cells[k];
        if piece != CELL_EMPTY {
            // If piece is White and not Wise
            if (piece & COLOUR_MASK) == COLOUR_WHITE && (piece & TYPE_MASK) != TYPE_WISE {
                return true;
            }
        }
    }
    for k in 39..45 {
        let piece = cells[k];
        if piece != CELL_EMPTY {
            // If piece is White and not Wise
            if (piece & COLOUR_MASK) == COLOUR_BLACK && (piece & TYPE_MASK) != TYPE_WISE {
                return true;
            }
        }
    }
    false
}

pub fn is_position_stalemate(cells: &[u8; 45], current_player: u8) -> bool {
    let available_actions = available_player_actions(cells, current_player);
    let n_moves = available_actions[MAX_PLAYER_ACTIONS - 1];
    n_moves == 0
}

pub fn get_winning_player(cells: &[u8; 45]) -> Option<u8> {
    for k in 0..6 {
        let piece = cells[k];
        if piece != CELL_EMPTY {
            // If piece is White and not Wise
            if (piece & COLOUR_MASK) == COLOUR_WHITE && (piece & TYPE_MASK) != TYPE_WISE {
                return Some(0);
            }
        }
    }
    for k in 39..45 {
        let piece = cells[k];
        if piece != CELL_EMPTY {
            // If piece is White and not Wise
            if (piece & COLOUR_MASK) == COLOUR_BLACK && (piece & TYPE_MASK) != TYPE_WISE {
                return Some(1);
            }
        }
    }
    None
}
