//! This module implements the structs and methods used to implement a transposition table to reduce search times.
//!
//! The transposition table stores previously searched positions at a given depth.

use crate::logic::{
    actions::{Action, ActionTrait, ACTION_MASK},
    index::INDEX_WIDTH,
};

const KEY_BIT_WIDTH: usize = 27;
const SEARCH_TABLE_SIZE: usize = 2 << KEY_BIT_WIDTH;
const SEARCH_TABLE_MASK: usize = (2 << (KEY_BIT_WIDTH)) - 1;

#[derive(Clone, Copy, Default)]
struct SearchEntry {
    score: i32,
    depth: u8,
    player: u8,
}

impl SearchEntry {
    #[inline]
    fn new(score: i32, depth: u64, player: u8) -> Self {
        SearchEntry {
            score: score as i32,
            depth: depth as u8,
            player,
        }
    }
    #[inline]
    fn unpack(self) -> (i32, u64, u8) {
        (self.score, self.depth as u64, self.player)
    }
}

pub struct SearchTable {
    data: Vec<SearchEntry>,
}

impl Default for SearchTable {
    fn default() -> Self {
        SearchTable {
            data: vec![Default::default(); SEARCH_TABLE_SIZE],
        }
    }
}

impl SearchTable {
    #[inline]
    pub fn insert(&mut self, hash: usize, score: i32, depth: u64, player: u8) {
        self.data[hash & SEARCH_TABLE_MASK] = SearchEntry::new(score, depth, player);
    }
    #[inline]
    pub fn read(&mut self, hash: usize) -> Option<(i32, u64, u8)> {
        let entry = self.data[hash & SEARCH_TABLE_MASK];
        Some(entry.unpack())
    }
}
