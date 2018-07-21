use coord::{Coord, Dir};
use field::Field;
use tetromino::Tetromino;

pub struct Game {
    tetro: Tetromino,
    tetro_dir: Dir,
    piece_pos: Coord,
    field: Field,
}

impl Game {
    pub fn new() -> Self {
        Game {
            tetro: Tetromino::I,
            tetro_dir: Dir::Up,
            piece_pos: Coord(2, 0),
            field: Field::new(16, 24),
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn slide_piece(&mut self, dir: Dir) {
        if dir != Dir::Up {
            let _ = self.move_piece(dir);
        }
    }

    pub fn tick(&mut self) {
        self.move_piece(Dir::Down).unwrap();
        // Delete no-gap lines.
    }

    fn move_piece(&mut self, dir: Dir) -> Result<(), ()> {
        let piece = self.tetro.make_piece(self.tetro_dir);
        let new_pos = self.piece_pos + dir.to_coord();
        let coords = piece.coords(new_pos);
        if !coords.iter().all(|c| self.field.is_in_range(*c)) {
            return Err(());
        }

        for pos in piece.coords(self.piece_pos) {
            self.field[pos] = None;
        }

        for pos in coords {
            self.field[pos] = Some(piece.block());
        }

        self.piece_pos = new_pos;
        Ok(())
    }
}
