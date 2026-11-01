use board::{Bishop, Magic, Slider, Square, SubsetIter};
use magics::MagicSearch;
use magics::XorShift;
#[test]
fn a_found_magic_maps_every_occupancy_to_its_attacks() {
    let square = Square::D4;
    let multiplier = MagicSearch::new::<Bishop>(square).find_multiplier(&mut XorShift::from(7));
    let mask = Bishop::relevant_occupancy(square);
    let magic = Magic::new(mask, multiplier, 0);
    let mut table = vec![None; magic.table_size()];
    for subset in SubsetIter::new(mask) {
        let attacks = Bishop::attacks_by_ray(square, subset);
        let slot = &mut table[magic.index(subset)];
        assert!(slot.is_none_or(|stored| stored == attacks));
        *slot = Some(attacks);
    }
}
