use self::Color::*;
use std::fmt;
use termion::color as tmc;

// XXX: It would be nice if we define Color using macro.

// Color wraps termion::Color as enum.
// Without this we need to use Trait object but it makes difficult to clone/copy values.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red(tmc::Red),
    Blue(tmc::Blue),
    Yellow(tmc::Yellow),
    LightBlue(tmc::LightBlue),
    Green(tmc::Green),
    LightRed(tmc::LightRed),
    Cyan(tmc::Cyan),
    LightMagenta(tmc::LightMagenta),
    Rgb(tmc::Rgb),
}

impl Color {
    pub fn red() -> Color {
        Red(tmc::Red)
    }

    pub fn blue() -> Color {
        Blue(tmc::Blue)
    }

    pub fn green() -> Color {
        Green(tmc::Green)
    }

    pub fn yellow() -> Color {
        Yellow(tmc::Yellow)
    }

    pub fn light_magenda() -> Color {
        LightMagenta(tmc::LightMagenta)
    }

    pub fn light_blue() -> Color {
        LightBlue(tmc::LightBlue)
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Rgb(tmc::Rgb(r, g, b))
    }

    pub fn orange() -> Color {
        Color::rgb(255, 165, 0)
    }
}

impl tmc::Color for Color {
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Red(c) => c.write_fg(f),
            Blue(c) => c.write_fg(f),
            Yellow(c) => c.write_fg(f),
            LightBlue(c) => c.write_fg(f),
            Green(c) => c.write_fg(f),
            LightRed(c) => c.write_fg(f),
            Cyan(c) => c.write_fg(f),
            LightMagenta(c) => c.write_fg(f),
            Rgb(c) => c.write_fg(f),
        }
    }

    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Red(c) => c.write_bg(f),
            Blue(c) => c.write_bg(f),
            Yellow(c) => c.write_bg(f),
            LightBlue(c) => c.write_bg(f),
            Green(c) => c.write_bg(f),
            LightRed(c) => c.write_bg(f),
            Cyan(c) => c.write_bg(f),
            LightMagenta(c) => c.write_bg(f),
            Rgb(c) => c.write_bg(f),
        }
    }
}
