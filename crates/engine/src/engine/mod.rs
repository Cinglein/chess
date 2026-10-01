mod lifecycle;
mod sink;
mod thinker;

pub use sink::Sink;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use board::{Board, State};
use lifecycle::Lifecycle;
use search::TableEntry;
use thinker::Thinker;
use uci::{EngineOption, GoLimits, Identity, Position, Receiver, Response};

pub struct Engine<S: Sink> {
    board: Board,
    entries: Vec<TableEntry>,
    stop: Arc<AtomicBool>,
    sink: S,
    lifecycle: Lifecycle,
}

impl<S: Sink> State for Engine<S> {}

impl<S: Sink> Engine<S> {
    pub const NAME: &str = "chess";
    pub const AUTHOR: &str = "Cinglein";
    const TABLE_BYTES: usize = 16 << 20;
    const ENTRY_CAPACITY: usize = Self::TABLE_BYTES / size_of::<TableEntry>();

    #[must_use]
    pub fn new(stop: Arc<AtomicBool>, sink: S) -> Engine<S> {
        const {
            assert!(Self::ENTRY_CAPACITY > 0);
        }
        Engine {
            board: Board::START,
            entries: vec![TableEntry::EMPTY; Self::ENTRY_CAPACITY],
            stop,
            sink,
            lifecycle: Lifecycle::Running,
        }
    }

    #[must_use]
    pub const fn sink(&self) -> &S {
        &self.sink
    }

    #[must_use]
    pub fn is_ending(&self) -> bool {
        self.lifecycle == Lifecycle::Ending
    }

    fn board_after(position: &Position<'_>) -> Option<Board> {
        position
            .moves()
            .iter()
            .try_fold(position.start(), |board, notation| {
                notation
                    .ok()
                    .and_then(|notation| board.resolve_move(notation))
                    .and_then(|chess_move| board.make_move(chess_move))
            })
    }
}

impl<'line, S: Sink> Receiver<'line> for Engine<S> {
    fn identify(mut self) -> Self {
        self.sink.emit(Response::Id(Identity::Name(Self::NAME)));
        self.sink.emit(Response::Id(Identity::Author(Self::AUTHOR)));
        self.sink.emit(Response::UciOk);
        self
    }

    fn confirm_ready(mut self) -> Self {
        self.sink.emit(Response::ReadyOk);
        self
    }

    fn configure(self, _: EngineOption<'line>) -> Self {
        self
    }

    fn reset_game(mut self) -> Self {
        self.entries.fill(TableEntry::EMPTY);
        self
    }

    fn place(self, position: Position<'line>) -> Self {
        Engine {
            board: Self::board_after(&position).unwrap_or(self.board),
            ..self
        }
    }

    fn start_search(mut self, limits: GoLimits) -> Self {
        self.stop.store(false, Ordering::Relaxed);
        Thinker::new(&self.board, &self.stop).think(&mut self.entries, &limits, &mut self.sink);
        self
    }

    fn halt(self) -> Self {
        self
    }

    fn shut_down(self) -> Self {
        Engine {
            lifecycle: Lifecycle::Ending,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::time::{Duration, Instant};

    use search::Depth;
    use uci::{Command, Response};

    use super::{Engine, Sink};

    const PLACE: &str = "position fen 6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
    const SEARCH: &str = "go depth 2";
    const EXPECTED: &str = "bestmove a1a8";
    const QUIT: &str = "quit";
    const TIMED: &str = "go wtime 3000 btime 3000 winc 100 binc 100";
    const SETTLED_DEPTH: Depth = Depth::new(4);

    impl Sink for Vec<String> {
        fn emit(&mut self, response: Response<'_>) {
            self.push(response.to_string());
        }
    }

    #[test]
    fn a_placed_position_searched_to_depth_two_announces_the_mate_and_quit_ends_the_engine() {
        let fresh = Engine::new(Arc::new(AtomicBool::new(false)), Vec::new());
        let placed = Command::try_from(PLACE).unwrap().deliver_to(fresh);
        let engine = Command::try_from(SEARCH).unwrap().deliver_to(placed);
        let announced = engine.sink();
        assert!(
            announced.iter().any(|line| line == EXPECTED),
            "{announced:?}"
        );
        assert!(!engine.is_ending());
        assert!(
            Command::try_from(QUIT)
                .unwrap()
                .deliver_to(engine)
                .is_ending()
        );
    }

    #[test]
    fn a_timed_search_deepens_past_the_opening_plies_and_stops_inside_its_allowance() {
        let fresh = Engine::new(Arc::new(AtomicBool::new(false)), Vec::new());
        let started = Instant::now();
        let engine = Command::try_from(TIMED).unwrap().deliver_to(fresh);
        let elapsed = started.elapsed();
        let deepest = engine
            .sink()
            .iter()
            .filter_map(|line| Response::try_from(line.as_str()).ok()?.info())
            .map(|info| info.depth())
            .max();
        assert!(deepest >= Some(SETTLED_DEPTH), "{deepest:?}");
        assert!(elapsed < Duration::from_secs(1), "{elapsed:?}");
    }
}
