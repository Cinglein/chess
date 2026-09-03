pub use board::START_FEN;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::{START_FEN, version};

    #[test]
    fn version_matches_manifest() {
        assert_eq!(version(), "0.1.0");
    }

    #[test]
    fn reexports_board_start_position() {
        assert!(START_FEN.starts_with("rnbqkbnr"));
    }
}
