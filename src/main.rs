extern crate textris;

use textris::game::Game;
use textris::screen;

fn main() {
    let game = Game::new();
    screen::play(game);
}
