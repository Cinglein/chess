mod stdin_lines;
mod stdout_sink;

use std::ops::ControlFlow;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use engine::Engine;
use stdin_lines::StdinLines;
use stdout_sink::StdoutSink;
use uci::Command;

fn main() {
    let stop = Arc::new(AtomicBool::new(false));
    let lines = StdinLines::spawn(Arc::clone(&stop));
    let _finished = lines
        .iter()
        .try_fold(Engine::new(stop, StdoutSink), |engine, line| {
            let engine = match Command::try_from(line.as_str()) {
                Ok(command) => command.deliver_to(engine),
                Err(_) => engine,
            };
            if engine.is_ending() {
                ControlFlow::Break(engine)
            } else {
                ControlFlow::Continue(engine)
            }
        });
}
