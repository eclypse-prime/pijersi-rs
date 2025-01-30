//! This module implements the structs and methods used to implement a transposition table to reduce search times.
//!
//! The transposition table stores previously searched positions at a given depth.

use crate::logic::{
    actions::{Action, ActionTrait},
    index::CellIndex,
    Player,
};

const KEY_BIT_WIDTH: usize = 24;
const SEARCH_TABLE_SIZE: usize = 2 << KEY_BIT_WIDTH;
const SEARCH_TABLE_MASK: usize = (2 << (KEY_BIT_WIDTH)) - 1;

#[derive(Clone, Copy, Default)]
struct SearchEntry {
    depth: u8,
    player: u8,
    key: usize,
    index_start: u8,
    index_mid: u8,
    index_end: u8,
}

impl SearchEntry {
    #[inline]
    fn new(depth: u64, player: Player, key: usize, action: Action) -> Self {
        let (index_start, index_mid, index_end) = action.to_indices();
        SearchEntry {
            depth: depth as u8,
            player,
            key,
            index_start: index_start as u8,
            index_mid: index_mid as u8,
            index_end: index_end as u8,
        }
    }
    #[inline]
    fn unpack(self) -> (u64, u8, Action) {
        (
            self.depth as u64,
            self.player,
            Action::from_indices(
                self.index_start as CellIndex,
                self.index_mid as CellIndex,
                self.index_end as CellIndex,
            ),
        )
    }
}

/// Search transposition table
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
    /// Inserts an entry corresponding to its position hash in the transposition table.
    pub fn insert(&mut self, hash: usize, depth: u64, player: u8, action: Action) {
        self.data[hash & SEARCH_TABLE_MASK] = SearchEntry::new(depth, player, hash, action);
    }
    #[inline]
    /// Reads the transposition table and returns the entry corresponding to the position hash if there is one.
    pub fn read(&mut self, hash: usize) -> Option<(u64, u8, Action)> {
        let entry = self.data[hash & SEARCH_TABLE_MASK];
        if entry.key == hash {
            Some(entry.unpack())
        } else {
            None
        }
    }
}
