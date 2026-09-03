use crate::bitboard::Bitboard;
use crate::square::Square;

type Step = (isize, isize);

const KNIGHT_STEPS: [Step; 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

const KING_STEPS: [Step; 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

const WHITE_PAWN_STEPS: [Step; 2] = [(-1, 1), (1, 1)];
const BLACK_PAWN_STEPS: [Step; 2] = [(-1, -1), (1, -1)];

pub(super) static KNIGHT: [Bitboard; Square::COUNT] = leaper_table(&KNIGHT_STEPS);
pub(super) static KING: [Bitboard; Square::COUNT] = leaper_table(&KING_STEPS);
pub(super) static PAWN: [[Bitboard; Square::COUNT]; 2] = [
    leaper_table(&WHITE_PAWN_STEPS),
    leaper_table(&BLACK_PAWN_STEPS),
];

const fn leaper_table(steps: &[Step]) -> [Bitboard; Square::COUNT] {
    let mut table = [Bitboard::EMPTY; Square::COUNT];
    let mut index = 0;
    while index < Square::COUNT {
        table[index] = leaper_attacks(Square::ALL[index], steps);
        index += 1;
    }
    table
}

const fn leaper_attacks(square: Square, steps: &[Step]) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let mut index = 0;
    while index < steps.len() {
        let (file_delta, rank_delta) = steps[index];
        if let Some(target) = square.translate(file_delta, rank_delta) {
            attacks = attacks.with(target);
        }
        index += 1;
    }
    attacks
}
