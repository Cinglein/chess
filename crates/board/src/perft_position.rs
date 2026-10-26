use crate::node_count::NodeCount;

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
}

#[cfg(test)]
mod tests {
    use super::{NodeCount, PerftPosition};
    use crate::board::Board;
    impl Board {
        fn perft(self, depth: u8) -> NodeCount {
            let moves = self.legal_moves();
            match depth {
                1 => NodeCount::new(u64::try_from(moves.len()).unwrap()),
                _ => moves
                    .into_iter()
                    .filter_map(|chess_move| self.make_move(chess_move))
                    .map(|board| board.perft(depth - 1))
                    .sum(),
            }
        }
    }

    impl PerftPosition {
        fn first_mismatch(depth_limit: usize) -> Option<String> {
            Self::REFERENCE.iter().find_map(|position| {
                let board: Board = position.fen.parse().unwrap();
                (1..)
                    .zip(position.nodes)
                    .take(depth_limit)
                    .find(|(depth, nodes)| board.perft(*depth) != NodeCount::new(*nodes))
                    .map(|(depth, _)| format!("{} depth {depth}", position.fen))
            })
        }
    }

    #[test]
    fn shallow_perft_counts_match_the_reference_positions() {
        assert_eq!(PerftPosition::first_mismatch(3), None);
    }

    #[test]
    #[ignore = "millions of nodes per position; run with --ignored"]
    fn deep_perft_counts_match_the_reference_positions() {
        assert_eq!(PerftPosition::first_mismatch(4), None);
    }
}
