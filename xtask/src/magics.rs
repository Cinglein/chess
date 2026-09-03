use std::fmt::Write;
use std::fs;
use std::path::Path;

use board::{Bitboard, Magic, Slider, Square};
use enum_map::EnumMap;

use crate::xor_shift::XorShift;

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const OUTPUT: &str = "crates/board/src/slider/magics.rs";

pub fn regenerate(workspace_root: &Path) -> Result<(), String> {
    let mut rng = XorShift::new(SEED);
    let rook = find_all(Slider::Rook, &mut rng);
    let bishop = find_all(Slider::Bishop, &mut rng);
    let path = workspace_root.join(OUTPUT);
    fs::write(&path, render(&rook, &bishop)).map_err(|error| format!("{OUTPUT}: {error}"))?;
    println!("wrote {OUTPUT}");
    Ok(())
}

fn find_all(slider: Slider, rng: &mut XorShift) -> EnumMap<Square, u64> {
    EnumMap::from_fn(|square| find_magic(slider, square, rng))
}

fn find_magic(slider: Slider, square: Square, rng: &mut XorShift) -> u64 {
    let mask = slider.relevant_occupancy(square);
    let expected: Vec<(Bitboard, Bitboard)> = mask
        .subsets()
        .map(|subset| (subset, slider.attacks_by_ray(square, subset)))
        .collect();
    let mut table = vec![Bitboard::EMPTY; Magic::new(mask, 0, 0).table_size()];
    loop {
        let candidate = rng.sparse();
        if spreads_poorly(mask, candidate) {
            continue;
        }
        let magic = Magic::new(mask, candidate, 0);
        table.fill(Bitboard::EMPTY);
        let collision_free = expected.iter().all(|&(subset, attacks)| {
            let slot = &mut table[magic.index(subset)];
            if slot.is_empty() {
                *slot = attacks;
                true
            } else {
                *slot == attacks
            }
        });
        if collision_free {
            return candidate;
        }
    }
}

fn spreads_poorly(mask: Bitboard, candidate: u64) -> bool {
    (mask.bits().wrapping_mul(candidate) >> 56).count_ones() < 6
}

fn render(rook: &EnumMap<Square, u64>, bishop: &EnumMap<Square, u64>) -> String {
    let mut out = String::from("use enum_map::EnumMap;\n\nuse crate::square::Square;\n\n");
    render_table(&mut out, "ROOK", rook);
    out.push('\n');
    render_table(&mut out, "BISHOP", bishop);
    out
}

fn render_table(out: &mut String, name: &str, values: &EnumMap<Square, u64>) {
    let _ = writeln!(
        out,
        "pub(super) const {name}: EnumMap<Square, u64> = EnumMap::from_array(["
    );
    for value in values.values() {
        let _ = writeln!(out, "    {},", hex_literal(*value));
    }
    out.push_str("]);\n");
}

fn hex_literal(value: u64) -> String {
    let digits = format!("{value:016x}");
    let groups: Vec<&str> = digits
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).expect("hex digits are ascii"))
        .collect();
    format!("0x{}", groups.join("_"))
}

#[cfg(test)]
mod tests {
    use board::{Magic, Slider, Square};

    use super::{find_magic, hex_literal};
    use crate::xor_shift::XorShift;

    #[test]
    fn hex_literals_are_grouped_in_fours_for_readability() {
        assert_eq!(hex_literal(0x8a80_1040_0080_0020), "0x8a80_1040_0080_0020");
        assert_eq!(hex_literal(0), "0x0000_0000_0000_0000");
    }

    #[test]
    fn a_found_magic_maps_every_occupancy_to_its_attacks() {
        let mut rng = XorShift::new(7);
        let square = Square::D4;
        let multiplier = find_magic(Slider::Bishop, square, &mut rng);
        let mask = Slider::Bishop.relevant_occupancy(square);
        let magic = Magic::new(mask, multiplier, 0);
        let mut table = vec![None; magic.table_size()];
        for subset in mask.subsets() {
            let attacks = Slider::Bishop.attacks_by_ray(square, subset);
            let slot = &mut table[magic.index(subset)];
            assert!(slot.is_none_or(|stored| stored == attacks));
            *slot = Some(attacks);
        }
    }
}
