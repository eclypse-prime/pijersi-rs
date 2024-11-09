#![warn(missing_docs)]
//! A UGI engine for the Pijersi board game.
//!
//! This project is a Rust implementation of a Pijersi game engine. It can be used standalone (using the [UGI protocol](https://github.com/arthur-liu-lsh/pijersi-engine/blob/main/ugi.md)) and will also provide bindings for use in C#/Unity and Python projects.
//!
//! The engine is named Natural Selection. It uses the Alpha-Beta search to find the best move for a given position.

pub mod board;
pub mod errors;
pub mod hash;
pub mod logic;
pub mod piece;
pub mod search;
pub mod ugi;
pub mod utils;

const ENGINE_NAME: &str = "Natural Selection";
const AUTHOR_NAME: &str = "Eclypse-Prime";
const VERSION: &str = env!("CARGO_PKG_VERSION");
