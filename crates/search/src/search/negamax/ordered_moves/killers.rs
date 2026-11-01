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
