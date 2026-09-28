mod contestant;

use std::env;

use arena::{ArenaError, Game};
use board::Board;
use contestant::Contestant;

fn main() -> Result<(), ArenaError> {
    let [white, black]: [String; 2] = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .try_into()
        .map_err(|_| ArenaError::Usage)?;
    let mut white = Contestant::from(white.as_str()).opponent()?;
    let mut black = Contestant::from(black.as_str()).opponent()?;
    let finished = Game::from(Board::START).play(white.as_mut(), black.as_mut());
    println!("{finished}");
    Ok(())
}
