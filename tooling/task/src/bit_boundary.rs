use std::path::Path;

pub struct BitBoundary;

impl BitBoundary {
    const FILES: [&str; 7] = [
        "crates/board/src/square/", "crates/board/src/direction.rs",
        "crates/board/src/slider/magic.rs", "crates/board/src/slider/magics.rs",
        "crates/board/src/slider/attack_table.rs", "crates/board/src/zobrist_keys/split_mix.rs",
        "tooling/magics/src/",
    ];

    #[must_use]
    pub fn covers(path: &Path) -> bool {
        Self::FILES
            .iter()
            .any(|boundary| path.starts_with(boundary))
    }
}
