use std::io::{self, BufRead};
use std::ops::ControlFlow;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct StdinLines;

impl StdinLines {
    const INTERRUPTING: [&str; 2] = ["stop", "quit"];

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
        if Self::INTERRUPTING.contains(&line.trim()) {
            stop.store(true, Ordering::Relaxed);
        }
        match sender.send(line) {
            Ok(()) => ControlFlow::Continue(()),
            Err(_) => ControlFlow::Break(()),
        }
    }
}
