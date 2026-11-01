use engine::{Engine, Sink};
use search::Depth;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use uci::{Command, Response};

const PLACE: &str = "position fen 6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
const BLACK_TO_MOVE: &str = "position startpos moves e2e4";
const SEARCH: &str = "go depth 2";
const EXPECTED: &str = "bestmove a1a8";
const QUIT: &str = "quit";
const TIMED: &str = "go wtime 3000 btime 3000 winc 100 binc 100";
const NODE_LIMITED: &str = "go nodes 2048";
const MOVE_TIMED: &str = "go movetime 20";
const NEARLY_OUT: &str = "go wtime 20 btime 20 winc 50 binc 50";
const OWN_CLOCK_SHORT: &str = "go wtime 600000 btime 100";
const UNBOUNDED: &str = "go infinite";
const SETTLED_DEPTH: Depth = Depth::new(4);
const SEARCH_CEILING: Depth = Depth::new(63);
const STOP_AFTER: Duration = Duration::from_millis(20);
const REPORT_LAG: Duration = Duration::from_millis(10);
const SHORT_OF_THE_LAG: &str = "go movetime 10";
const MANY_MOVES_OF_LAG: &str = "go wtime 6000 btime 6000";
const ONE_PLY: Depth = Depth::new(1);

#[derive(Default)]
struct Lines(Vec<String>);

impl Sink for Lines {
    fn emit(&mut self, response: Response<'_>) {
        self.0.push(response.to_string());
    }
}

impl Lines {
    fn deepest(&self) -> Option<Depth> {
        self.0
            .iter()
            .filter_map(|line| Response::try_from(line.as_str()).ok()?.info())
            .map(|info| *AsRef::<Depth>::as_ref(&info))
            .max()
    }

    fn answered(&self) -> bool {
        self.0.iter().any(|line| line.starts_with("bestmove"))
    }
}

#[derive(Default)]
struct Lagging(Lines);

impl Sink for Lagging {
    fn emit(&mut self, response: Response<'_>) {
        self.0.emit(response);
        thread::sleep(REPORT_LAG);
    }
}

impl Lagging {
    fn deepest_after(line: &str) -> Option<Depth> {
        let fresh = Engine::new(Arc::default(), Lagging::default());
        Command::try_from(line)
            .unwrap()
            .deliver_to(fresh)
            .sink()
            .0
            .deepest()
    }
}

struct Session;

impl Session {
    fn after(stop: Arc<AtomicBool>, lines: &[&str]) -> Engine<Lines> {
        let fresh = Engine::new(stop, Lines::default());
        lines.iter().fold(fresh, |engine, line| {
            Command::try_from(*line).unwrap().deliver_to(engine)
        })
    }

    fn answered(lines: &[&str]) -> Engine<Lines> {
        Self::after(Arc::default(), lines)
    }
}

struct Timed {
    engine: Engine<Lines>,
    elapsed: Duration,
}

impl Timed {
    fn after(lines: &[&str]) -> Timed {
        let started = Instant::now();
        let engine = Session::answered(lines);
        Timed {
            engine,
            elapsed: started.elapsed(),
        }
    }
}

#[test]
fn a_placed_position_searched_to_depth_two_announces_the_mate_and_quit_ends_the_engine() {
    let engine = Session::answered(&[PLACE, SEARCH]);
    let announced = &engine.sink().0;
    assert!(
        announced.iter().any(|line| line == EXPECTED) && !engine.is_ending(),
        "{announced:?}"
    );
    assert!(Session::answered(&[QUIT]).is_ending());
}

#[test]
fn a_timed_search_deepens_past_the_opening_plies_and_stops_inside_its_allowance() {
    let Timed { engine, elapsed } = Timed::after(&[TIMED]);
    let deepest = engine.sink().deepest();
    assert!(deepest >= Some(SETTLED_DEPTH), "{deepest:?}");
    assert!(elapsed < Duration::from_secs(1), "{elapsed:?}");
}

#[test]
fn a_node_limit_or_a_move_time_ends_the_search_with_an_answer_long_before_the_deepest_ply() {
    for limit in [NODE_LIMITED, MOVE_TIMED] {
        let Timed { engine, elapsed } = Timed::after(&[limit]);
        let lines = engine.sink();
        assert!(
            lines.answered() && lines.deepest() < Some(SEARCH_CEILING),
            "{limit}"
        );
        assert!(elapsed < Duration::from_secs(1), "{limit}: {elapsed:?}");
    }
}

#[test]
fn the_stop_flag_ends_an_unbounded_search_with_an_answer() {
    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    let stopper = thread::spawn(move || {
        thread::sleep(STOP_AFTER);
        flag.store(true, Ordering::Relaxed);
    });
    let engine = Session::after(stop, &[UNBOUNDED]);
    stopper.join().unwrap();
    assert!(engine.sink().answered());
}

#[test]
fn a_move_spends_its_own_sides_clock_and_never_more_than_that_clock_holds() {
    let positions: [&[&str]; 2] = [&[NEARLY_OUT], &[BLACK_TO_MOVE, OWN_CLOCK_SHORT]];
    for lines in positions {
        let Timed { engine, elapsed } = Timed::after(lines);
        assert!(engine.sink().answered(), "{lines:?}");
        assert!(elapsed < Duration::from_secs(1), "{lines:?}: {elapsed:?}");
    }
}

#[test]
fn a_search_past_half_its_allowance_starts_no_new_ply_and_a_clock_grants_a_share_of_what_it_holds()
{
    assert_eq!(Lagging::deepest_after(SHORT_OF_THE_LAG), Some(ONE_PLY));
    assert!(Lagging::deepest_after(MANY_MOVES_OF_LAG) > Some(ONE_PLY));
}
