use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub chr: char,
    pub color: Color,
}

impl Block {
    pub fn new(chr: char) -> Self {
        Block {
            chr,
            color: if chr < 'O' {
                Color::red()
            } else {
                Color::blue()
            },
        }
    }
}
