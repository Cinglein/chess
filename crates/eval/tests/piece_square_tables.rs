use board::{Board, Color, PieceKind, Square};
use eval::Score;
use eval::{Evaluator, PieceSquareTables};
const CAPTURED_KNIGHT: &str = "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KNIGHT: PieceKind = PieceKind::Knight;
const HOME: Square = Square::G8;
const CENTRE: Square = Square::E4;
const CORNER: Square = Square::A1;

#[test]
fn the_start_position_is_balanced_a_lost_piece_costs_its_value_and_its_square_adds_to_it() {
    assert_eq!(PieceSquareTables::evaluate(&Board::START), Score::DRAW);
    let down_a_knight: Board = CAPTURED_KNIGHT.parse().unwrap();
    let knight = PieceSquareTables::piece_value(Color::Black, KNIGHT, HOME);
    assert_eq!(PieceSquareTables::evaluate(&down_a_knight), knight);
    assert!(
        PieceSquareTables::piece_value(Color::White, KNIGHT, CENTRE)
            > PieceSquareTables::piece_value(Color::White, KNIGHT, CORNER)
    );
}
