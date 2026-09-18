use core::marker::PhantomData;

use strum::{IntoEnumIterator, VariantArray};

use crate::bitboard::Bitboard;
use crate::board::{Board, MoveList};
use crate::castling_right::CastlingRight;
use crate::castling_squares::CastlingSquares;
use crate::chess_move::{Castling, ChessMove, DoublePush, EnPassant, MoveKind, Normal, Promotion};
use crate::king_safety::KingSafety;
use crate::leaper::{King, Knight, Leaper, Pawn};
use crate::piece_kind::PieceKind;
use crate::promotion_piece::PromotionPiece;
use crate::slider::{Bishop, Rook, Slider};
use crate::square::Square;

pub struct MoveGenerator<'board, P: Pawn> {
    board: &'board Board,
    safety: KingSafety,
    occupied: Bitboard,
    ours: Bitboard,
    theirs: Bitboard,
    moves: MoveList,
    side: PhantomData<P>,
}

impl<'board, P: Pawn> MoveGenerator<'board, P> {
    #[must_use]
    pub fn new(board: &'board Board) -> Option<Self> {
        let placement = board.placement();
        Some(MoveGenerator {
            board,
            safety: KingSafety::new(placement, P::COLOR)?,
            occupied: placement.occupied(),
            ours: placement.occupied_by(P::COLOR),
            theirs: placement.occupied_by(!P::COLOR),
            moves: MoveList::new(),
            side: PhantomData,
        })
    }

    #[must_use]
    pub fn legal_moves(mut self) -> MoveList {
        let ours = |kind| self.board.placement().pieces(P::COLOR, kind);
        let (pawns, knights, queens) = (
            ours(PieceKind::Pawn),
            ours(PieceKind::Knight),
            ours(PieceKind::Queen),
        );
        let (bishops, rooks) = (
            ours(PieceKind::Bishop) | queens,
            ours(PieceKind::Rook) | queens,
        );
        let occupied = self.occupied;
        self.pawns(pawns);
        self.pieces(knights, Knight::attacks);
        self.pieces(bishops, |from| Bishop::attacks(from, occupied));
        self.pieces(rooks, |from| Rook::attacks(from, occupied));
        self.king_moves();
        self.moves
    }

    fn pawns(&mut self, pawns: Bitboard) {
        for from in pawns {
            let allowed = self.safety.allowed(from);
            let push = (from + P::PUSH).filter(|to| !self.occupied.contains(*to));
            let double = push
                .filter(|_| from.rank() == P::START_RANK)
                .and_then(|to| to + P::PUSH)
                .filter(|to| !self.occupied.contains(*to) && allowed.contains(*to));
            let single = push.map_or(Bitboard::EMPTY, Bitboard::from_square);
            for to in (single | (P::attacks(from) & self.theirs)) & allowed {
                if to.rank() == P::PROMOTION_RANK {
                    self.moves.extend(
                        PromotionPiece::iter()
                            .map(|piece| ChessMove::Promotion(Promotion::new(from, to, piece))),
                    );
                } else {
                    self.moves.push(ChessMove::Normal(Normal::new(from, to)));
                }
            }
            if let Some(to) = double {
                self.moves
                    .push(ChessMove::DoublePush(DoublePush::new(from, to)));
            }
            if let Some(target) = self.board.en_passant_square()
                && P::attacks(from).contains(target)
            {
                let en_passant = ChessMove::EnPassant(EnPassant::new(from, target));
                let safe = en_passant
                    .play(*self.board.placement())
                    .is_some_and(|after| {
                        after
                            .attackers(self.safety.king(), !P::COLOR, after.occupied())
                            .is_empty()
                    });
                if safe {
                    self.moves.push(en_passant);
                }
            }
        }
    }

    fn pieces(&mut self, pieces: Bitboard, attacks: impl Fn(Square) -> Bitboard) {
        for from in pieces {
            for to in attacks(from) & self.safety.allowed(from) {
                self.moves.push(ChessMove::Normal(Normal::new(from, to)));
            }
        }
    }

    fn king_moves(&mut self) {
        let placement = self.board.placement();
        let king = self.safety.king();
        let without_king = self.occupied ^ Bitboard::from_square(king);
        for to in King::attacks(king) & !self.ours {
            if placement.attackers(to, !P::COLOR, without_king).is_empty() {
                self.moves.push(ChessMove::Normal(Normal::new(king, to)));
            }
        }
        if self.safety.in_check() {
            return;
        }
        for right in CastlingRight::VARIANTS.iter().copied() {
            let squares = CastlingSquares::new(right);
            let king_path = squares.king_origin().between(squares.king_destination())
                | Bitboard::from_square(squares.king_destination());
            let may_castle = right.color() == P::COLOR
                && self.board.castling_rights().contains(right)
                && king == squares.king_origin()
                && placement
                    .pieces(P::COLOR, PieceKind::Rook)
                    .contains(squares.rook_origin())
                && (squares.king_origin().between(squares.rook_origin()) & self.occupied)
                    .is_empty()
                && king_path.into_iter().all(|square| {
                    placement
                        .attackers(square, !P::COLOR, self.occupied)
                        .is_empty()
                });
            if may_castle {
                self.moves.push(ChessMove::Castling(Castling::new(right)));
            }
        }
    }
}
