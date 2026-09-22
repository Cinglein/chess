#![cfg_attr(not(test), no_std)]

mod bounds;
mod depth;
mod full_width;
mod negamax;
mod principal;
mod quiescence;
mod regime;
mod search;
mod window;

pub use depth::Depth;
pub use search::Search;
