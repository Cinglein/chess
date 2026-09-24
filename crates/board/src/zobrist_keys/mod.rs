mod split_mix;

use enum_map::EnumMap;
use strum::{EnumCount, VariantArray};

use crate::bitboard::Bitboard;
use crate::castling_right::CastlingRight;
use crate::castling_rights::CastlingRights;
use crate::color::Color;
use crate::file::File;
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::square::Square;
use crate::zobrist::Zobrist;
use split_mix::SplitMix64;

type SquareKeys = [Zobrist; Square::COUNT];

pub(crate) struct ZobristKeys {
    pieces: [[SquareKeys; PieceKind::COUNT]; Color::COUNT],
    castling: EnumMap<CastlingRight, Zobrist>,
    en_passant: EnumMap<File, Zobrist>,
    black_to_move: Zobrist,
}

impl ZobristKeys {
    const SEED: u64 = 0x0C4E_55B0_A4D6_4C10;
    pub(crate) const KEYS: ZobristKeys = Self::generate();

    pub(crate) const fn piece(&self, piece: Piece, square: Square) -> Zobrist {
        self.pieces[piece.color() as usize][piece.kind() as usize][square as usize]
    }

    pub(crate) fn castling(&self, rights: CastlingRights) -> Zobrist {
        CastlingRight::VARIANTS
            .iter()
            .filter(|right| rights.contains(**right))
            .fold(Zobrist::EMPTY, |hash, right| hash ^ self.castling[*right])
    }

    pub(crate) fn en_passant(&self, file: Option<File>) -> Zobrist {
        file.map_or(Zobrist::EMPTY, |file| self.en_passant[file])
    }

    pub(crate) const fn side_to_move(&self, color: Color) -> Zobrist {
        match color {
            Color::White => Zobrist::EMPTY,
            Color::Black => self.black_to_move,
        }
    }

    pub(crate) const fn hash_of(
        &self,
        pieces: &EnumMap<Color, EnumMap<PieceKind, Bitboard>>,
    ) -> Zobrist {
        let mut hash = Zobrist::EMPTY;
        let mut colors = Color::VARIANTS;
        while let [color, rest @ ..] = colors {
            let mut kinds = PieceKind::VARIANTS;
            while let [kind, tail @ ..] = kinds {
                let keys = &self.pieces[*color as usize][*kind as usize];
                let bitboard = pieces.as_array()[*color as usize].as_array()[*kind as usize];
                hash = hash.xor(Self::hash_bitboard(keys, bitboard));
                kinds = tail;
            }
            colors = rest;
        }
        hash
    }

    const fn hash_bitboard(keys: &SquareKeys, bitboard: Bitboard) -> Zobrist {
        let mut hash = Zobrist::EMPTY;
        let mut squares = Square::VARIANTS;
        while let [square, rest @ ..] = squares {
            if bitboard.contains(*square) {
                hash = hash.xor(keys[*square as usize]);
            }
            squares = rest;
        }
        hash
    }

    const fn generate() -> ZobristKeys {
        let mut generator = SplitMix64::new(Self::SEED);
        let mut pieces = [[[Zobrist::EMPTY; Square::COUNT]; PieceKind::COUNT]; Color::COUNT];
        let mut colors: &mut [[SquareKeys; PieceKind::COUNT]] = &mut pieces;
        while let [color, rest @ ..] = colors {
            generator = Self::fill_kinds(generator, color);
            colors = rest;
        }
        let mut castling = [Zobrist::EMPTY; CastlingRight::VARIANTS.len()];
        generator = Self::fill(generator, &mut castling);
        let mut en_passant = [Zobrist::EMPTY; File::COUNT];
        generator = Self::fill(generator, &mut en_passant);
        let generator = generator.next();
        ZobristKeys {
            pieces,
            castling: EnumMap::from_array(castling),
            en_passant: EnumMap::from_array(en_passant),
            black_to_move: Zobrist::from_bits(generator.output()),
        }
    }

    const fn fill_kinds(generator: SplitMix64, kinds: &mut [SquareKeys]) -> SplitMix64 {
        let mut generator = generator;
        let mut kinds = kinds;
        while let [kind, rest @ ..] = kinds {
            generator = Self::fill(generator, kind);
            kinds = rest;
        }
        generator
    }

    const fn fill(generator: SplitMix64, keys: &mut [Zobrist]) -> SplitMix64 {
        let mut generator = generator;
        let mut keys = keys;
        while let [key, rest @ ..] = keys {
            generator = generator.next();
            *key = Zobrist::from_bits(generator.output());
            keys = rest;
        }
        generator
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use strum::{IntoEnumIterator, VariantArray};

    use super::ZobristKeys;
    use crate::castling_right::CastlingRight;
    use crate::color::Color;
    use crate::file::File;
    use crate::piece::Piece;
    use crate::piece_kind::PieceKind;
    use crate::square::Square;
    use crate::zobrist::Zobrist;

    #[test]
    fn every_key_is_distinct_and_nonzero() {
        let keys = &ZobristKeys::KEYS;
        let pieces = Color::iter().flat_map(|color| {
            PieceKind::iter().flat_map(move |kind| {
                Square::iter().map(move |square| keys.piece(Piece::new(color, kind), square))
            })
        });
        let rights = CastlingRight::VARIANTS
            .iter()
            .map(|right| keys.castling(right.to_string().parse().unwrap()));
        let files = File::iter().map(|file| keys.en_passant(Some(file)));
        let all: Vec<u64> = pieces
            .chain(rights)
            .chain(files)
            .chain([keys.side_to_move(Color::Black)])
            .map(Zobrist::bits)
            .collect();
        let distinct: HashSet<u64> = all.iter().copied().collect();
        assert_eq!(distinct.len(), all.len());
        assert!(!distinct.contains(&0));
    }
}
