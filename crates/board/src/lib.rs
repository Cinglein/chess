#![cfg_attr(not(test), no_std)]

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[cfg(test)]
mod tests {
    use super::START_FEN;

    #[test]
    fn start_fen_has_six_fields() {
        assert_eq!(START_FEN.split(' ').count(), 6);
    }
}
