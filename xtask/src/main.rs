mod ci;
mod distinct_signatures;
mod fn_shape;
mod hex_literal;
mod lint;
mod magic_search;
mod magic_tables;
mod magics;
mod named_lifetimes;
mod no_comments;
mod no_free_fns;
mod private_fns;
mod source_file;
mod state_graph;
mod task;
mod test_budget;
mod wasm;
mod workspace;
mod xor_shift;

use std::env;
use std::process::ExitCode;

use crate::task::Task;

fn main() -> ExitCode {
    let task = env::args()
        .nth(1)
        .ok_or_else(Task::usage)
        .and_then(|argument| argument.parse::<Task>().map_err(|_| Task::usage()));
    match task.and_then(Task::run) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
