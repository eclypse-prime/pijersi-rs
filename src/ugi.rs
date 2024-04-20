use std::process::exit;

use clap::{Args, Parser, Subcommand};

use crate::{board::Board, logic::translate::action_to_string, AUTHOR_NAME, ENGINE_NAME};

// TODO: make private
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
    moves: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum QueryArgs {
    Gameover,
    P1turn,
    Result,
    Islegal { action: String },
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
        Self {
            board: Board::default(),
        }
    }

    fn ugi(&self) {
        println!("id name {}", ENGINE_NAME);
        println!("id author {}", AUTHOR_NAME);
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
            GoArgs::Movetime { time } => {}
            GoArgs::Manual { action } => {}
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
                        for action_string in action_list.iter().skip(1) {
                            // TODO: rollback if err
                            let _ = self.board.play_from_string(action_string);
                        }
                    }
                }
            }
            PositionArgs::Fen(fen_args) => {
                let action_list: Vec<String> = fen_args.moves;
                match action_list.len() {
                    0 => {
                        // string to state
                    }
                    1 => {
                        println!("invalid argument {}", action_list[0]);
                    }
                    _ if action_list[0] != "moves" => {
                        println!("invalid argument {}", action_list[0]);
                    }
                    _ => {
                        // string to state
                        for action_string in action_list.iter().skip(1) {
                            // TODO: rollback if err
                            let _ = self.board.play_from_string(action_string);
                        }
                    }
                }
            }
        }
    }
    fn query(&self, query_args: QueryArgs) {
        match query_args {
            QueryArgs::Gameover => {}
            QueryArgs::P1turn => {}
            QueryArgs::Result => {}
            QueryArgs::Islegal { action } => {}
            QueryArgs::Fen => {}
        }
    }

    pub fn get_command(&mut self, command: &str) {
        let words: Vec<&str> = command.split_whitespace().collect();
        let parse_results = UgiParser::try_parse_from(words);
        println!("{parse_results:?}");

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
            // TODO: use error message from clap
            Err(_e) => println!("invalid command {:?}", command),
        }
    }
}
