extern crate termion;
extern crate textris;

use std::{env, io, process};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use textris::cli::{self, CliParsed, Config};
use textris::game::Game;
use textris::inputs::Inputs;
use textris::screen::Screen;

enum Exit {
    Ok,
    Err(i32),
}

impl Exit {
    pub fn code(&self) -> i32 {
        match self {
            Exit::Ok => 0,
            Exit::Err(n) => *n,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let exit_code = match cli::parse_args(&args) {
        Ok(parsed) => match parsed {
            CliParsed::Help(msg) => {
                println!("{}", msg);
                Exit::Ok
            }
            CliParsed::Run(config) => run(config),
        },
        Err(err) => {
            println!("ERR: {}", err);
            for e in err.iter_causes() {
                println!("  {}", e);
            }
            Exit::Err(1)
        }
    };
    process::exit(exit_code.code());
}

fn run(conf: Config) -> Exit {
    let stdout = io::stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();

    let inputs = Inputs::new(io::stdin().events(), conf.key());
    let screen = Screen::new(stdout);
    let mut game = Game::new(inputs, screen);

    match game.start() {
        Ok(_) => Exit::Ok,
        Err(err) => {
            game.stop_by_error(err);
            Exit::Err(1)
        }
    }
}
