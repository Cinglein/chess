use board::ChessMove;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Killers {
    first: Option<ChessMove>,
    second: Option<ChessMove>,
}

impl Killers {
    pub(crate) const NONE: Killers = Killers {
        first: None,
        second: None,
    };

    pub(crate) fn remembers(self, chess_move: ChessMove) -> bool {
        self.first == Some(chess_move) || self.second == Some(chess_move)
    }

    pub(crate) fn remembering(self, chess_move: ChessMove) -> Killers {
        if self.first == Some(chess_move) {
            self
        } else {
            Killers {
                first: Some(chess_move),
                second: self.first,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use board::Board;

    use super::Killers;

    #[test]
    fn the_two_latest_distinct_moves_are_remembered_and_the_newest_is_not_pushed_down_by_itself() {
        let [first, second, third, ..] = Board::START.legal_moves()[..] else {
            panic!()
        };
        let killers = Killers::NONE.remembering(first).remembering(second);
        assert!(killers.remembers(first) && killers.remembers(second) && !killers.remembers(third));
        assert_eq!(killers.remembering(second), killers);
        assert!(!killers.remembering(third).remembers(first));
    }
}
