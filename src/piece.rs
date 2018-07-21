use coord::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub chr: char,
}

impl Block {
    pub fn new(chr: char) -> Self {
        Block { chr }
    }
}

pub type PieceCoords = [Coord; 4];

#[derive(Debug)]
pub struct Piece {
    block: Block,
    coords: PieceCoords,
}

impl Piece {
    pub fn new(chr: char, coords: PieceCoords) -> Self {
        Piece {
            block: Block::new(chr),
            coords,
        }
    }

    pub fn block(&self) -> Block {
        self.block
    }

    pub fn coords(&self, base: Coord) -> Vec<Coord> {
        self.coords.iter().map(|c| base + *c).collect()
    }
}
