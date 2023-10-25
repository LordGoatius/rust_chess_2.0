pub mod piece;
pub mod board;
pub mod game;
pub mod errors;

use crate::game::*;

fn main() {
    let mut thing = Game::default();
    thing.game_loop();
}
