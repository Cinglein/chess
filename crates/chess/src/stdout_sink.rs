use engine::Sink;
use uci::Response;

pub struct StdoutSink;

impl Sink for StdoutSink {
    fn emit(&mut self, response: Response) {
        println!("{response}");
    }
}
