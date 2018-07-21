use coord::Coord;
use field::Field;
use piece::{Block, Piece};

pub enum Dir {
    Left,
    Right,
    Down,
}

impl Dir {
    pub fn to_coord(&self) -> Coord {
        match self {
            Dir::Left => Coord(-1, 0),
            Dir::Right => Coord(1, 0),
            Dir::Down => Coord(0, 1),
        }
    }
}

pub struct Game {
    piece: Piece,
    piece_pos: Coord,
    field: Field,
}

impl Game {
    pub fn new() -> Self {
        let piece = Piece::new(
            Block::new('I'),
            [Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3)],
        );
        Game {
            piece,
            piece_pos: Coord(2, 0),
            field: Field::new(16, 24),
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn slide_piece(&mut self, dir: Dir) {
        let _ = self.try_move_piece(dir);
    }

    pub fn tick(&mut self) {
        self.try_move_piece(Dir::Down).unwrap();
        // Delete no-gap lines.
    }

    fn try_move_piece(&mut self, dir: Dir) -> Result<(), ()> {
        let new_pos = self.piece_pos + dir.to_coord();
        let coords = self.piece.coords(new_pos);
        if !coords.iter().all(|c| self.field.is_in_range(*c)) {
            return Err(());
        }

        for pos in self.piece.coords(self.piece_pos) {
            self.field[pos] = None;
        }

        for pos in coords {
            self.field[pos] = Some(self.piece.block());
        }

        self.piece_pos = new_pos;
        Ok(())
    }
}
