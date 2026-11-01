use board::{Bishop, Bitboard, Rook, Slider, Square, SubsetIter};
use proptest::prelude::*;
use strum::IntoEnumIterator;

trait Checks: Slider {
    fn first_square_where_a_lookup_differs() -> Option<Square> {
        Square::iter().find(|square| {
            SubsetIter::new(Self::relevant_occupancy(*square)).any(|occupied| {
                Self::attacks(*square, occupied) != Self::attacks_by_ray(*square, occupied)
            })
        })
    }

    fn first_square_where_irrelevant_pieces_matter(occupied: Bitboard) -> Option<Square> {
        Square::iter().find(|square| {
            let relevant = occupied & Self::relevant_occupancy(*square);
            Self::attacks(*square, occupied) != Self::attacks_by_ray(*square, relevant)
        })
    }
}

impl<S: Slider> Checks for S {}

#[test]
fn lookups_match_ray_walking_for_every_relevant_occupancy() {
    assert_eq!(Rook::first_square_where_a_lookup_differs(), None);
    assert_eq!(Bishop::first_square_where_a_lookup_differs(), None);
}

#[test]
fn lookups_ignore_pieces_outside_the_relevant_occupancy() {
    proptest!(|(occupied: Bitboard)| {
        prop_assert_eq!(
            (Rook::first_square_where_irrelevant_pieces_matter(occupied), Bishop::first_square_where_irrelevant_pieces_matter(occupied)),
            (None, None)
        );
    });
}
