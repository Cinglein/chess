const RANK_MASK: u64 = 0xFF;
const JOIN: SetOperation = SetOperation::Union;
const LEAVE: SetOperation = SetOperation::Difference;
use board::{Bitboard, SetOperation, Square, SubsetIter};
use proptest::prelude::*;
use std::collections::HashSet;
use strum::EnumCount;
#[test]
fn set_algebra_iteration_and_display_agree() {
    proptest!(|(left: Bitboard, right: Bitboard)| {
        let mut toggled = left;
        toggled ^= right;
        prop_assert_eq!(
            ((left | right).count() + (left & right).count(), toggled, Bitboard::from_bits(left.bits())),
            (left.count() + right.count(), left ^ right, left)
        );
        let squares: Vec<Square> = left.into_iter().take(Square::COUNT + 1).collect();
        prop_assert!(squares.is_sorted() && squares.iter().copied().collect::<Bitboard>() == left && squares.len() == left.into_iter().len());
        let listed: Vec<String> = squares.iter().map(ToString::to_string).collect();
        let (debugged, hex) = (format!("{left:?}"), format!("{:x}", left.bits()));
        prop_assert!(left.to_string() == listed.join(" ") && debugged.contains(&hex));
    });
}

#[test]
fn a_square_joins_and_leaves_a_set_and_subsets_of_a_mask_are_its_distinct_sub_bitboards() {
    proptest!(|(set: Bitboard, square: Square)| {
        prop_assert!(set.combined(Bitboard::from(square), JOIN).contains(square) && set.combined(Bitboard::from(square), LEAVE) == set & !Bitboard::from(square));
        let mask = Bitboard::from_bits(set.bits() & RANK_MASK);
        let subsets: Vec<Bitboard> = SubsetIter::new(mask).take((1 << mask.count()) + 1).collect();
        prop_assert_eq!(subsets.len(), 1 << mask.count());
        prop_assert!(subsets.iter().collect::<HashSet<_>>().len() == subsets.len() && subsets.iter().all(|subset| subset.combined(mask, LEAVE).is_empty()));
    });
}
