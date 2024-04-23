use std::{process::exit, time::Instant};

use clap::{Args, Parser, Subcommand};

use crate::{
    board::Board,
    logic::{
        perft::perft,
        rules::is_action_legal,
        translate::{action_to_string, string_to_action},
    },
    AUTHOR_NAME, ENGINE_NAME,
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
}

#[derive(Subcommand, Debug)]
enum GoArgs {
    Depth { depth: u64 },
    Movetime { time: u64 },
    Manual { action: String },
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
    player: char,
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

pub struct UgiEngine {
    board: Board,
}

impl Default for UgiEngine {
    fn default() -> Self {
        UgiEngine::new()
    }
}

impl UgiEngine {
    pub fn new() -> Self {
        let mut new_self = Self {
            board: Board::default(),
        };
        new_self.board.init();
        new_self
    }

    fn ugi(&self) {
        println!("id name {ENGINE_NAME}");
        println!("id author {AUTHOR_NAME}");
        println!("ugiok");
    }

    fn isready(&self) {
        // TODO: heavy inits here
        println!("readyok");
    }
    fn uginewgame(&mut self) {
        self.board.init();
    }
    // fn help(&self) {
    //     println!("ugi");
    //     println!("isready");
    //     println!("uginewgame");
    //     println!("quit");
    //     println!("go");
    //     println!("position");
    //     println!("query");
    // }
    fn exit(&self) {
        exit(0);
    }
    fn go(&self, go_args: GoArgs) {
        match go_args {
            GoArgs::Depth { depth } => {
                let action = self.board.search_to_depth(depth);
                let action_string = match action {
                    Some(action) => action_to_string(&self.board.cells, action),
                    None => "------".to_owned(), // TODO: info null move
                };
                println!("bestmove {action_string}");
            }
            GoArgs::Movetime { time } => {
                let action = self.board.search_to_time(time);
                let action_string = match action {
                    Some(action) => action_to_string(&self.board.cells, action),
                    None => "------".to_owned(), // TODO: info null move
                };
                println!("bestmove {action_string}");
            }
            GoArgs::Manual { action } => {}
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
                    1 => {
                        println!("invalid argument {}", action_list[0]);
                    }
                    _ if action_list[0] != "moves" => {
                        println!("invalid argument {}", action_list[0]);
                    }
                    _ => {
                        self.board.init();
                        // TODO: make function (duplicate code)
                        for action_string in action_list.iter().skip(1) {
                            // TODO: rollback if err
                            let result = self.board.play_from_string(action_string);
                            match result {
                                Ok(_v) => (),
                                Err(e) => println!("info error \"{e}\""),
                            }
                        }
                    }
                }
            }
            PositionArgs::Fen(fen_args) => {
                let action_list: Vec<String> = fen_args.moves;
                match action_list.len() {
                    0 => {
                        match self.board.set_state(
                            &fen_args.fen,
                            fen_args.player,
                            fen_args.half_moves,
                            fen_args.full_moves,
                        ) {
                            Ok(()) => (),
                            Err(e) => println!("info error \"{e}\""),
                        }
                    }
                    1 => {
                        println!("invalid argument {}", action_list[0]);
                    }
                    _ if action_list[0] != "moves" => {
                        println!("invalid argument {}", action_list[0]);
                    }
                    _ => {
                        match self.board.set_state(
                            &fen_args.fen,
                            fen_args.player,
                            fen_args.half_moves,
                            fen_args.full_moves,
                        ) {
                            Ok(()) => {
                                // TODO: make function (duplicate code)
                                for action_string in action_list.iter().skip(1) {
                                    // TODO: rollback if err
                                    let result = self.board.play_from_string(action_string);
                                    match result {
                                        Ok(_v) => (),
                                        Err(e) => println!("info error \"{e}\""),
                                    }
                                }
                            }
                            Err(e) => println!("info error \"{e}\""),
                        }
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
                println!("{}", self.board.get_state());
            }
        }
    }

    pub fn get_command(&mut self, command: &str) {
        let words: Vec<&str> = command.split_whitespace().collect();
        let parse_results = UgiParser::try_parse_from(words);

        match parse_results {
            Ok(v) => match v.command {
                Commands::Ugi => self.ugi(),
                Commands::Isready => self.isready(),
                Commands::Uginewgame => self.uginewgame(),
                Commands::Quit => self.exit(),
                Commands::Go(go_args) => self.go(go_args),
                Commands::Position(position_args) => self.position(position_args),
                Commands::Query(query_args) => self.query(query_args),
            },
            Err(e) => {
                println!(
                    "info error \"{}\"",
                    e.to_string().split('\n').next().unwrap()
                );
            }
        }
    }
}
