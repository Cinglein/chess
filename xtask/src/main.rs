mod hex_literal;
mod magic_search;
mod magic_tables;
mod no_comments;
mod no_free_fns;
mod task;
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
