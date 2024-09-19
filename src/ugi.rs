//! This module implements the UGI protocol.

use clap::{Args, Parser, Subcommand};
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use std::{process::exit, time::Instant};

use crate::{
    board::Board,
    errors::{get_error_trace, RuntimeError, UgiErrorKind},
    logic::{
        perft::perft,
        rules::is_action_legal,
        translate::{action_to_string, string_to_action, string_to_cells, string_to_player},
    },
    search::openings::OpeningBook,
    utils::parse_bool_arg,
    AUTHOR_NAME, ENGINE_NAME, VERSION,
};

#[derive(Parser, Debug)]
#[command(no_binary_name(true))]
struct UgiParser {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ugi,
    Isready,
    Uginewgame,
    Quit,
    #[command(subcommand)]
    Go(GoArgs),
    #[command(subcommand)]
    Position(PositionArgs),
    #[command(subcommand)]
    Query(QueryArgs),
    #[command(subcommand)]
    Setoption(SetoptionArgs),
}

#[derive(Subcommand, Debug)]
enum GoArgs {
    Depth { depth: u64 },
    Movetime { time: u64 },
    Manual { action_string: String },
    Perft { depth: u64 },
}

#[derive(Subcommand, Debug)]
enum PositionArgs {
    Startpos(StartposArgs),
    Fen(FenArgs),
}

#[derive(Args, Debug)]
struct StartposArgs {
    moves: Vec<String>,
}

#[derive(Args, Debug)]
struct FenArgs {
    fen: String,
    player: String,
    half_moves: u64,
    full_moves: u64,
    moves: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum QueryArgs {
    Gameover,
    P1turn,
    Result,
    Islegal { action_string: String },
    Fen,
}

#[derive(Subcommand, Debug)]
enum SetoptionArgs {
    UseBook { value: String },
    Verbose { value: String },
}

/// The UgiEngine struct that implements the UGI protocol.
pub struct UgiEngine {
    board: Board,
    opening_book: Option<OpeningBook>,
}

impl Default for UgiEngine {
    fn default() -> Self {
        UgiEngine::new()
    }
}

impl UgiEngine {
    /// Creates a new UgiEngine
    pub fn new() -> Self {
        let mut new_self = Self {
            board: Board::default(),
            opening_book: None,
        };
        new_self.board.init();
        new_self
    }

    fn ugi(&self) {
        println!("id name {ENGINE_NAME} {VERSION}");
        println!("id author {AUTHOR_NAME}");
        println!("info target platform {CURRENT_PLATFORM} compiled on {COMPILED_ON}");
        println!("option name verbose type check default true");
        println!("option name use-book type check default true");
        println!("ugiok");
    }

    fn isready(&mut self) {
        self.opening_book = Some(OpeningBook::new());
        println!("readyok");
    }

    fn uginewgame(&mut self) {
        self.board.init();
    }

    // TODO: help function?
    fn quit(&self) {
        exit(0);
    }

    fn go(&mut self, go_args: GoArgs) {
        match go_args {
            GoArgs::Depth { depth } => {
                let result = self
                    .board
                    .search_to_depth(depth, self.opening_book.as_ref());
                let action_string = match result {
                    Some((action, _score)) => action_to_string(&self.board.cells, action),
                    None => "------".to_owned(), // TODO: info null move
                };
                println!("bestmove {action_string}");
            }
            GoArgs::Movetime { time } => {
                let action = self.board.search_to_time(time, self.opening_book.as_ref());
                let action_string = match action {
                    Some((action, _score)) => action_to_string(&self.board.cells, action),
                    None => "------".to_owned(), // TODO: info null move
                };
                println!("bestmove {action_string}");
            }
            GoArgs::Manual { action_string } => {
                let result = self.board.play_from_string(&action_string);
                match result {
                    Ok(_v) => (),
                    Err(e) => print_error_trace(&e),
                }
            }
            GoArgs::Perft { depth } => {
                let start_time = Instant::now();
                let count = perft(&self.board.cells, self.board.current_player, depth);
                let duration: f64 = start_time.elapsed().as_micros() as f64 / 1000f64;
                println!("info perft depth {depth} result {count} time {duration}");
            }
        }
    }

    fn position(&mut self, position_args: PositionArgs) {
        match position_args {
            PositionArgs::Startpos(startpos_args) => {
                let action_list = startpos_args.moves;
                match action_list.len() {
                    0 => {
                        self.board.init();
                    }
                    1 => print_error_trace(&RuntimeError::UGI(UgiErrorKind::InvalidUGIPosition(
                        action_list.join(" "),
                    ))),
                    _ if action_list[0] != "moves" => print_error_trace(&RuntimeError::UGI(
                        UgiErrorKind::InvalidUGIPosition(action_list.join(" ")),
                    )),
                    _ => {
                        self.board.init();
                        play_actions(&mut self.board, &action_list[1..]);
                    }
                }
            }
            PositionArgs::Fen(fen_args) => {
                let action_list: &Vec<String> = &fen_args.moves;
                match action_list.len() {
                    0 => {
                        set_fen(&mut self.board, &fen_args);
                    }
                    1 => print_error_trace(&RuntimeError::UGI(UgiErrorKind::InvalidUGIPosition(
                        action_list.join(" "),
                    ))),
                    _ if action_list[0] != "moves" => print_error_trace(&RuntimeError::UGI(
                        UgiErrorKind::InvalidUGIPosition(action_list.join(" ")),
                    )),
                    _ => {
                        set_fen(&mut self.board, &fen_args);
                        play_actions(&mut self.board, &action_list[1..]);
                    }
                }
            }
        }
    }

