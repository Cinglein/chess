use board::LongAlgebraic;
use eval::Score;
use proptest::prelude::*;
use uci_response::{Response, SearchInfo};
const STOCKFISH_INFO: &str =
    "info depth 1 seldepth 0 multipv 1 score cp 0 nodes 0 nps 0 hashfull 0 tbhits 0 time 1 pv ";
const STOCKFISH_INFO_KEPT: &str = "info depth 1 score cp 0 nodes 0 nps 0 time 1";
const REPORT: &str = "info depth 5 score mate 1 nodes 66 nps 33000 time 2 pv a1a8";
const MATED: &str = "info depth 1 score mate 0 nodes 0 nps 0 time 0";
const PARSED: [(&str, Result<&str, &str>); 10] = [
    ("id name chess", Ok("id name chess")),
    ("id author Cinglein", Ok("id author Cinglein")),
    ("uciok", Ok("uciok")),
    ("readyok", Ok("readyok")),
    (REPORT, Ok(REPORT)),
    (STOCKFISH_INFO, Ok(STOCKFISH_INFO_KEPT)),
    ("bestmove (none) ponder e7e5", Ok("bestmove 0000")),
    (
        "info string Using 1 thread",
        Err("info needs a depth and a score"),
    ),
    (
        "Stockfish 19 by the Stockfish developers",
        Err("unknown response"),
    ),
    (
        "bestmove zz",
        Err("bestmove needs a move in long algebraic notation"),
    ),
];

#[test]
fn any_search_report_or_best_move_prints_back_to_itself_and_is_seen_through_its_view() {
    proptest!(|(info: SearchInfo, notation: Option<LongAlgebraic>)| {
        let (report, best) = (Response::Info(info), Response::BestMove(notation));
        for response in [report, best] {
            let line = response.to_string();
            prop_assert_eq!(Response::try_from(line.as_str()), Ok(response));
        }
        prop_assert_eq!(report.info(), Some(info));
        prop_assert_eq!((best.best_move(), report.best_move()), (Some(notation), None));
    });
}

#[test]
fn every_response_prints_as_the_line_the_arena_keeps_and_lines_without_a_search_are_rejected() {
    for (line, kept) in PARSED {
        let printed = Response::try_from(line)
            .map(|response| response.to_string())
            .map_err(|error| error.to_string());
        assert_eq!(printed.as_deref().map_err(String::as_str), kept, "{line}");
    }
    let mated = Response::try_from(MATED).unwrap().info();
    assert_eq!(mated.map(|info| info.score()), Some(-Score::mate_in(0)));
}
