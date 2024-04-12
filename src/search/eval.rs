use crate::logic::lookup::PIECE_TO_INDEX;
use crate::logic::{
    COLOUR_MASK, HALF_PIECE_WIDTH, INDEX_MASK, INDEX_WIDTH, TOP_MASK, TYPE_MASK, TYPE_WISE,
};
use crate::search::lookup::PIECE_SCORES;

pub const MAX_SCORE: i64 = 524288;

#[inline]
pub fn evaluate_piece(piece: u8, index: usize) -> i64 {
    PIECE_SCORES[PIECE_TO_INDEX[piece as usize] * 45 + index]
}

#[inline]
pub fn evaluate_action_terminal(
    cells: &[u8; 45],
    action: u64,
    current_player: u8,
    previous_score: i64,
    previous_piece_scores: &[i64; 45],
) -> i64 {
    let index_start: usize = (action & INDEX_MASK) as usize;
    let index_mid: usize = ((action >> INDEX_WIDTH) & INDEX_MASK) as usize;
    let index_end: usize = ((action >> (2 * INDEX_WIDTH)) & INDEX_MASK) as usize;

    let mut current_score = previous_score;

    if (cells[index_start] & TYPE_MASK) != TYPE_WISE
        && ((current_player == 1 && (index_end <= 5)) || (current_player == 0 && (index_end >= 39)))
    {
        return -MAX_SCORE;
    }

    if index_mid > 44 {
        // Starting cell
        current_score -= previous_piece_scores[index_start];

        // Ending cell
        current_score -= previous_piece_scores[index_end];
        current_score += evaluate_piece(cells[index_start], index_end);
    } else {
        let mut start_piece: u8 = cells[index_start];
        let mut mid_piece: u8 = cells[index_mid];
        let mut end_piece: u8 = cells[index_end];
        // The piece at the mid coordinates is an ally : stack and action
        if mid_piece != 0
            && (mid_piece & COLOUR_MASK) == (start_piece & COLOUR_MASK)
            && (index_mid != index_start)
        {
            end_piece = (start_piece & TOP_MASK) + (mid_piece << HALF_PIECE_WIDTH);
            start_piece >>= HALF_PIECE_WIDTH;
            mid_piece = 0;

            // Starting cell
            current_score -= previous_piece_scores[index_start];
            current_score += evaluate_piece(start_piece, index_start);

            // Middle cell
            current_score -= previous_piece_scores[index_mid];
            current_score += evaluate_piece(mid_piece, index_mid);

            // Ending cell
            if index_start != index_end {
                current_score -= previous_piece_scores[index_end];
            }
            current_score += evaluate_piece(end_piece, index_end);
        }
        // The piece at the end coordinates is an ally : action and stack
        else if end_piece != 0 && (end_piece & COLOUR_MASK) == (start_piece & COLOUR_MASK) {
            mid_piece = start_piece;
            start_piece = 0;
            end_piece = (mid_piece & TOP_MASK) + (end_piece << HALF_PIECE_WIDTH);
            if index_start == index_end {
                end_piece = mid_piece & 15;
            }
            mid_piece >>= HALF_PIECE_WIDTH;

            // Starting cell
            if index_start != index_mid {
                current_score -= previous_piece_scores[index_start];
            }

            // Middle cell
            current_score -= previous_piece_scores[index_mid];
            current_score += evaluate_piece(mid_piece, index_mid);

            // Ending cell
            if index_start != index_end {
                current_score -= previous_piece_scores[index_end];
            }
            current_score += evaluate_piece(end_piece, index_end);
        }
        // The end coordinates contain an enemy or no piece : action and unstack
        else {
            mid_piece = start_piece;
            start_piece = 0;
            end_piece = mid_piece & TOP_MASK;
            mid_piece >>= HALF_PIECE_WIDTH;

            // Starting cell
            if index_start != index_mid {
                current_score -= previous_piece_scores[index_start];
            }

            // Middle cell
            current_score -= previous_piece_scores[index_mid];
            current_score += evaluate_piece(mid_piece, index_mid);

            // Ending cell
            current_score -= previous_piece_scores[index_end];
            current_score += evaluate_piece(end_piece, index_end);
        }
    }

    if current_player == 0 {
        current_score
    } else {
        -current_score
    }
}
