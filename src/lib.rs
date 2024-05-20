pub mod board;
pub mod errors;
pub mod logic;
pub mod piece;
pub mod search;
pub mod ugi;
pub mod utils;

pub const ENGINE_NAME: &str = "Natural Selection";
pub const AUTHOR_NAME: &str = "Eclypse-Prime";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
