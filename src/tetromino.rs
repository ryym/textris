use coord::{Coord, Dir};
use piece::Piece;

pub enum Tetromino {
    I,
    T,
}

impl Tetromino {
    pub fn make_piece(&self, dir: Dir) -> Piece {
        use coord::Dir::*;

        match self {
            Tetromino::I => {
                let coords = match dir {
                    Up | Down => [Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3)],
                    Left | Right => [Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0)],
                };
                Piece::new('I', coords)
            }
            Tetromino::T => {
                let coords = match dir {
                    Up => [Coord(0, 0), Coord(0, 1), Coord(-1, 1), Coord(1, 1)],
                    Right => [Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(0, 2)],
                    Down => [Coord(0, 0), Coord(-1, 0), Coord(1, 0), Coord(0, 1)],
                    Left => [Coord(0, 0), Coord(0, 1), Coord(-1, 1), Coord(0, 2)],
                };
                Piece::new('T', coords)
            }
        }
    }
}
