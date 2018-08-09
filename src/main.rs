extern crate termion;
extern crate textris;

use std::{env, io, process};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use textris::cli::{self, CliParsed, Config};
use textris::game::Game;
use textris::inputs::Inputs;
use textris::screen::Screen;

fn main() {
    let args: Vec<String> = env::args().collect();
    let exit_code = match cli::parse_args(&args) {
        Ok(parsed) => match parsed {
            CliParsed::Help(msg) => {
                println!("{}", msg);
                0
            }
            CliParsed::Run(config) => run(config),
        },
        Err(err) => {
            println!("{}", err);
            1
        }
    };
    process::exit(exit_code);
}

fn run(conf: Config) -> i32 {
    let stdout = io::stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();

    let inputs = Inputs::new(io::stdin().events(), conf.key());
    let screen = Screen::new(stdout);
    let mut game = Game::new(inputs, screen);

    match game.start() {
        Ok(_) => 0,
        Err(err) => {
            game.stop_by_error(err);
            1
        }
    }
}
