use coord::{Coord, Dir};
use piece::Piece;

pub type Tetrominos = [Tetromino; 7];

#[derive(Clone, Copy)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Tetromino {
    pub fn all() -> Tetrominos {
        use self::Tetromino::*;
        [I, J, L, O, S, T, Z]
    }

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

            Tetromino::J => {
                let coords = match dir {
                    Up => [Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(-1, 2)],
                    Right => [Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1)],
                    Down => [Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 0)],
                    Left => [Coord(-1, 0), Coord(0, 0), Coord(1, 0), Coord(1, 1)],
                };
                Piece::new('J', coords)
            }

            Tetromino::L => {
                let coords = match dir {
                    Up => [Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 2)],
                    Right => [Coord(0, 1), Coord(0, 0), Coord(1, 0), Coord(2, 0)],
                    Down => [Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(1, 2)],
                    Left => [Coord(-1, 1), Coord(0, 1), Coord(1, 1), Coord(1, 0)],
                };
                Piece::new('L', coords)
            }

            Tetromino::O => {
                let coords = match dir {
                    _ => [Coord(0, 0), Coord(0, 1), Coord(1, 0), Coord(1, 1)],
                };
                Piece::new('O', coords)
            }

            Tetromino::S => {
                let coords = match dir {
                    Up | Down => [Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(-1, 1)],
                    Right | Left => [Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(1, 2)],
                };
                Piece::new('S', coords)
            }

            Tetromino::Z => {
                let coords = match dir {
                    Up | Down => [Coord(0, 0), Coord(-1, 0), Coord(0, 1), Coord(1, 1)],
                    Right | Left => [Coord(0, 0), Coord(0, 1), Coord(-1, 1), Coord(-1, 2)],
                };
                Piece::new('Z', coords)
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
