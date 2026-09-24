#![cfg_attr(not(test), no_std)]

mod search;

pub use search::{BoundKind, Depth, Search, TableEntry, TranspositionTable};
