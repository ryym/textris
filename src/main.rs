extern crate termion;
extern crate textris;

use std::io::{stdout, Read};
use std::process;
use termion::async_stdin;
use termion::raw::IntoRawMode;
use textris::game::Game;
use textris::screen::Screen;

fn main() {
    // Separate the main code to 'run' function
    // so that destructors are called properly.
    // (https://stackoverflow.com/questions/30281235/how-to-cleanly-end-the-program-with-an-exit-code/)
    let exit_code = match run() {
        Ok(_) => 0,
        Err(_) => 1,
    };
    process::exit(exit_code);
}

fn run() -> Result<(), ()> {
    let stdout = stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin().bytes();

    let screen = Screen::new(stdin, stdout);
    let mut game = Game::new(screen);
    match game.start() {
        Ok(_) => Ok(()),
        Err(err) => {
            game.stop_by_error(err);
            Err(())
        }
    }
}
