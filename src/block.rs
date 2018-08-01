#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub chr: char,
}

impl Block {
    pub fn new(chr: char) -> Self {
        Block { chr }
    }
}
