use board::Color;
use tally::{Tally, Verdict};
const WHITE_WIN: Verdict = Verdict::Win(Color::White);
const BLACK_WIN: Verdict = Verdict::Win(Color::Black);
const TALLIED: [(&[(Verdict, Verdict)], &str); 4] = [
    (
        &[(WHITE_WIN, BLACK_WIN); 4],
        "+8 =0 -0 8/8 Elo +1320 (+1320 to +1320) decisive",
    ),
    (
        &[(BLACK_WIN, WHITE_WIN); 4],
        "+0 =0 -8 0/8 Elo -1320 (-1320 to -1320) decisive",
    ),
    (
        &[
            (WHITE_WIN, BLACK_WIN),
            (BLACK_WIN, WHITE_WIN),
            (WHITE_WIN, WHITE_WIN),
            (BLACK_WIN, BLACK_WIN),
        ],
        "+4 =0 -4 4/8 Elo +0 (-297 to +297) inconclusive",
    ),
    (
        &[(WHITE_WIN, BLACK_WIN), (WHITE_WIN, Verdict::Draw)],
        "+3 =1 -0 3.5/4 Elo +338 (+149 to +1320) decisive",
    ),
];

#[test]
fn a_tally_counts_from_the_challenger_side_and_its_elo_bound_is_decisive_only_away_from_even() {
    for (pairs, written) in TALLIED {
        let tally = pairs
            .iter()
            .fold(Tally::default(), |tally, (as_white, as_black)| {
                tally.recorded_pair(*as_white, *as_black)
            });
        assert_eq!(tally.to_string(), written);
    }
}
