#![cfg_attr(not(test), no_std)]

mod depth;
mod negamax;
mod search;

pub use depth::Depth;
pub use search::Search;
