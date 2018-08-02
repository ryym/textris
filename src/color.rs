use self::Color::*;
use std::fmt;
use termion::color as tmc;

// Color wraps termion::Color as enum.
// Without this we need to use Trait object but it makes difficult to clone/copy values.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red(tmc::Red),
    Blue(tmc::Blue),
}

impl Color {
    pub fn red() -> Color {
        Red(tmc::Red)
    }

    pub fn blue() -> Color {
        Blue(tmc::Blue)
    }
}

impl tmc::Color for Color {
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Red(c) => c.write_fg(f),
            Blue(c) => c.write_fg(f),
        }
    }

    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Red(c) => c.write_bg(f),
            Blue(c) => c.write_bg(f),
        }
    }
}
