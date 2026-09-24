use uci::Response;

pub trait Sink {
    fn emit(&mut self, response: Response);
}
