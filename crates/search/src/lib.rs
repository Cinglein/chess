#![cfg_attr(not(test), no_std)]

mod bound;
mod bounds;
mod depth;
mod full_width;
mod killer_table;
mod killers;
mod limit;
mod lower;
mod move_priority;
mod negamax;
mod ordered_moves;
mod principal;
mod quiescence;
mod regime;
mod search;
mod upper;
mod window;

pub use depth::Depth;
pub use search::Search;
