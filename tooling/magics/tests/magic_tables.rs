use enum_map::EnumMap;
use magics::MagicTables;

const EXPECTED: [&str; 4] = [
    "pub(crate) const ROOK: EnumMap<Square, u64>", "pub(crate) const BISHOP: EnumMap<Square, u64>",
    "    0xffff_ffff_ffff_ffff,\n]);", "    0x0000_0000_0000_0001,\n]);",
];

#[test]
fn the_tables_print_as_two_named_constants_of_grouped_hex_literals() {
    let full = EnumMap::from_fn(|_| u64::MAX);
    let one = EnumMap::from_fn(|_| 1);
    let printed = MagicTables::new(full, one).to_string();
    assert!(EXPECTED.iter().all(|part| printed.contains(part)));
}
