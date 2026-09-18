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

pub struct MoveGenerator<'a, P: Pawn> {
    board: &'a Board,
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

impl<'a, P: Pawn> MoveGenerator<'a, P> {
    #[must_use]
    pub fn new(board: &'a Board) -> Option<Self> {
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
            targets: Self::check_targets(king, checkers, ours),
            moves: MoveList::new(),
            side: PhantomData,
        })
    }

    #[must_use]
    pub fn legal_moves(mut self) -> MoveList {
        let queens = self.ours_of(PieceKind::Queen);
        let knights = self.ours_of(PieceKind::Knight);
        let bishops = self.ours_of(PieceKind::Bishop) | queens;
        let rooks = self.ours_of(PieceKind::Rook) | queens;
        self.pawns();
        self.leapers::<Knight>(knights);
        self.sliders::<Bishop>(bishops);
        self.sliders::<Rook>(rooks);
        self.king_moves();
        self.moves
    }

    fn pawns(&mut self) {
        for from in self.ours_of(PieceKind::Pawn) {
            let allowed = self.targets & self.pin_line(from);
            if let Some(to) = (from + P::PUSH).filter(|to| !self.occupied.contains(*to)) {
                if allowed.contains(to) {
                    self.pawn_advance(from, to);
                }
                if from.rank() == P::START_RANK
                    && let Some(double) = (to + P::PUSH).filter(|double| {
                        !self.occupied.contains(*double) && allowed.contains(*double)
                    })
                {
                    self.moves
                        .push(ChessMove::DoublePush(DoublePush::new(from, double)));
                }
            }
            for to in P::attacks(from) & self.theirs & allowed {
                self.pawn_advance(from, to);
            }
            if let Some(target) = self.board.en_passant_square()
                && P::attacks(from).contains(target)
            {
                self.en_passant(from, target);
            }
        }
    }

    fn pawn_advance(&mut self, from: Square, to: Square) {
        if to.rank() == P::PROMOTION_RANK {
            self.moves.extend(
                PromotionPiece::iter()
                    .map(|piece| ChessMove::Promotion(Promotion::new(from, to, piece))),
            );
        } else {
            self.moves.push(ChessMove::Normal(Normal::new(from, to)));
        }
    }

    fn en_passant(&mut self, from: Square, to: Square) {
        let chess_move = ChessMove::EnPassant(EnPassant::new(from, to));
        let safe = chess_move
            .play(*self.board.placement())
            .is_some_and(|after| Self::attackers(&after, self.king, after.occupied()).is_empty());
        if safe {
            self.moves.push(chess_move);
        }
    }

    fn leapers<L: Leaper>(&mut self, pieces: Bitboard) {
        for from in pieces {
            self.piece_moves(from, L::attacks(from));
        }
    }

    fn sliders<S: Slider>(&mut self, pieces: Bitboard) {
        for from in pieces {
            self.piece_moves(from, S::attacks(from, self.occupied));
        }
    }

    fn piece_moves(&mut self, from: Square, attacks: Bitboard) {
        for to in attacks & self.targets & self.pin_line(from) {
            self.moves.push(ChessMove::Normal(Normal::new(from, to)));
        }
    }

    fn king_moves(&mut self) {
        let without_king = self.occupied ^ Bitboard::from_square(self.king);
        for to in King::attacks(self.king) & !self.ours {
            if Self::attackers(self.board.placement(), to, without_king).is_empty() {
                self.moves
                    .push(ChessMove::Normal(Normal::new(self.king, to)));
            }
        }
        if self.checkers.is_empty() {
            for right in CastlingRight::VARIANTS.iter().copied() {
                if right.color() == P::COLOR
                    && self.board.castling_rights().contains(right)
                    && self.may_castle(CastlingSquares::new(right))
                {
                    self.moves.push(ChessMove::Castling(Castling::new(right)));
                }
            }
        }
    }

    fn may_castle(&self, squares: CastlingSquares) -> bool {
        let rook_path = squares.king_origin().between(squares.rook_origin());
        let king_path = squares.king_origin().between(squares.king_destination())
            | Bitboard::from_square(squares.king_destination());
        self.king == squares.king_origin()
            && self
                .ours_of(PieceKind::Rook)
                .contains(squares.rook_origin())
            && (rook_path & self.occupied).is_empty()
            && king_path.into_iter().all(|square| {
                Self::attackers(self.board.placement(), square, self.occupied).is_empty()
            })
    }

    fn ours_of(&self, kind: PieceKind) -> Bitboard {
        self.board.placement().pieces(P::COLOR, kind)
    }

    fn pin_line(&self, from: Square) -> Bitboard {
        if self.pinned.contains(from) {
            self.king.line_through(from)
        } else {
            Bitboard::FULL
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

    fn check_targets(king: Square, checkers: Bitboard, ours: Bitboard) -> Bitboard {
        match (checkers.count(), checkers.least_significant_bit()) {
            (0, _) => !ours,
            (1, Some(checker)) => king.between(checker) | checkers,
            _ => Bitboard::EMPTY,
        }
    }
}
