use errors::*;
use play::Play;
use std::io::{self, Bytes, Read, Write};
use std::iter;
use std::thread;
use std::time::Duration;
use termion as tm;
use termion::cursor::Goto;

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

    fn clear_screen(&mut self) -> Result<()> {
        write!(
            self.stdout,
            "{}{}{}",
            tm::clear::All,
            Goto(1, 1),
            tm::cursor::Hide
        )?;
        Ok(())
    }

    pub fn render_title(&mut self) -> Result<()> {
        self.clear_screen()?;

        let interval = Duration::from_millis(32);
        for i in 0..(TITLE.len() + 1) {
            write!(self.stdout, "{}{}", Goto(1, 1), &TITLE[0..i])?;
            self.stdout.flush()?;
            thread::sleep(interval);
        }

        Ok(())
    }

    pub fn render_header(&mut self) -> Result<()> {
        self.clear_screen()?;
        write!(self.stdout, "{}", TITLE)?;
        Ok(())
    }

    pub fn render(&mut self, play: &Play) -> Result<()> {
        let field = play.field();

        for (i, line) in field.lines_iter().enumerate() {
            write!(
                self.stdout,
                "{}|",
                Goto(FIELD_X as u16, (i + FIELD_Y) as u16)
            )?;
            for cell in line.iter() {
                match cell {
                    Some(block) => write!(self.stdout, "{} ", block.chr),
                    None => write!(self.stdout, "  "),
                }?;
            }
            write!(self.stdout, "|")?;
        }

        write!(
            self.stdout,
            "{}",
            Goto(FIELD_X as u16, (field.height() + FIELD_Y) as u16)
        )?;

        let width = field.width();
        for floor in iter::repeat("--").take(width + 1) {
            write!(self.stdout, "{}", floor)?;
        }

        self.render_side_menu(&play, (field.width() * 2 + 4) as u16)?;

        Ok(())
    }

    fn render_side_menu(&mut self, play: &Play, x: u16) -> Result<()> {
        let y = FIELD_Y as u16;
        write!(self.stdout, "{}{}", Goto(x, y), "?: Help")?;
        write!(self.stdout, "{}Time:  {}", Goto(x, y + 2), play.elapsed())?;
        write!(self.stdout, "{}Score: {}", Goto(x, y + 3), play.score())?;
        Ok(())
    }

    pub fn render_game_over(&mut self, play: &Play) -> Result<()> {
        self.show_modal(&Modal {
            title: "GAME OVER",
            content: &[
                &format!("Time:  {}", play.elapsed()),
                &format!("Score: {}", play.score()),
            ],
        })
    }

    pub fn show_modal(&mut self, modal: &Modal) -> Result<()> {
        let border = "---------------------------------------";
        let cleared = iter::repeat(" ").take(border.len()).collect::<String>();
        let cleared_content = &cleared[1..cleared.len() - 1];
        let y_start = 5;
        let mut y = y_start;
        let x = 3;

        write!(self.stdout, "{}{}", Goto(x, y), border)?;
        y += 1;
        write!(self.stdout, "{}|{}|", Goto(x, y), cleared_content)?;
        write!(self.stdout, "{}{}", Goto(x + 2, y), modal.title)?;
        y += 1;
        write!(
            self.stdout,
            "{}|{}|",
            Goto(x, y),
            &border[1..border.len() - 1]
        )?;

        for line in modal.content.iter() {
            y += 1;
            write!(self.stdout, "{}|{}|", Goto(x, y), cleared_content)?;
            write!(self.stdout, "{}{}", Goto(x + 2, y), line)?;
        }
        write!(self.stdout, "{}{}", Goto(x, y + 1), border)?;

        self.stdout.flush()?;
        self.wait_any_key_input(Duration::from_millis(50));

        write!(self.stdout, "{}{}", Goto(x, y_start), cleared)?;
        for y in y_start..=y {
            write!(self.stdout, "{}{}", Goto(x, y), cleared)?;
        }
        write!(self.stdout, "{}{}", Goto(x, y + 1), cleared)?;

        Ok(())
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
        write!(self.stdout, "{}", tm::cursor::Show).expect("restore cursor");
    }
}
