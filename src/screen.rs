use crate::action::Action;
use crate::color::Color;
use crate::coord::Dir;
use crate::inputs::{Inputs, Order};
use crate::play::Play;
use failure::{Fail, Fallible};
use std::io::Write;
use std::iter;
use std::thread;
use std::time::Duration;
use termion as tm;
use termion::color;
use termion::cursor::Goto;

pub struct Modal {
    pub title: String,
    pub content: Vec<String>,
    pub actions: Vec<Action>,
}

const TITLE: &str = "- T E X T R I S -";
const FIELD_X: usize = 1;
const FIELD_Y: usize = 3;

pub struct Screen<W: Write> {
    stdout: W,
    field_bg: Color,
}

impl<W: Write> Screen<W> {
    pub fn new(stdout: W) -> Screen<W> {
        Screen {
            stdout,
            field_bg: Color::black(),
        }
    }

    fn clear_screen(&mut self) -> Fallible<()> {
        write!(
            self.stdout,
            "{}{}{}",
            tm::clear::All,
            Goto(1, 1),
            tm::cursor::Hide
        )
        .map_err(|e| e.context("failed to clear screen"))?;
        Ok(())
    }

    pub fn render_title(&mut self) -> Fallible<()> {
        self.clear_screen()?;

        let interval = Duration::from_millis(32);
        for i in 0..=TITLE.len() {
            write!(self.stdout, "{}{}", Goto(1, 1), &TITLE[0..i])
                .and_then(|_| self.stdout.flush())
                .map_err(|e| e.context("failed to render title"))?;
            thread::sleep(interval);
        }

        Ok(())
    }

    pub fn render_header(&mut self) -> Fallible<()> {
        self.clear_screen()?;
        write!(self.stdout, "{}", TITLE)?;
        Ok(())
    }

    pub fn render(&mut self, play: &Play) -> Fallible<()> {
        let field = play.field();

        for (i, line) in field.lines_iter().enumerate() {
            write!(
                self.stdout,
                "{}|{}",
                Goto(FIELD_X as u16, (i + FIELD_Y) as u16),
                color::Bg(self.field_bg),
            )?;
            for cell in line.iter() {
                match cell {
                    Some(block) => write!(self.stdout, "{} ", block),
                    None => write!(self.stdout, "  "),
                }?;
            }
            write!(self.stdout, "{}|", color::Bg(color::Reset))?;
        }

        write!(
            self.stdout,
            "{}",
            Goto(FIELD_X as u16, (field.height() + FIELD_Y) as u16)
        )?;

        let width = field.width();
        for floor in iter::repeat("==").take(width + 1) {
            write!(self.stdout, "{}", floor)?;
        }

        self.render_side_menu(&play, (field.width() * 2 + 4) as u16)
            .map_err(|e| e.context("failed to render side menu"))?;

        Ok(())
    }

    fn render_side_menu(&mut self, play: &Play, x: u16) -> Fallible<()> {
        let y = FIELD_Y as u16;

        let next_block = play.next_tetro_hint();
        write!(self.stdout, "{}Next: {}", Goto(x, y), next_block)?;
        write!(self.stdout, "{}?: Help", Goto(x, y + 2))?;
        write!(self.stdout, "{}Time:  {}", Goto(x, y + 4), play.elapsed())?;
        write!(self.stdout, "{}Score: {}", Goto(x, y + 5), play.score())?;
        Ok(())
    }

    pub fn render_game_over(&mut self, inputs: &mut Inputs, play: &Play) -> Fallible<Action> {
        self.show_modal(
            inputs,
            &Modal {
                title: "GAME OVER".to_string(),
                content: vec![
                    format!("Time:  {}", play.elapsed()),
                    format!("Score: {}", play.score()),
                ],
                actions: vec![Action::Retry, Action::Quit],
            },
        )
    }

    pub fn show_modal(&mut self, inputs: &mut Inputs, modal: &Modal) -> Fallible<Action> {
        let border = "---------------------------------------";
        let inner_border = format!("|{}|", &border[1..border.len() - 1]);
        let back = iter::repeat(" ").take(border.len()).collect::<String>();
        let inner_back = format!("|{}|", &back[1..back.len() - 1]);
        let y_start = 5;
        let mut y = y_start;
        let x = 3;

        write!(self.stdout, "{}{}", Goto(x, y), border)?;
        y += 1;
        write!(self.stdout, "{}{}", Goto(x, y), inner_back)?;
        write!(self.stdout, "{}{}", Goto(x + 2, y), modal.title)?;
        y += 1;
        write!(self.stdout, "{}{}", Goto(x, y), inner_border)?;
        y += 1;

        for line in modal.content.iter() {
            write!(self.stdout, "{}{}", Goto(x, y), inner_back)?;
            write!(self.stdout, "{}{}", Goto(x + 2, y), line)?;
            y += 1;
        }

        write!(self.stdout, "{}{}", Goto(x, y), inner_border)?;
        y += 1;
        write!(self.stdout, "{}{}", Goto(x, y), inner_back)?;

        let y_actions = y;

        let actions = &modal.actions;
        let mut select = 0;

        let action_btns = self.write_inline_actions(actions, select);
        write!(self.stdout, "{}{}", Goto(x + 1, y_actions), action_btns)?;
        y += 1;
        write!(self.stdout, "{}{}", Goto(x, y), border)?;

        self.stdout.flush()?;

        loop {
            if let Ok(order) = inputs.recv_order()? {
                match order {
                    Order::Move(Dir::Left) => {
                        if select > 0 {
                            select -= 1;
                        }
                    }
                    Order::Move(Dir::Right) => {
                        if select < actions.len() - 1 {
                            select += 1;
                        }
                    }
                    Order::Select | Order::Quit => break,
                    _ => {}
                }
            }

            let action_btns = self.write_inline_actions(actions, select);
            write!(self.stdout, "{}{}", Goto(x + 1, y_actions), action_btns)?;
            self.stdout.flush()?;
        }

        // Clear modal
        write!(self.stdout, "{}{}", Goto(x, y_start), back)?;
        for y in y_start..=y {
            write!(self.stdout, "{}{}", Goto(x, y), back)?;
        }

        Ok(actions[select])
    }

    fn write_inline_actions(&self, actions: &[Action], select: usize) -> String {
        actions
            .iter()
            .enumerate()
            .map(|(i, a)| {
                if i == select {
                    format!(" [{}] ", a)
                } else {
                    format!("  {}  ", a)
                }
            })
            .collect()
    }
}

impl<W: Write> Drop for Screen<W> {
    fn drop(&mut self) {
        write!(self.stdout, "{}", tm::cursor::Show).expect("restore cursor");
    }
}
