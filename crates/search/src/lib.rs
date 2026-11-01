#![cfg_attr(not(test), no_std)]

mod search;

pub use search::{Depth, Interrupt, Search, TableEntry, TranspositionTable};
