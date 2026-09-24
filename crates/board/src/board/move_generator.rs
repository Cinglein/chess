use core::marker::PhantomData;

use strum::{IntoEnumIterator, VariantArray};

use super::{Board, MoveList};
use crate::bitboard::Bitboard;
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
        let queens = ours(PieceKind::Queen);
        let pawns = ours(PieceKind::Pawn);
        let knights = ours(PieceKind::Knight);
        let bishops = ours(PieceKind::Bishop) | queens;
        let rooks = ours(PieceKind::Rook) | queens;
        let occupied = self.occupied;
        for from in pawns {
            self.pawn_moves(from);
        }
        self.pieces(knights, Knight::attacks);
        self.pieces(bishops, |from| Bishop::attacks(from, occupied));
        self.pieces(rooks, |from| Rook::attacks(from, occupied));
        self.king_moves();
        self.moves
    }

    fn pawn_moves(&mut self, from: Square) {
        let allowed = self.safety.allowed(from);
        let push = (from + P::PUSH).filter(|to| !self.occupied.contains(*to));
        let double = push
            .filter(|_| from.rank() == P::START_RANK)
            .and_then(|to| to + P::PUSH)
            .filter(|to| !self.occupied.contains(*to) && allowed.contains(*to));
        let single = push.map_or(Bitboard::EMPTY, Bitboard::from_square);
        let destinations = (single | (P::attacks(from) & self.theirs)) & allowed;
        let promoting = destinations & Bitboard::rank(P::PROMOTION_RANK);
        let en_passant = self
            .board
            .en_passant_square()
            .filter(|target| P::attacks(from).contains(*target))
            .map(|target| ChessMove::EnPassant(EnPassant::new(from, target)))
            .filter(|chess_move| {
                chess_move
                    .play(*self.board.placement())
                    .is_some_and(|after| {
                        after
                            .attackers(self.safety.king(), !P::COLOR, after.occupied())
                            .is_empty()
                    })
            });
        self.moves.extend(
            (destinations & !promoting)
                .into_iter()
                .map(|to| ChessMove::Normal(Normal::new(from, to))),
        );
        self.moves.extend(promoting.into_iter().flat_map(|to| {
            PromotionPiece::iter()
                .map(move |piece| ChessMove::Promotion(Promotion::new(from, to, piece)))
        }));
        self.moves
            .extend(double.map(|to| ChessMove::DoublePush(DoublePush::new(from, to))));
        self.moves.extend(en_passant);
    }

    fn pieces(&mut self, pieces: Bitboard, attacks: impl Fn(Square) -> Bitboard) {
        let safety = &self.safety;
        self.moves.extend(pieces.into_iter().flat_map(|from| {
            (attacks(from) & safety.allowed(from))
                .into_iter()
                .map(move |to| ChessMove::Normal(Normal::new(from, to)))
        }));
    }

    fn king_moves(&mut self) {
        let board = self.board;
        let safety = &self.safety;
        let occupied = self.occupied;
        let king = safety.king();
        let without_king = occupied ^ Bitboard::from_square(king);
        self.moves.extend(
            (King::attacks(king) & !self.ours)
                .into_iter()
                .filter(|to| {
                    board
                        .placement()
                        .attackers(*to, !P::COLOR, without_king)
                        .is_empty()
                })
                .map(|to| ChessMove::Normal(Normal::new(king, to))),
        );
        self.moves.extend(
            CastlingRight::VARIANTS
                .iter()
                .copied()
                .filter(|right| Self::may_castle(board, safety, occupied, *right))
                .map(|right| ChessMove::Castling(Castling::new(right))),
        );
    }

    fn may_castle(
        board: &Board,
        safety: &KingSafety,
        occupied: Bitboard,
        right: CastlingRight,
    ) -> bool {
        let placement = board.placement();
        let squares = CastlingSquares::new(right);
        let king_path = squares.king_origin().between(squares.king_destination())
            | Bitboard::from_square(squares.king_destination());
        !safety.in_check()
            && right.color() == P::COLOR
            && board.castling_rights().contains(right)
            && safety.king() == squares.king_origin()
            && placement
                .pieces(P::COLOR, PieceKind::Rook)
                .contains(squares.rook_origin())
            && (squares.king_origin().between(squares.rook_origin()) & occupied).is_empty()
            && king_path
                .into_iter()
                .all(|square| placement.attackers(square, !P::COLOR, occupied).is_empty())
    }
}
