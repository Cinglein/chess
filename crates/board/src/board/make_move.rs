use super::Board;
use crate::chess_move::ChessMove;
use crate::color::Color;
use crate::file::File;
use crate::halfmove_clock::HalfmoveClock;
use crate::leaper::{BlackPawn, Pawn, WhitePawn};
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::piece_placement::PiecePlacement;
use crate::square::Square;

impl Board {
    #[must_use]
    pub fn make_move(self, chess_move: ChessMove) -> Option<Board> {
        let piece = self
            .placement
            .piece_at(chess_move.from())
            .filter(|piece| piece.color == self.side_to_move)?;
        let placement = self.placement_after(chess_move, piece);
        let captured = placement.occupied().count() < self.placement.occupied().count();
        Some(Board {
            placement,
            side_to_move: !self.side_to_move,
            castling_rights: self
                .castling_rights
                .without_touching(chess_move.from())
                .without_touching(chess_move.to()),
            en_passant_file: Self::double_push_file(piece, chess_move),
            halfmove_clock: if piece.kind == PieceKind::Pawn || captured {
                HalfmoveClock::ZERO
            } else {
                self.halfmove_clock.incremented()
            },
            fullmove_number: match self.side_to_move {
                Color::White => self.fullmove_number,
                Color::Black => self.fullmove_number.incremented(),
            },
        })
    }

    fn placement_after(&self, chess_move: ChessMove, piece: Piece) -> PiecePlacement {
        let lifted = self.placement.without(piece, chess_move.from());
        match chess_move {
            ChessMove::Normal { to, .. } => lifted.cleared(to).with(piece, to),
            ChessMove::Promotion {
                to,
                piece: promotion,
                ..
            } => lifted
                .cleared(to)
                .with(Piece::new(piece.color, promotion.into()), to),
            ChessMove::EnPassant { from, to } => lifted
                .cleared(Square::new(to.file(), from.rank()))
                .with(piece, to),
            ChessMove::Castling(right) => {
                let castling = right.castling();
                let rook = Piece::new(piece.color, PieceKind::Rook);
                lifted
                    .with(piece, castling.king_to)
                    .without(rook, castling.rook_from)
                    .with(rook, castling.rook_to)
            }
        }
    }

    fn double_push_file(piece: Piece, chess_move: ChessMove) -> Option<File> {
        let forward = match piece.color {
            Color::White => WhitePawn::PUSH,
            Color::Black => BlackPawn::PUSH,
        };
        let from = chess_move.from();
        (piece.kind == PieceKind::Pawn && (from + forward) + forward == Some(chess_move.to()))
            .then_some(from.file())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest::sample::select;
    use strum::VariantArray;

    use super::Board;
    use crate::castling_right::CastlingRight;
    use crate::chess_move::ChessMove;
    use crate::color::Color;
    use crate::promotion::Promotion;
    use crate::square::Square;

    const TRANSITIONS: [(&str, ChessMove, &str); 7] = [
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ChessMove::Normal {
                from: Square::E2,
                to: Square::E4,
            },
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        ),
        (
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            ChessMove::Normal {
                from: Square::G8,
                to: Square::F6,
            },
            "rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            ChessMove::Castling(CastlingRight::WhiteKingside),
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
            ChessMove::Castling(CastlingRight::BlackQueenside),
            "2kr3r/8/8/8/8/8/8/R4RK1 w - - 2 2",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            ChessMove::Normal {
                from: Square::A1,
                to: Square::A8,
            },
            "R3k2r/8/8/8/8/8/8/4K2R b Kk - 0 1",
        ),
        (
            "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
            ChessMove::EnPassant {
                from: Square::E5,
                to: Square::D6,
            },
            "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3",
        ),
        (
            "r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1",
            ChessMove::Promotion {
                from: Square::B7,
                to: Square::A8,
                piece: Promotion::Queen,
            },
            "Q3k3/8/8/8/8/8/8/4K3 b - - 0 1",
        ),
    ];

    #[test]
    fn making_a_move_produces_the_position_fen_describes() {
        for (before, chess_move, after) in TRANSITIONS {
            let board = before.parse::<Board>().unwrap().make_move(chess_move);
            let played = board.map(|board| board.to_string());
            assert_eq!(played.as_deref(), Some(after), "{before} {chess_move}");
        }
    }

    #[test]
    fn a_move_is_applied_exactly_when_the_side_to_move_owns_the_origin() {
        proptest!(|(from in select(Square::VARIANTS), to in select(Square::VARIANTS))| {
            let owned = Board::START.placement().occupied_by(Color::White).contains(from);
            prop_assert_eq!(Board::START.make_move(ChessMove::Normal { from, to }).is_some(), owned);
        });
    }
}
