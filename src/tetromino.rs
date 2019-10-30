use crate::block::Block;
use crate::color::Color;
use crate::coord::{Coord, Dir};

pub const N_TETROS: usize = 7;

pub type Tetrominos = [Tetromino; N_TETROS];

pub type TetroCoords = [Coord; 4];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn make_coords(self, base: Coord, dir: Dir) -> TetroCoords {
        let moves = self.make_moves(dir);
        [
            base + moves[0],
            base + moves[1],
            base + moves[2],
            base + moves[3],
        ]
    }

    fn make_moves(self, dir: Dir) -> TetroCoords {
        use crate::coord::Dir::*;

        match self {
            Tetromino::I => match dir {
                Up => [Coord(0, -2), Coord(0, -1), Coord(0, 0), Coord(0, 1)],
                Right => [Coord(-1, 0), Coord(0, 0), Coord(1, 0), Coord(2, 0)],
                Down => [Coord(0, -1), Coord(0, 0), Coord(0, 1), Coord(0, 2)],
                Left => [Coord(1, 0), Coord(0, 0), Coord(-1, 0), Coord(-2, 0)],
            },

            Tetromino::J => match dir {
                Up => [Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(-1, 2)],
                Right => [Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1)],
                Down => [Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 0)],
                Left => [Coord(-1, 0), Coord(0, 0), Coord(1, 0), Coord(1, 1)],
            },

            Tetromino::L => match dir {
                Up => [Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 2)],
                Right => [Coord(0, 1), Coord(0, 0), Coord(1, 0), Coord(2, 0)],
                Down => [Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(1, 2)],
                Left => [Coord(-1, 1), Coord(0, 1), Coord(1, 1), Coord(1, 0)],
            },

            Tetromino::O => match dir {
                _ => [Coord(0, 0), Coord(0, 1), Coord(1, 0), Coord(1, 1)],
            },

            Tetromino::S => match dir {
                Up | Down => [Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(-1, 1)],
                Right | Left => [Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(1, 2)],
            },

            Tetromino::Z => match dir {
                Up | Down => [Coord(0, 0), Coord(-1, 0), Coord(0, 1), Coord(1, 1)],
                Right | Left => [Coord(0, 0), Coord(0, 1), Coord(-1, 1), Coord(-1, 2)],
            },

            Tetromino::T => match dir {
                Up => [Coord(0, 0), Coord(0, 1), Coord(-1, 1), Coord(1, 1)],
                Right => [Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(0, 2)],
                Down => [Coord(0, 0), Coord(-1, 0), Coord(1, 0), Coord(0, 1)],
                Left => [Coord(0, 0), Coord(0, 1), Coord(-1, 1), Coord(0, 2)],
            },
        }
    }

    pub fn default_char(self) -> char {
        use crate::tetromino::Tetromino::*;
        match self {
            I => 'I',
            J => 'J',
            L => 'L',
            O => 'O',
            S => 'S',
            T => 'T',
            Z => 'Z',
        }
    }

    pub fn default_block(self) -> Block {
        use crate::tetromino::Tetromino::*;

        let chr = self.default_char();
        match self {
            I => Block::new(chr, Color::red()),
            J => Block::new(chr, Color::blue()),
            L => Block::new(chr, Color::light_red()),
            O => Block::new(chr, Color::yellow()),
            S => Block::new(chr, Color::magenta()),
            T => Block::new(chr, Color::light_blue()),
            Z => Block::new(chr, Color::green()),
        }
    }
}
