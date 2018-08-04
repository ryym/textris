use std::fmt;
use termion::color as tmc;

macro_rules! define_color {
    (
        $($var:ident => $method:ident),+$(,)*
    ) => {

        // Color wraps termion::Color as enum.
        // Without this we need to use Trait object to hold various colors as a same field
        // but it makes difficult to clone/copy values.
        #[derive(Debug, Clone, Copy)]
        pub enum Color {
            $($var(tmc::$var),)*
        }

        impl Color {
            $(
                pub fn $method() -> Color {
                    Color::$var(tmc::$var)
                }
            )*
        }

        impl tmc::Color for Color {
            fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(
                        Color::$var(c) => c.write_fg(f),
                    )*
                }
            }

            fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(
                        Color::$var(c) => c.write_bg(f),
                    )*
                }
            }
        }
    };
}

define_color! {
    Red => red,
    Blue => blue,
    Yellow => yellow,
    Green => green,
    Cyan => cyan,
    Magenta => magenta,
    Black => black,
    White => white,
    LightBlue => light_blue,
    LightRed => light_red,
    LightMagenta => light_magenta,
}
