//! This module contains the Board struct and methods to represent a Pijersi board and play games.
//!
//! A board is represented as a `[Piece; 45]` array.
//!
//! Its board are indexed as such:
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

use crate::bitboard::Board;
use crate::errors::{ParseError, ParseErrorKind, RulesErrorKind, RuntimeError};
use crate::hash::search::SearchTable;
use crate::logic::actions::{Action, ActionTrait};
use crate::logic::rules::is_action_legal;
use crate::logic::translate::{
    action_to_string, player_to_string, string_to_action, string_to_player,
};
use crate::logic::{Player, MAX_HALF_MOVES};
use crate::search::alphabeta::search_iterative;
use crate::search::openings::OpeningBook;
use crate::search::Score;

/// This struct represents the board options.
///
/// It contains various parameters for the search engine:
/// * Using the opening book
/// * Printing the info logs during searches
pub struct GameOptions {
    /// Using the opening book
    pub use_book: bool,
    /// Using the hash table
    pub use_table: bool,
    /// Printing the info logs during searches
    pub verbose: bool,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl GameOptions {
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
/// * Current board
/// * Current player
/// * Current half moves count
/// * Current full moves count
/// * Piece count
pub struct Game {
    /// The board options
    pub options: GameOptions,
    /// The current board represented as bitboards (see [`crate::bitboard`])
    pub board: Board,
    /// The current player: 0 if white, 1 if black
    pub current_player: Player,
    half_moves: u64,
    full_moves: u64,
    last_piece_count: u64,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Board constructor: the board are empty on initialization, the current player is white.
    pub fn new() -> Self {
        Self {
            options: GameOptions::new(),
            board: Board::EMPTY,
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
        self.board.init();

        self.current_player = 0;
        self.half_moves = 0;
        self.full_moves = 1;
        self.last_piece_count = self.board.count_pieces(); // 28 starting pieces (14 for each side)
    }

    /// Prints the current pieces on the board.
    pub fn print(&self) {
        println!("{}", self.board.to_pretty_string());
    }

    /// Searches and returns the action corresponding to the current board state according to the opening book (if it exists)
    fn search_book(&self, opening_book: Option<&OpeningBook>) -> Option<(Action, u64, Score)> {
        if let Some(opening_book) = opening_book {
            if let Some(&(action, score)) = opening_book.lookup(self) {
                let depth = action.search_depth();
                let action_string = action_to_string(&self.board, action);
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
            &self.board,
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
            &self.board,
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
    pub fn get_state(&self) -> (Board, Player, u64, u64) {
        (
            self.board,
            self.current_player,
            self.half_moves,
            self.full_moves,
        )
    }

    /// Sets the board state.
    pub fn set_state(&mut self, board: &Board, player: Player, half_moves: u64, full_moves: u64) {
        self.board = *board;
        self.current_player = player;
        self.half_moves = half_moves;
        self.full_moves = full_moves;
        self.last_piece_count = self.board.count_pieces();
    }

    /// Get the Pijersi Standard Notation of the current board state.
    pub fn get_string_state(&self) -> String {
        let (board, current_player, half_moves, full_moves) = self.get_state();
        format!(
            "{} {} {} {}",
            board.to_fen(),
            player_to_string(current_player).unwrap(),
            half_moves,
            full_moves,
        )
    }

    /// Sets the state of the board according to Pijersi Standard Notation data.
    pub fn set_string_state(&mut self, state_string: &str) -> Result<(), ParseError> {
        if let [board_string, player_string, half_moves_string, full_moves_string] =
            state_string.split(' ').collect::<Vec<&str>>()[..]
        {
            let new_board = board_string.try_into()?;
            let player = string_to_player(player_string)?;
            let half_moves = half_moves_string.parse::<u64>().map_err(|err| ParseError {
                kind: ParseErrorKind::InvalidInt(err),
                value: half_moves_string.to_string(),
            })?;
            let full_moves = full_moves_string.parse::<u64>().map_err(|err| ParseError {
                kind: ParseErrorKind::InvalidInt(err),
                value: full_moves_string.to_string(),
            })?;
            self.set_state(&new_board, player, half_moves, full_moves);
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
        let action = string_to_action(&self.board, action_string)?;
        self.play(action)?;
        Ok(())
    }

    /// Plays the chosen action provided in `Action` representation.
    pub fn play(&mut self, action: Action) -> Result<(), RulesErrorKind> {
        if is_action_legal(&self.board, self.current_player, action) {
            self.board.play_action(action);
            if self.current_player == 1 {
                self.full_moves += 1;
            }
            self.current_player = 1 - self.current_player;
            let piece_count = self.board.count_pieces();
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

    /// Returns whether the board is in a winning position (one player is winning).
    pub fn is_win(&self) -> bool {
        self.board.is_win() || self.board.is_stalemate(self.current_player)
    }

    /// Returns whether the board is in a drawing position (half move counter reaches 20).
    pub fn is_draw(&self) -> bool {
        self.half_moves >= MAX_HALF_MOVES
    }

    /// Returns the winner of the game if there is one.
    pub fn get_winner(&self) -> Option<Player> {
        self.board.get_winner()
    }
}
