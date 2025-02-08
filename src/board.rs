//! This module contains the Board struct and methods to represent a Pijersi board and play games.
//!
//! A board is represented as a `[Piece; 45]` array.
//!
//! Its cells are indexed as such:
//! ```not_rust
//!   0   1   2   3   4   5
//! 6   7   8   9   10  11  12
//!   13  14  15  16  17  18
//! 19  20  21  22  23  24  25
//!   26  27  28  29  30  31
//! 32  33  34  35  36  37  38
//!   39  40  41  42  43  44
//! ```
use std::sync::RwLock;
use std::time::{Duration, Instant};

use crate::errors::{ParseError, ParseErrorKind, RulesErrorKind, RuntimeError};
use crate::hash::search::SearchTable;
use crate::logic::actions::{play_action, Action, ActionTrait};
use crate::logic::rules::{
    get_winning_player, is_action_legal, is_position_stalemate, is_position_win,
};
use crate::logic::translate::{
    action_to_string, cells_to_pretty_string, cells_to_string, player_to_string, string_to_action,
    string_to_cells, string_to_player,
};
use crate::logic::{Cells, Player, CELLS_EMPTY, MAX_HALF_MOVES};
use crate::piece::{
    PieceTrait, BLACK_PAPER, BLACK_ROCK, BLACK_SCISSORS, BLACK_WISE, WHITE_PAPER, WHITE_ROCK,
    WHITE_SCISSORS, WHITE_WISE,
};
use crate::search::alphabeta::search_iterative;
use crate::search::openings::OpeningBook;
use crate::search::Score;

/// This struct represents the board options.
///
/// It contains various parameters for the search engine:
/// * Using the opening book
/// * Printing the info logs during searches
pub struct BoardOptions {
    /// Using the opening book
    pub use_book: bool,
    /// Using the hash table
    pub use_table: bool,
    /// Printing the info logs during searches
    pub verbose: bool,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardOptions {
    /// `BoardOptions` constructor. By default, the options are set to:
    /// ```not_rust
    /// use_book: true
    /// use_table: true
    /// verbose: true
    /// ```
    pub const fn new() -> Self {
        Self {
            use_book: true,
            use_table: true,
            verbose: true,
        }
    }
}

/// This struct represents a Pijersi board.
///
/// It contains all the necessary information to represent a Pijersi game at any point:
/// * Current cells
/// * Current player
/// * Current half moves count
/// * Current full moves count
/// * Piece count
pub struct Board {
    /// The board options
    pub options: BoardOptions,
    /// The current cells storing the piece data as `Piece` (see [`crate::piece`])
    pub cells: Cells,
    /// The current player: 0 if white, 1 if black
    pub current_player: Player,
    half_moves: u64,
    full_moves: u64,
    last_piece_count: u64,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /// Board constructor: the cells are empty on initialization, the current player is white.
    pub fn new() -> Self {
        Self {
            options: BoardOptions::new(),
            cells: CELLS_EMPTY,
            current_player: 0,
            half_moves: 0u64,
            full_moves: 0u64,
            last_piece_count: 0u64,
        }
    }

    /// Initializes the the board to the starting configuration.
    ///
    /// Sets the pieces to their original position and the current player to white.
    ///
    /// Sets the half move counter to 0 and the full move counter to 1.
    pub fn init(&mut self) {
        self.cells.fill(0);

        self.cells[0] = BLACK_SCISSORS;
        self.cells[1] = BLACK_PAPER;
        self.cells[2] = BLACK_ROCK;
        self.cells[3] = BLACK_SCISSORS;
        self.cells[4] = BLACK_PAPER;
        self.cells[5] = BLACK_ROCK;
        self.cells[6] = BLACK_PAPER;
        self.cells[7] = BLACK_ROCK;
        self.cells[8] = BLACK_SCISSORS;
        self.cells[9] = BLACK_WISE.stack_on(BLACK_WISE);
        self.cells[10] = BLACK_ROCK;
        self.cells[11] = BLACK_SCISSORS;
        self.cells[12] = BLACK_PAPER;

        self.cells[44] = WHITE_SCISSORS;
        self.cells[43] = WHITE_PAPER;
        self.cells[42] = WHITE_ROCK;
        self.cells[41] = WHITE_SCISSORS;
        self.cells[40] = WHITE_PAPER;
        self.cells[39] = WHITE_ROCK;
        self.cells[38] = WHITE_PAPER;
        self.cells[37] = WHITE_ROCK;
        self.cells[36] = WHITE_SCISSORS;
        self.cells[35] = WHITE_WISE.stack_on(WHITE_WISE);
        self.cells[34] = WHITE_ROCK;
        self.cells[33] = WHITE_SCISSORS;
        self.cells[32] = WHITE_PAPER;

        self.current_player = 0;
        self.half_moves = 0;
        self.full_moves = 1;
        self.last_piece_count = self.count_pieces(); // 28 starting pieces (14 for each side)
    }

    /// Prints the current pieces on the board.
    pub fn print(&self) {
        println!("{}", cells_to_pretty_string(&self.cells));
    }

    /// Searches and returns the action corresponding to the current board state according to the opening book (if it exists)
    fn search_book(&self, opening_book: Option<&OpeningBook>) -> Option<(Action, u64, Score)> {
        if let Some(opening_book) = opening_book {
            if let Some(&(action, score)) = opening_book.lookup(self) {
                let depth = action.search_depth();
                let action_string = action_to_string(&self.cells, action);
                if self.options.verbose {
                    println!("info book depth {depth} score {score} pv {action_string}");
                }
                return Some((action, depth, score as Score));
            }
        }
        None
    }

