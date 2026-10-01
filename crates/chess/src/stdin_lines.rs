use std::io::{self, BufRead};
use std::ops::ControlFlow;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use uci::Command;

pub struct StdinLines;

impl StdinLines {
    pub fn spawn(stop: Arc<AtomicBool>) -> Receiver<String> {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            io::stdin()
                .lock()
                .lines()
                .map_while(Result::ok)
                .try_for_each(|line| Self::relay(&stop, &sender, line))
        });
        receiver
    }

    fn relay(stop: &AtomicBool, sender: &Sender<String>, line: String) -> ControlFlow<()> {
        stop.fetch_or(
            Command::try_from(line.as_str()).is_ok_and(|command| command.interrupts()),
            Ordering::Relaxed,
        );
        match sender.send(line) {
            Ok(()) => ControlFlow::Continue(()),
            Err(_) => ControlFlow::Break(()),
        }
    }
}
