use color::Color;

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
