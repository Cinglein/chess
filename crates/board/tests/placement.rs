use board::Board;

const MATERIAL: [(&str, bool); 10] = [
    ("1b2k3/8/8/8/8/8/8/2B1K3", true),
    ("8/8/8/4k3/8/8/8/4K3", true),
    ("8/8/8/4k3/8/8/8/4KB2", true),
    ("8/8/8/4k3/8/8/8/4KN2", true),
    ("b3k3/8/8/8/8/8/8/4KB2", true),
    ("b3k3/8/8/8/8/8/8/2B1K3", false),
    ("8/8/8/4k3/8/8/8/3BKN2", false),
    ("8/8/8/4k3/8/8/8/3NKN2", false),
    ("8/8/8/4k3/8/8/4P3/4K3", false),
    ("4k3/8/8/8/8/8/8/R3K3", false),
];

#[test]
fn material_fixtures_say_whether_either_side_could_still_mate() {
    for (fen, lacks) in MATERIAL {
        let board: Board = format!("{fen} w - - 0 1").parse().unwrap();
        assert_eq!(board.placement().lacks_mating_material(), lacks, "{fen}");
    }
}
