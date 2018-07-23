use play::Play;
use std::io::{self, Bytes, Read, Write};
use std::iter;
use std::thread;
use std::time::Duration;
use termion as tm;

pub struct Modal<'a> {
    pub title: &'a str,
    pub content: &'a [&'a str],
}

const TITLE: &'static str = "- T E X T R I S -";
const FIELD_X: usize = 1;
const FIELD_Y: usize = 3;

pub struct Screen<R: Read, W: Write> {
    stdin: Bytes<R>,
    stdout: W,
}

impl<R, W> Screen<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(stdin: Bytes<R>, stdout: W) -> Screen<R, W> {
        Screen { stdin, stdout }
    }

    pub fn next_input(&mut self) -> Option<io::Result<u8>> {
        self.stdin.next()
    }

    pub fn render_title(&mut self) {
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
    }

    pub fn render_header(&mut self) {
        write!(
            self.stdout,
            "{}{}{}{}",
            tm::clear::All,
            tm::cursor::Goto(1, 1),
            tm::cursor::Hide,
            TITLE,
        ).unwrap();

        write!(
            self.stdout,
            "{}{}",
            tm::cursor::Goto(1, 2),
            "(press '?' for help)"
        ).unwrap();
    }

    pub fn render(&mut self, play: &Play) {
        let field = play.field();

        for (i, line) in field.lines_iter().enumerate() {
            write!(
                self.stdout,
                "{}|",
                tm::cursor::Goto(FIELD_X as u16, (i + FIELD_Y) as u16)
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
            tm::cursor::Goto(FIELD_X as u16, (field.height() + FIELD_Y) as u16)
        ).unwrap();

        let width = field.width();
        for floor in iter::repeat("--").take(width + 1) {
            write!(self.stdout, "{}", floor).unwrap();
        }

        write!(
            self.stdout,
            "{}Time: {}",
            tm::cursor::Goto((field.width() * 2 + 4) as u16, FIELD_Y as u16),
            play.elapsed(),
        ).unwrap();
    }

    pub fn render_game_over(&mut self, play: &Play) {
        self.show_modal(&Modal {
            title: "GAME OVER",
            content: &[&format!("Time: {}", play.elapsed())],
        });
    }

    pub fn show_modal(&mut self, modal: &Modal) {
        let border = "---------------------------------------";
        let cleared = iter::repeat(" ").take(border.len()).collect::<String>();
        let cleared_content = &cleared[1..cleared.len() - 1];
        let y_start = 5;
        let mut y = y_start;
        let x = 3;

        write!(self.stdout, "{}{}", tm::cursor::Goto(x, y), border).unwrap();
        y += 1;
        write!(
            self.stdout,
            "{}|{}|",
            tm::cursor::Goto(x, y),
            cleared_content
        ).unwrap();
        write!(self.stdout, "{}{}", tm::cursor::Goto(x + 2, y), modal.title).unwrap();
        y += 1;
        write!(
            self.stdout,
            "{}|{}|",
            tm::cursor::Goto(x, y),
            &border[1..border.len() - 1]
        ).unwrap();

        for line in modal.content.iter() {
            y += 1;
            write!(
                self.stdout,
                "{}|{}|",
                tm::cursor::Goto(x, y),
                cleared_content
            ).unwrap();
            write!(self.stdout, "{}{}", tm::cursor::Goto(x + 2, y), line).unwrap();
        }
        write!(self.stdout, "{}{}", tm::cursor::Goto(x, y + 1), border).unwrap();

        self.stdout.flush().unwrap();
        self.wait_any_key_input(Duration::from_millis(50));

        // y = y_start;
        write!(self.stdout, "{}{}", tm::cursor::Goto(x, y_start), cleared).unwrap();
        for y in y_start..=y {
            write!(self.stdout, "{}{}", tm::cursor::Goto(x, y), cleared).unwrap();
        }
        write!(self.stdout, "{}{}", tm::cursor::Goto(x, y + 1), cleared).unwrap();
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

impl<R, W> Drop for Screen<R, W>
where
    R: Read,
    W: Write,
{
    fn drop(&mut self) {
        write!(self.stdout, "{}", tm::cursor::Show).unwrap();
    }
}
