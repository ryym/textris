use crate::color::Color;
use std::fmt;
use termion::color;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub chr: char,
    pub color: Color,
}

impl Block {
    pub fn new(chr: char, color: Color) -> Self {
        Block { chr, color }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bg = if self.chr == 'X' { Color::white() } else { self.color };
        let fg = if self.chr == 'X' { Color::black() } else { Color::white() };

        write!(
            f,
            "{}{}{}{}{}",
            color::Fg(fg),
            color::Bg(bg),
            self.chr,
            color::Fg(color::Reset),
            color::Bg(Color::black()),
        )
    }
}
