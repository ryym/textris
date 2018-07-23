extern crate termion;
extern crate textris;

use std::io::{stdout, Read};
use termion::async_stdin;
use termion::raw::IntoRawMode;
use textris::game::Game;
use textris::screen::Screen;

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin().bytes();

    let screen = Screen::new(stdin, stdout);
    let mut game = Game::new(screen);
    game.start();
}
