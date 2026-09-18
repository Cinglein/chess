use core::marker::PhantomData;

use strum::{IntoEnumIterator, VariantArray};

use crate::bitboard::Bitboard;
use crate::board::{Board, MoveList};
use crate::castling_right::CastlingRight;
use crate::castling_squares::CastlingSquares;
use crate::chess_move::{Castling, ChessMove, DoublePush, EnPassant, MoveKind, Normal, Promotion};
use crate::leaper::{King, Knight, Leaper, Pawn};
use crate::piece_kind::PieceKind;
use crate::placement::PiecePlacement;
use crate::promotion_piece::PromotionPiece;
use crate::slider::{Bishop, Rook, Slider};
use crate::square::Square;

pub struct MoveGenerator<'board, P: Pawn> {
    board: &'board Board,
    king: Square,
    occupied: Bitboard,
    ours: Bitboard,
    theirs: Bitboard,
    checkers: Bitboard,
    pinned: Bitboard,
    targets: Bitboard,
    moves: MoveList,
    side: PhantomData<P>,
}

impl<'board, P: Pawn> MoveGenerator<'board, P> {
    #[must_use]
    pub fn new(board: &'board Board) -> Option<Self> {
        let placement = board.placement();
        let king = placement
            .pieces(P::COLOR, PieceKind::King)
            .least_significant_bit()?;
        let occupied = placement.occupied();
        let ours = placement.occupied_by(P::COLOR);
        let checkers = Self::attackers(placement, king, occupied);
        Some(MoveGenerator {
            board,
            king,
            occupied,
            ours,
            theirs: placement.occupied_by(!P::COLOR),
            checkers,
            pinned: Self::pins(placement, king, occupied, ours),
            targets: match checkers.least_significant_bit() {
                None => !ours,
                Some(checker) if checkers.count() == 1 => king.between(checker) | checkers,
                Some(_) => Bitboard::EMPTY,
            },
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
            let allowed = self.allowed(from);
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
                        Self::attackers(&after, self.king, after.occupied()).is_empty()
                    });
                if safe {
                    self.moves.push(en_passant);
                }
            }
        }
    }

    fn pieces(&mut self, pieces: Bitboard, attacks: impl Fn(Square) -> Bitboard) {
        for from in pieces {
            for to in attacks(from) & self.allowed(from) {
                self.moves.push(ChessMove::Normal(Normal::new(from, to)));
            }
        }
    }

    fn king_moves(&mut self) {
        let placement = self.board.placement();
        let without_king = self.occupied ^ Bitboard::from_square(self.king);
        for to in King::attacks(self.king) & !self.ours {
            if Self::attackers(placement, to, without_king).is_empty() {
                self.moves
                    .push(ChessMove::Normal(Normal::new(self.king, to)));
            }
        }
        if !self.checkers.is_empty() {
            return;
        }
        for right in CastlingRight::VARIANTS.iter().copied() {
            let squares = CastlingSquares::new(right);
            let king_path = squares.king_origin().between(squares.king_destination())
                | Bitboard::from_square(squares.king_destination());
            let may_castle = right.color() == P::COLOR
                && self.board.castling_rights().contains(right)
                && self.king == squares.king_origin()
                && placement
                    .pieces(P::COLOR, PieceKind::Rook)
                    .contains(squares.rook_origin())
                && (squares.king_origin().between(squares.rook_origin()) & self.occupied)
                    .is_empty()
                && king_path
                    .into_iter()
                    .all(|square| Self::attackers(placement, square, self.occupied).is_empty());
            if may_castle {
                self.moves.push(ChessMove::Castling(Castling::new(right)));
            }
        }
    }

    fn allowed(&self, from: Square) -> Bitboard {
        if self.pinned.contains(from) {
            self.targets & self.king.line_through(from)
        } else {
            self.targets
        }
    }

    fn attackers(placement: &PiecePlacement, square: Square, occupied: Bitboard) -> Bitboard {
        let theirs = |kind| placement.pieces(!P::COLOR, kind);
        let queens = theirs(PieceKind::Queen);
        (P::attacks(square) & theirs(PieceKind::Pawn))
            | (Knight::attacks(square) & theirs(PieceKind::Knight))
            | (King::attacks(square) & theirs(PieceKind::King))
            | (Bishop::attacks(square, occupied) & (theirs(PieceKind::Bishop) | queens))
            | (Rook::attacks(square, occupied) & (theirs(PieceKind::Rook) | queens))
    }

    fn pins(
        placement: &PiecePlacement,
        king: Square,
        occupied: Bitboard,
        ours: Bitboard,
    ) -> Bitboard {
        let theirs = |kind| placement.pieces(!P::COLOR, kind);
        let queens = theirs(PieceKind::Queen);
        let snipers = (Rook::attacks(king, Bitboard::EMPTY) & (theirs(PieceKind::Rook) | queens))
            | (Bishop::attacks(king, Bitboard::EMPTY) & (theirs(PieceKind::Bishop) | queens));
        snipers
            .into_iter()
            .map(|sniper| king.between(sniper) & occupied)
            .filter(|blockers| blockers.count() == 1 && !(*blockers & ours).is_empty())
            .fold(Bitboard::EMPTY, |pinned, blocker| pinned | blocker)
    }
}
