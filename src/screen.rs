use coord::{Coord, Dir};
use elapsed::Elapsed;
use game::Game;
use std::io::{Bytes, Read, Write};
use std::iter;
use std::thread;
use std::time::Duration;
use termion as tm;

struct State {
    field_pos: Coord,
    elapsed: Elapsed,
}

pub struct Screen<R: Read, W: Write> {
    stdin: Bytes<R>,
    stdout: W,
    game: Game,
}

const TITLE: &'static str = "- T E X T R I S -";

impl<R, W> Screen<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(stdin: Bytes<R>, stdout: W, game: Game) -> Screen<R, W> {
        Screen {
            stdin,
            stdout,
            game,
        }
    }

    pub fn start(&mut self) {
        let interval = Duration::from_millis(32);
        for i in 0..(TITLE.len() + 1) {
            write!(
                self.stdout,
                "{}{}{}{}",
                tm::clear::All,
                tm::cursor::Goto(1, 1),
                tm::cursor::Hide,
                &TITLE[0..i],
            ).unwrap();
            self.stdout.flush().unwrap();
            thread::sleep(interval);
        }
        thread::sleep(Duration::from_millis(800));

        self.play();
    }

    pub fn play(&mut self) {
        write!(
            self.stdout,
            "{}{}{}{}",
            tm::clear::All,
            tm::cursor::Goto(1, 1),
            tm::cursor::Hide,
            TITLE,
        ).unwrap();

        let mut state = State {
            field_pos: Coord(1, 2),
            elapsed: Elapsed::new(),
        };

        let interval = Duration::from_millis(50);
        let mut t = 0;
        loop {
            match self.stdin.next() {
                Some(Ok(key)) => match key {
                    b'q' => break,
                    b'h' => self.game.slide_piece(Dir::Left),
                    b'l' => self.game.slide_piece(Dir::Right),
                    b'j' => self.game.slide_piece(Dir::Down),
                    b'd' => self.game.rotate_piece(false),
                    b'f' => self.game.rotate_piece(true),
                    _ => {}
                },
                _ => {}
            }

            if t % 10 == 0 {
                match self.game.tick() {
                    Ok(_) => {}
                    Err(_) => {
                        self.render_game_over();
                        self.stdout.flush().unwrap();
                        self.wait_any_key_input(interval);
                        break;
                    }
                }
            }

            if t % 20 == 0 {
                state.elapsed.add_secs(1);
            }

            self.render(&state);
            self.stdout.flush().unwrap();

            thread::sleep(interval);
            t += 1;
        }

        write!(self.stdout, "{}", tm::cursor::Show).unwrap();
    }

    fn render(&mut self, state: &State) {
        let field = self.game.field();
        let Coord(x, y) = state.field_pos;
        let x = x as usize;
        let y = y as usize;

        for (i, line) in field.lines_iter().enumerate() {
            write!(
                self.stdout,
                "{}|",
                tm::cursor::Goto(x as u16, (i + y) as u16)
            ).unwrap();
            for cell in line.iter() {
                match cell {
                    Some(block) => write!(self.stdout, "{} ", block.chr),
                    None => write!(self.stdout, "  "),
                }.unwrap();
            }
            write!(self.stdout, "|").unwrap();
        }

        write!(
            self.stdout,
            "{}",
            tm::cursor::Goto(x as u16, (field.height() + y) as u16)
        ).unwrap();
        let width = field.width();
        for floor in iter::repeat("--").take(width + 1) {
            write!(self.stdout, "{}", floor).unwrap();
        }

        write!(
            self.stdout,
            "{}Time: {}",
            tm::cursor::Goto((field.width() * 2 + 4) as u16, y as u16),
            state.elapsed,
        ).unwrap();
    }

    fn render_game_over(&mut self) {
        write!(self.stdout, "{}", tm::cursor::Goto(1, 1)).unwrap();
        write!(self.stdout, "====== GAME OVER ======").unwrap();
    }

    fn wait_any_key_input(&mut self, interval: Duration) {
        loop {
            match self.stdin.next() {
                Some(_) => break,
                _ => {}
            }
            thread::sleep(interval);
        }
    }
}
