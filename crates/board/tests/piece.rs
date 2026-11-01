use board::Piece;
use proptest::prelude::any;

laws::roundtrip!(
    any_piece_prints_and_parses_back_to_itself,
    any::<Piece>(),
    ToString::to_string,
    |text: &String| text.parse::<Piece>(),
    Ok
);