    fn query(&self, query_args: QueryArgs) {
        match query_args {
            QueryArgs::Gameover => {
                if self.board.is_win() || self.board.is_draw() {
                    println!("response true");
                } else {
                    println!("response false");
                }
            }
            QueryArgs::P1turn => {
                if self.board.current_player == 0 {
                    println!("response true");
                } else {
                    println!("response false");
                }
            }
            QueryArgs::Result => {
                if self.board.is_win() {
                    let winner = self.board.get_winner();
                    match winner {
                        Some(0) => {
                            println!("response p1win");
                        }
                        Some(1) => {
                            println!("response p2win");
                        }
                        _ => {
                            println!("response none");
                        }
                    };
                } else if self.board.is_draw() {
                    println!("response draw");
                } else {
                    println!("response none");
                }
            }
            QueryArgs::Islegal { action_string } => {
                let action_result = string_to_action(&self.board.cells, &action_string);
                match action_result {
                    Ok(action) => {
                        if is_action_legal(&self.board.cells, self.board.current_player, action) {
                            println!("response true");
                        } else {
                            println!("response false");
                        }
                    }
                    Err(_) => {
                        println!("response false");
                    }
                }
            }
            QueryArgs::Fen => {
                println!("{}", self.board.get_string_state());
            }
        }
    }

    fn setoption(&mut self, option: SetoptionArgs) {
        match option {
            SetoptionArgs::UseBook { value } => match parse_bool_arg(&value) {
                Ok(value) => {
                    self.board.options.use_book = value;
                }
                Err(e) => print_error_trace(&e),
            },
            SetoptionArgs::Verbose { value } => match parse_bool_arg(&value) {
                Ok(value) => {
                    self.board.options.verbose = value;
                }
                Err(e) => print_error_trace(&e),
            },
        }
    }

    /// Reads a command and responds to it (using stdout).
    ///
    /// The parsing is done using the clap crate.
    pub fn get_command(&mut self, command: &str) {
        let words: Vec<&str> = command.split_whitespace().collect();
        let parse_results = UgiParser::try_parse_from(words);

        match parse_results {
            Ok(v) => match v.command {
                Commands::Ugi => self.ugi(),
                Commands::Isready => self.isready(),
                Commands::Uginewgame => self.uginewgame(),
                Commands::Quit => self.quit(),
                Commands::Go(go_args) => self.go(go_args),
                Commands::Position(position_args) => self.position(position_args),
                Commands::Query(query_args) => self.query(query_args),
                Commands::Setoption(setoption_args) => self.setoption(setoption_args),
            },
            Err(e) => {
                print_error_trace(&if command.is_empty() {
                    RuntimeError::UGI(UgiErrorKind::EmptyCommand)
                } else {
                    RuntimeError::UGI(UgiErrorKind::ClapError(e))
                });
            }
        }
    }
}

/// Utility function to print an error's traceback.
fn print_error_trace(error: &dyn std::error::Error) {
    let trace = get_error_trace(error);
    for source in trace {
        for line in source.lines().filter(|&line| !line.is_empty()) {
            println!("info error \"{line}\"")
        }
    }
}

/// Plays all the actions in the list. If there is an invalid action in the list, stops and rolls back to the initial state.
fn play_actions(board: &mut Board, actions: &[String]) {
    let (cells, player, half_moves, full_moves) = board.get_state();
    for action_string in actions {
        let result = board.play_from_string(action_string);
        match result {
            Ok(_v) => (),
            Err(e) => {
                board.set_state(&cells, player, half_moves, full_moves);
                print_error_trace(&e);
                break;
            }
        }
    }
}

/// Sets the state of the board using PSN/FEN arguments
fn set_fen(board: &mut Board, fen_args: &FenArgs) {
    let new_cells = string_to_cells(&fen_args.fen);
    let player = string_to_player(&fen_args.player);
    match (new_cells, player) {
        (Ok(new_cells), Ok(player)) => {
            board.set_state(&new_cells, player, fen_args.half_moves, fen_args.full_moves);
        }
        (Err(e), Ok(_player)) => print_error_trace(&e),
        (Ok(_player), Err(e)) => print_error_trace(&e),
        (Err(e1), Err(e2)) => {
            print_error_trace(&e1);
            print_error_trace(&e2);
        }
    }
}
