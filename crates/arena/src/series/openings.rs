use board::Board;

pub struct Openings;

impl Openings {
    const FENS: [&str; 8] = [
        "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4",
        "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4",
        "rnbqkb1r/pp2pppp/3p1n2/8/3NP3/8/PPP2PPP/RNBQKB1R w KQkq - 1 5",
        "rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 2 4",
        "rn1qkbnr/pp2pppp/2p5/5b2/3PN3/8/PPP2PPP/R1BQKBNR w KQkq - 1 5",
        "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 2 4",
        "rnbqk2r/ppppppbp/5np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 2 4",
        "rnbqkb1r/ppp2ppp/5n2/3pp3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq d6 0 4",
    ];

    pub fn boards() -> impl Iterator<Item = Board> {
        Self::FENS.iter().filter_map(|fen| fen.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use board::Board;

    use super::Openings;

    #[test]
    fn every_opening_parses_and_offers_a_legal_move_from_the_side_to_move() {
        let boards: Vec<Board> = Openings::boards().collect();
        assert_eq!(boards.len(), Openings::FENS.len());
        assert!(boards.iter().all(|board| !board.legal_moves().is_empty()));
    }
}
