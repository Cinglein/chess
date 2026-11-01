use eval::Score;
use game::Opponent;
use opponent_ports::InProcessEngine;
use search::Depth;
use uci::{GoLimits, Position};
const MATE_IN_TWO: &str = "7k/8/8/8/8/8/R7/1R4K1 w - - 0 1";
const SHALLOW: GoLimits = GoLimits::Depth(Depth::new(1));
const DEEP: GoLimits = GoLimits::Depth(Depth::new(4));

struct Fixture;

impl Fixture {
    fn scored(engine: &mut InProcessEngine, fen: &str, limits: GoLimits) -> Option<Score> {
        engine
            .choose_move(Position::played(fen.parse().unwrap(), &[]), limits)
            .unwrap()
            .score()
    }
}

#[test]
fn a_new_game_forgets_the_table_so_a_shallow_search_scores_like_a_fresh_engine() {
    let mut warmed = InProcessEngine::default();
    Fixture::scored(&mut warmed, MATE_IN_TWO, DEEP);
    warmed.begin_game().unwrap();
    let mut fresh = InProcessEngine::default();
    assert_eq!(
        Fixture::scored(&mut warmed, MATE_IN_TWO, SHALLOW),
        Fixture::scored(&mut fresh, MATE_IN_TWO, SHALLOW)
    );
}