    /// Searches and returns the best action at a given depth.
    pub fn search_to_depth(
        &self,
        depth: u64,
        opening_book: Option<&OpeningBook>,
        transposition_table: Option<&RwLock<SearchTable>>,
    ) -> Option<(Action, Score)> {
        if self.options.use_book {
            if let Some((action, book_depth, score)) = self.search_book(opening_book) {
                // TODO: start searching from the book move's depth and use it to sort the search order
                if book_depth >= depth {
                    return Some((action, score));
                }
            }
        }
        search_iterative(
            &self.cells,
            self.current_player,
            depth,
            None,
            self.options.verbose,
            if self.options.use_table {
                transposition_table
            } else {
                None
            },
        )
    }

    /// Searches and returns the best action after a given time.
    pub fn search_to_time(
        &self,
        movetime: u64,
        opening_book: Option<&OpeningBook>,
        transposition_table: Option<&RwLock<SearchTable>>,
    ) -> Option<(Action, Score)> {
        if self.options.use_book {
            if let Some((action, _depth, score)) = self.search_book(opening_book) {
                // TODO: start searching from the book move's depth and use it to sort the search order
                return Some((action, score));
            }
        }
        search_iterative(
            &self.cells,
            self.current_player,
            u64::MAX,
            Some(Instant::now() + Duration::from_millis(movetime)),
            self.options.verbose,
            if self.options.use_table {
                transposition_table
            } else {
                None
            },
        )
    }

    /// Get the current board state.
    pub fn get_state(&self) -> (Cells, Player, u64, u64) {
        (
            self.cells,
            self.current_player,
            self.half_moves,
            self.full_moves,
        )
    }

    /// Sets the board state.
    pub fn set_state(&mut self, cells: &Cells, player: Player, half_moves: u64, full_moves: u64) {
        self.cells = *cells;
        self.current_player = player;
        self.half_moves = half_moves;
        self.full_moves = full_moves;
        self.last_piece_count = self.count_pieces();
    }

    /// Get the Pijersi Standard Notation of the current board state.
    pub fn get_string_state(&self) -> String {
        let (cells, current_player, half_moves, full_moves) = self.get_state();
        format!(
            "{} {} {} {}",
            cells_to_string(&cells),
            player_to_string(current_player).unwrap(),
            half_moves,
            full_moves,
        )
    }

    /// Sets the state of the board according to Pijersi Standard Notation data.
    pub fn set_string_state(&mut self, state_string: &str) -> Result<(), ParseError> {
        if let [cells_string, player_string, half_moves_string, full_moves_string] =
            state_string.split(' ').collect::<Vec<&str>>()[..]
        {
            let new_cells = string_to_cells(cells_string)?;
            let player = string_to_player(player_string)?;
            let half_moves = half_moves_string.parse::<u64>().map_err(|err| ParseError {
                kind: ParseErrorKind::InvalidInt(err),
                value: half_moves_string.to_string(),
            })?;
            let full_moves = full_moves_string.parse::<u64>().map_err(|err| ParseError {
                kind: ParseErrorKind::InvalidInt(err),
                value: full_moves_string.to_string(),
            })?;
            self.set_state(&new_cells, player, half_moves, full_moves);
            Ok(())
        } else {
            Err(ParseError {
                kind: ParseErrorKind::InvalidPSN,
                value: state_string.to_owned(),
            })
        }
    }

    /// Plays the chosen action provided in string representation.
    pub fn play_from_string(&mut self, action_string: &str) -> Result<(), RuntimeError> {
        let action = string_to_action(&self.cells, action_string)?;
        self.play(action)?;
        Ok(())
    }

    /// Plays the chosen action provided in `Action` representation.
    pub fn play(&mut self, action: Action) -> Result<(), RulesErrorKind> {
        if is_action_legal(&self.cells, self.current_player, action) {
            play_action(&mut self.cells, action);
            if self.current_player == 1 {
                self.full_moves += 1;
            }
            self.current_player = 1 - self.current_player;
            let piece_count = self.count_pieces();
            if self.last_piece_count == piece_count {
                self.half_moves += 1;
            } else {
                self.last_piece_count = piece_count;
                self.half_moves = 0;
            }
            Ok(())
        } else {
            Err(RulesErrorKind::IllegalAction(action))
        }
    }

    /// Counts the number of pieces on the board.
    ///
    /// A stack counts as two pieces.
    pub fn count_pieces(&self) -> u64 {
        self.cells
            .iter()
            .filter(|&&piece| !piece.is_empty())
            .map(|&piece| if piece.is_stack() { 2 } else { 1 })
            .sum()
    }

    /// Returns whether the board is in a winning position (one player is winning).
    pub fn is_win(&self) -> bool {
        is_position_win(&self.cells) || is_position_stalemate(&self.cells, self.current_player)
    }

    /// Returns whether the board is in a drawing position (half move counter reaches 20).
    pub fn is_draw(&self) -> bool {
        self.half_moves >= MAX_HALF_MOVES
    }

    /// Returns the winner of the game if there is one.
    pub fn get_winner(&self) -> Option<Player> {
        get_winning_player(&self.cells)
    }
}
