use enum_map::EnumMap;
use strum::{EnumCount, VariantArray};

use crate::bitboard::Bitboard;
use crate::castling_right::CastlingRight;
use crate::castling_rights::CastlingRights;
use crate::color::Color;
use crate::file::File;
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::split_mix::SplitMix64;
use crate::square::Square;
use crate::zobrist::Zobrist;

pub struct ZobristKeys {
    pieces: EnumMap<Color, EnumMap<PieceKind, EnumMap<Square, Zobrist>>>,
    castling: EnumMap<CastlingRight, Zobrist>,
    en_passant: EnumMap<File, Zobrist>,
    black_to_move: Zobrist,
}

impl ZobristKeys {
    const SEED: u64 = 0x0C4E_55B0_A4D6_4C10;
    pub const KEYS: ZobristKeys = Self::generate(SplitMix64::new(Self::SEED));

    #[must_use]
    pub fn piece(&self, piece: Piece, square: Square) -> Zobrist {
        self.pieces[piece.color()][piece.kind()][square]
    }

    #[must_use]
    pub fn castling(&self, rights: CastlingRights) -> Zobrist {
        CastlingRight::VARIANTS
            .iter()
            .filter(|right| rights.contains(**right))
            .fold(Zobrist::EMPTY, |hash, right| hash ^ self.castling[*right])
    }

    #[must_use]
    pub fn en_passant(&self, file: Option<File>) -> Zobrist {
        file.map_or(Zobrist::EMPTY, |file| self.en_passant[file])
    }

    #[must_use]
    pub const fn side_to_move(&self, color: Color) -> Zobrist {
        match color {
            Color::White => Zobrist::EMPTY,
            Color::Black => self.black_to_move,
        }
    }

    #[must_use]
    pub const fn hash_of(&self, pieces: &EnumMap<Color, EnumMap<PieceKind, Bitboard>>) -> Zobrist {
        let mut hash = Zobrist::EMPTY;
        let mut color = 0;
        while color < Color::COUNT {
            let mut kind = 0;
            while kind < PieceKind::COUNT {
                let bitboard = pieces.as_array()[color].as_array()[kind];
                let keys = self.pieces.as_array()[color].as_array()[kind].as_array();
                let mut square = 0;
                while square < Square::COUNT {
                    if bitboard.contains(Square::VARIANTS[square]) {
                        hash = hash.xor(keys[square]);
                    }
                    square += 1;
                }
                kind += 1;
            }
            color += 1;
        }
        hash
    }

    const fn generate(generator: SplitMix64) -> ZobristKeys {
        let mut generator = generator;
        let mut colors = [const {
            EnumMap::from_array(
                [const { EnumMap::from_array([Zobrist::EMPTY; Square::COUNT]) }; PieceKind::COUNT],
            )
        }; Color::COUNT];
        let mut color = 0;
        while color < Color::COUNT {
            let mut kinds =
                [const { EnumMap::from_array([Zobrist::EMPTY; Square::COUNT]) }; PieceKind::COUNT];
            let mut kind = 0;
            while kind < PieceKind::COUNT {
                let (next, keys) = Self::fill::<{ Square::COUNT }>(generator);
                generator = next;
                kinds[kind] = EnumMap::from_array(keys);
                kind += 1;
            }
            colors[color] = EnumMap::from_array(kinds);
            color += 1;
        }
        let (generator, castling) = Self::fill::<{ CastlingRight::VARIANTS.len() }>(generator);
        let (generator, en_passant) = Self::fill::<{ File::COUNT }>(generator);
        let (_, black_to_move) = generator.next();
        ZobristKeys {
            pieces: EnumMap::from_array(colors),
            castling: EnumMap::from_array(castling),
            en_passant: EnumMap::from_array(en_passant),
            black_to_move: Zobrist::from_bits(black_to_move),
        }
    }

    const fn fill<const COUNT: usize>(generator: SplitMix64) -> (SplitMix64, [Zobrist; COUNT]) {
        let mut generator = generator;
        let mut keys = [Zobrist::EMPTY; COUNT];
        let mut index = 0;
        while index < COUNT {
            let (next, key) = generator.next();
            generator = next;
            keys[index] = Zobrist::from_bits(key);
            index += 1;
        }
        (generator, keys)
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
