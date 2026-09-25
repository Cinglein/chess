use std::io::{BufRead, BufReader};
use std::process::ChildStdout;
use std::sync::mpsc::{self, Receiver};
use std::thread;

pub(super) struct LineReader;

impl LineReader {
    pub(super) fn spawn(stdout: ChildStdout) -> Receiver<String> {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            BufReader::new(stdout)
                .lines()
                .map_while(Result::ok)
                .try_for_each(|line| sender.send(line))
        });
        receiver
    }
}
