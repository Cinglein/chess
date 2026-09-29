use crate::node_count::NodeCount;

pub struct PerftPosition {
    fen: &'static str,
    nodes: [NodeCount; 4],
}

impl PerftPosition {
    pub const REFERENCE: [PerftPosition; 6] = [
        PerftPosition {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            nodes: [
                NodeCount::new(20),
                NodeCount::new(400),
                NodeCount::new(8_902),
                NodeCount::new(197_281),
            ],
        },
        PerftPosition {
            fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            nodes: [
                NodeCount::new(48),
                NodeCount::new(2_039),
                NodeCount::new(97_862),
                NodeCount::new(4_085_603),
            ],
        },
        PerftPosition {
            fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            nodes: [
                NodeCount::new(14),
                NodeCount::new(191),
                NodeCount::new(2_812),
                NodeCount::new(43_238),
            ],
        },
        PerftPosition {
            fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            nodes: [
                NodeCount::new(6),
                NodeCount::new(264),
                NodeCount::new(9_467),
                NodeCount::new(422_333),
            ],
        },
        PerftPosition {
            fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            nodes: [
                NodeCount::new(44),
                NodeCount::new(1_486),
                NodeCount::new(62_379),
                NodeCount::new(2_103_487),
            ],
        },
        PerftPosition {
            fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            nodes: [
                NodeCount::new(46),
                NodeCount::new(2_079),
                NodeCount::new(89_890),
                NodeCount::new(3_894_594),
            ],
        },
    ];

    #[must_use]
    pub const fn fen(&self) -> &'static str {
        self.fen
    }

    pub fn nodes_by_depth(&self) -> impl Iterator<Item = NodeCount> + '_ {
        self.nodes.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeCount, PerftPosition};
    use crate::board::Board;

    impl Board {
        fn perft(self, depth: u8) -> NodeCount {
            let Some(remaining) = depth.checked_sub(1) else {
                return NodeCount::ONE;
            };
            let moves = self.legal_moves();
            if remaining == 0 {
                return NodeCount::new(u64::try_from(moves.len()).unwrap_or(u64::MAX));
            }
            moves
                .into_iter()
                .filter_map(|chess_move| self.make_move(chess_move))
                .map(|board| board.perft(remaining))
                .sum()
        }
    }

    #[test]
    fn shallow_perft_counts_match_the_reference_positions() {
        for position in &PerftPosition::REFERENCE {
            let board: Board = position.fen().parse().unwrap();
            for (depth, nodes) in (1..).zip(position.nodes_by_depth()).take(3) {
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
            for (depth, nodes) in (1..).zip(position.nodes_by_depth()) {
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
