use coord::Dir;
use play::Play;
use screen::{Modal, Screen};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub enum Action {
    Quit,
}

const FRAME: u64 = 50;
const TICK: u64 = 1000 / FRAME;
const UPDATE: u64 = TICK / 2;

pub struct Game<R: Read, W: Write> {
    screen: Screen<R, W>,
    help_modal: Modal<'static>,
}

impl<R, W> Game<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(screen: Screen<R, W>) -> Self {
        Game {
            screen,
            help_modal: Modal {
                title: "HELP",
                content: vec![
                    "h - Move left",
                    "l - Move right",
                    "j - Speed up",
                    "d,f - Rotate",
                    "q - Quit",
                ],
            },
        }
    }

    pub fn start(&mut self) {
        self.screen.render_title();
        thread::sleep(Duration::from_millis(800));
        self.play(Play::new());
    }

    fn play(&mut self, mut play: Play) {
        self.screen.render_header();

        let interval = Duration::from_millis(FRAME);
        let mut t = 0;
        loop {
            match self.handle_user_input(&mut play) {
                Some(Action::Quit) => break,
                _ => {}
            };

            if t % UPDATE == 0 {
                if let Err(_) = play.update() {
                    self.screen.render_game_over(&play);
                    break;
                }
            }
            if t % TICK == 0 {
                play.tick();
            }

            self.screen.render(&play);
            thread::sleep(interval);
            t += 1;
        }
    }

    fn handle_user_input(&mut self, play: &mut Play) -> Option<Action> {
        match self.screen.next_input() {
            Some(Ok(key)) => match key {
                b'q' => return Some(Action::Quit),
                b'h' => play.slide_piece(Dir::Left),
                b'l' => play.slide_piece(Dir::Right),
                b'j' => play.slide_piece(Dir::Down),
                b'd' => play.rotate_piece(false),
                b'f' => play.rotate_piece(true),
                b'?' => self.screen.show_modal(&self.help_modal),
                _ => {}
            },
            _ => {}
        };
        None
    }
}
