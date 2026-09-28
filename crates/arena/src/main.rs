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
    let mut white = white
        .parse::<Contestant>()
        .map_err(|_| ArenaError::Usage)?
        .opponent()?;
    let mut black = black
        .parse::<Contestant>()
        .map_err(|_| ArenaError::Usage)?
        .opponent()?;
    let finished = Game::from(Board::START).play(white.as_mut(), black.as_mut());
    println!("{finished}");
    Ok(())
}
