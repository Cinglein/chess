pub struct PerftPosition {
    fen: &'static str,
    nodes: [u64; 4],
}

impl PerftPosition {
    pub const REFERENCE: [PerftPosition; 6] = [
        PerftPosition {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            nodes: [20, 400, 8_902, 197_281],
        },
        PerftPosition {
            fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            nodes: [48, 2_039, 97_862, 4_085_603],
        },
        PerftPosition {
            fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            nodes: [14, 191, 2_812, 43_238],
        },
        PerftPosition {
            fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            nodes: [6, 264, 9_467, 422_333],
        },
        PerftPosition {
            fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            nodes: [44, 1_486, 62_379, 2_103_487],
        },
        PerftPosition {
            fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            nodes: [46, 2_079, 89_890, 3_894_594],
        },
    ];

    #[must_use]
    pub const fn fen(&self) -> &'static str {
        self.fen
    }

    pub fn expected(&self) -> impl Iterator<Item = (u8, u64)> + '_ {
        (1..).zip(self.nodes.iter().copied())
    }
}

#[cfg(test)]
mod tests {
    use super::PerftPosition;
    use crate::board::Board;

    #[test]
    fn shallow_perft_counts_match_the_reference_positions() {
        for position in &PerftPosition::REFERENCE {
            let board: Board = position.fen().parse().unwrap();
            for (depth, nodes) in position.expected().filter(|(depth, _)| *depth <= 3) {
                assert_eq!(
                    board.perft(depth),
                    nodes,
                    "{} depth {depth}",
                    position.fen()
                );
            }
        }
    }

    #[test]
    #[ignore = "millions of nodes per position; run with --ignored"]
    fn deep_perft_counts_match_the_reference_positions() {
        for position in &PerftPosition::REFERENCE {
            let board: Board = position.fen().parse().unwrap();
            for (depth, nodes) in position.expected() {
                assert_eq!(
                    board.perft(depth),
                    nodes,
                    "{} depth {depth}",
                    position.fen()
                );
            }
        }
    }
}
