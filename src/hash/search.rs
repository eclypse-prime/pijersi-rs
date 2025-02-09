//! This module implements the structs and methods used to implement a transposition table to reduce search times.
//!
//! The transposition table stores previously searched positions at a given depth.

use crate::{
    logic::{
        actions::{Action, ActionTrait},
        index::CellIndex,
    },
    search::{NodeType, Score},
};

const KEY_BIT_WIDTH: usize = 24;
const SEARCH_TABLE_SIZE: usize = 2 << KEY_BIT_WIDTH;
const SEARCH_TABLE_MASK: usize = (2 << (KEY_BIT_WIDTH)) - 1;

const BUCKET_SIZE: usize = 4;

/// A search entry. It contains information about a previously searched position.
/// It contains:
/// * Its hash key that represents the position and the current player
/// * The best action
/// * The search depth
/// * The score
/// * The node type (PV, Cut, All)
#[derive(Clone, Copy, Default, Debug)]
struct SearchEntry {
    hash: usize,
    index_start: u8,
    index_mid: u8,
    index_end: u8,
    depth: u8,
    score: Score,
    node_type: NodeType,
}

impl SearchEntry {
    #[inline]
    fn new(hash: usize, action: Action, depth: u64, score: Score, node_type: NodeType) -> Self {
        let (index_start, index_mid, index_end) = action.to_indices();
        SearchEntry {
            hash,
            index_start: index_start as u8,
            index_mid: index_mid as u8,
            index_end: index_end as u8,
            depth: depth as u8,
            score,
            node_type,
        }
    }
    /// Converts stored search information into usable formats
    #[inline]
    fn unpack(self) -> (u64, Action, Score, NodeType) {
        (
            self.depth as u64,
            Action::from_indices(
                self.index_start as CellIndex,
                self.index_mid as CellIndex,
                self.index_end as CellIndex,
            ),
            self.score,
            self.node_type,
        )
    }
}

/// Search transposition table bucket. It contains a fixed number of search entries.
#[derive(Clone, Copy, Default, Debug)]
struct Bucket {
    entries: [SearchEntry; BUCKET_SIZE],
}

impl Bucket {
    /// Inserts an entry in the bucket if the replace conditions are met.
    /// 
    /// * If there no entry with the same hash, replace the first empty entry or the entry with the lowest stored depth
    /// * If there is an entry with the same hash:
    ///   - Replace the entry if the new depth is higher
    ///   - Replace the entry if the new depth is the same as the entry's and the new depth is a PV node and the stored entry is a Cut or All node
    fn insert(
        &mut self,
        hash: usize,
        depth: u64,
        action: Action,
        score: Score,
        node_type: NodeType,
    ) {
        let mut min_depth = u8::MAX;
        let mut min_index: usize = 0;
        let mut empty_entry = false;
        for i in 0..BUCKET_SIZE {
            let entry = self.entries[i];
            if hash == entry.hash {
                if depth as u8 > entry.depth
                    || (depth as u8 == entry.depth
                        && entry.node_type != NodeType::PV
                        && node_type == NodeType::PV)
                {
                    self.entries[i] = SearchEntry::new(hash, action, depth, score, node_type);
                }
                return;
            }
            if entry.depth == 0 {
                min_index = i;
                empty_entry = true;
            }
            if entry.depth < min_depth && !empty_entry {
                min_depth = entry.depth;
                min_index = i;
            }
        }
        self.entries[min_index] = SearchEntry::new(hash, action, depth, score, node_type);
    }

    /// Searches if there is an entry in the bucket with the right hash.
    fn read(&self, hash: usize) -> Option<(u64, Action, Score, NodeType)> {
        for entry in self.entries {
            if entry.hash == hash {
                return Some(entry.unpack());
            }
        }
        None
    }
}

/// Search transposition table. It contains a vector of buckets which contain search entries.
pub struct SearchTable {
    data: Vec<Bucket>,
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
    pub fn insert(
        &mut self,
        hash: usize,
        depth: u64,
        action: Action,
        score: Score,
        node_type: NodeType,
    ) {
        let bucket = &mut self.data[hash & SEARCH_TABLE_MASK];
        bucket.insert(hash, depth, action, score, node_type);
    }
    #[inline]
    /// Reads the transposition table and returns the entry corresponding to the position hash if there is one.
    pub fn read(&self, hash: usize) -> Option<(u64, Action, Score, NodeType)> {
        let bucket = self.data[hash & SEARCH_TABLE_MASK];
        bucket.read(hash)
    }
    #[inline]
    /// Empties the transposition table
    pub fn empty(&mut self) {
        for i in 0..SEARCH_TABLE_SIZE {
            self.data[i] = Default::default();
        }
    }
}
