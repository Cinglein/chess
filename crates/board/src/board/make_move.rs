use super::Board;
use crate::castling_right::CastlingRight;
use crate::castling_side::CastlingSide;
use crate::chess_move::ChessMove;
use crate::color::Color;
use crate::file::File;
use crate::halfmove_clock::HalfmoveClock;
use crate::move_kind::MoveKind;
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::piece_placement::PiecePlacement;
use crate::square::Square;

impl Board {
    #[must_use]
    pub fn make_move(self, chess_move: ChessMove) -> Option<Board> {
        let from = chess_move.from();
        let to = chess_move.to();
        let piece = self
            .placement
            .piece_at(from)
            .filter(|piece| piece.color == self.side_to_move)?;
        let captured = self.captured_by(chess_move);
        let irreversible = piece.kind == PieceKind::Pawn || captured.is_some();
        Some(Board {
            placement: self.placement_after(chess_move, piece, captured)?,
            side_to_move: self.side_to_move.opposite(),
            castling_rights: self
                .castling_rights
                .without_touching(from)
                .without_touching(to),
            en_passant_file: Self::double_push_file(piece, from, to),
            halfmove_clock: if irreversible {
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

    fn captured_by(&self, chess_move: ChessMove) -> Option<(Piece, Square)> {
        let square = match chess_move.kind() {
            MoveKind::EnPassant => Square::new(chess_move.to().file(), chess_move.from().rank()),
            _ => chess_move.to(),
        };
        self.placement
            .piece_at(square)
            .filter(|piece| piece.color != self.side_to_move)
            .map(|piece| (piece, square))
    }

    fn placement_after(
        &self,
        chess_move: ChessMove,
        piece: Piece,
        captured: Option<(Piece, Square)>,
    ) -> Option<PiecePlacement> {
        let arriving = chess_move
            .promotion_piece()
            .map_or(piece, |promotion| Piece::new(piece.color, promotion.into()));
        let placement = captured
            .map_or(self.placement, |(captured, square)| {
                self.placement.without(captured, square)
            })
            .without(piece, chess_move.from())
            .with(arriving, chess_move.to());
        match chess_move.kind() {
            MoveKind::Castling => {
                let side = CastlingSide::from_king_destination_file(chess_move.to().file())?;
                let right = CastlingRight::new(piece.color, side);
                let rook = Piece::new(piece.color, PieceKind::Rook);
                Some(
                    placement
                        .without(rook, right.rook_square())
                        .with(rook, right.rook_destination()),
                )
            }
            _ => Some(placement),
        }
    }

    fn double_push_file(piece: Piece, from: Square, to: Square) -> Option<File> {
        let forward = piece.color.pawn_push_direction();
        (piece.kind == PieceKind::Pawn && (from + forward) + forward == Some(to))
            .then_some(from.file())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest::sample::select;
    use strum::VariantArray;

    use super::Board;
    use crate::chess_move::ChessMove;
    use crate::color::Color;
    use crate::promotion::Promotion;
    use crate::square::Square;

    const TRANSITIONS: [(&str, ChessMove, &str); 7] = [
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ChessMove::normal(Square::E2, Square::E4),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        ),
        (
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            ChessMove::normal(Square::G8, Square::F6),
            "rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            ChessMove::castling(Square::E1, Square::G1),
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
            ChessMove::castling(Square::E8, Square::C8),
            "2kr3r/8/8/8/8/8/8/R4RK1 w - - 2 2",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            ChessMove::normal(Square::A1, Square::A8),
            "R3k2r/8/8/8/8/8/8/4K2R b Kk - 0 1",
        ),
        (
            "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
            ChessMove::en_passant(Square::E5, Square::D6),
            "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3",
        ),
        (
            "r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1",
            ChessMove::promotion(Square::B7, Square::A8, Promotion::Queen),
            "Q3k3/8/8/8/8/8/8/4K3 b - - 0 1",
        ),
    ];

    #[test]
    fn making_a_move_produces_the_position_fen_describes() {
        for (before, chess_move, after) in TRANSITIONS {
            let board: Board = before.parse().unwrap();
            let played = board.make_move(chess_move).map(|board| board.to_string());
            assert_eq!(played.as_deref(), Some(after), "{before} {chess_move}");
        }
    }

    #[test]
    fn a_move_is_applied_exactly_when_the_side_to_move_owns_the_origin() {
        proptest!(|(from in select(Square::VARIANTS), to in select(Square::VARIANTS))| {
            let owned = Board::START.placement().occupied_by(Color::White).contains(from);
            let played = Board::START.make_move(ChessMove::normal(from, to));
            prop_assert_eq!(played.is_some(), owned);
        });
    }
}
