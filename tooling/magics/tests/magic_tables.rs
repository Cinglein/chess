use enum_map::EnumMap;
use magics::MagicTables;

const EXPECTED: [&str; 3] = [
    "pub(crate) const ROOK: EnumMap<Square, u64>", "pub(crate) const BISHOP: EnumMap<Square, u64>",
    "    0xffff_ffff_ffff_ffff,\n]);",
];

#[test]
fn the_tables_print_as_two_named_constants_of_grouped_hex_literals() {
    let full = EnumMap::from_fn(|_| u64::MAX);
    let printed = MagicTables::new(full, full).to_string();
    assert!(EXPECTED.iter().all(|part| printed.contains(part)));
}
