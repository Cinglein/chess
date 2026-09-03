use std::fmt::Write;
use std::fs;
use std::path::Path;

use board::attacks::{Magic, Slider};
use board::{Bitboard, Square};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const OUTPUT: &str = "crates/board/src/attacks/magics.rs";

pub fn regenerate(workspace_root: &Path) -> Result<(), String> {
    let mut rng = XorShift(SEED);
    let rook = find_all(Slider::Rook, &mut rng);
    let bishop = find_all(Slider::Bishop, &mut rng);
    let path = workspace_root.join(OUTPUT);
    fs::write(&path, render(&rook, &bishop)).map_err(|error| format!("{OUTPUT}: {error}"))?;
    println!("wrote {OUTPUT}");
    Ok(())
}

fn find_all(slider: Slider, rng: &mut XorShift) -> [u64; Square::COUNT] {
    Square::ALL.map(|square| find_magic(slider, square, rng))
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

struct XorShift(u64);

impl XorShift {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn sparse(&mut self) -> u64 {
        self.next() & self.next() & self.next()
    }
}

fn render(rook: &[u64; Square::COUNT], bishop: &[u64; Square::COUNT]) -> String {
    let mut out = String::from("use crate::square::Square;\n\n");
    render_array(&mut out, "ROOK", rook);
    out.push('\n');
    render_array(&mut out, "BISHOP", bishop);
    out
}

fn render_array(out: &mut String, name: &str, values: &[u64; Square::COUNT]) {
    let _ = writeln!(out, "pub(super) const {name}: [u64; Square::COUNT] = [");
    for value in values {
        let _ = writeln!(out, "    {},", hex_literal(*value));
    }
    out.push_str("];\n");
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
    use super::{XorShift, find_magic, hex_literal};
    use board::Square;
    use board::attacks::{Magic, Slider};

    #[test]
    fn hex_literals_are_grouped_in_fours_for_readability() {
        assert_eq!(hex_literal(0x8a80_1040_0080_0020), "0x8a80_1040_0080_0020");
        assert_eq!(hex_literal(0), "0x0000_0000_0000_0000");
    }

    #[test]
    fn a_found_magic_maps_every_occupancy_to_its_attacks() {
        let mut rng = XorShift(7);
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
