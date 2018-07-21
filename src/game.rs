use coord::{Coord, Dir};
use piece::{Block, Piece};
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
            tetro: Tetromino::T,
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

    pub fn rotate_piece(&mut self, clockwise: bool) {
        let coords = self.make_piece().coords(self.piece_pos);
        self.clear_blocks(&coords);

        self.tetro_dir = self.tetro_dir.next_dir(clockwise);
        let piece = self.make_piece();
        let new_coords = piece.coords(self.piece_pos);
        self.render_blocks(piece.block(), &new_coords);
    }

    pub fn tick(&mut self) {
        self.move_piece(Dir::Down).unwrap();
        // Delete no-gap lines.
    }

    fn make_piece(&self) -> Piece {
        self.tetro.make_piece(self.tetro_dir)
    }

    fn move_piece(&mut self, dir: Dir) -> Result<(), ()> {
        let piece = self.make_piece();
        let new_pos = self.piece_pos + dir.to_coord();
        let coords = piece.coords(new_pos);
        if !coords.iter().all(|c| self.field.is_in_range(*c)) {
            return Err(());
        }

        let current_coords = piece.coords(self.piece_pos);
        self.clear_blocks(&current_coords);
        self.render_blocks(piece.block(), &coords);
        self.piece_pos = new_pos;

        Ok(())
    }

    fn clear_blocks(&mut self, coords: &[Coord]) {
        for pos in coords {
            self.field[*pos] = None;
        }
    }

    fn render_blocks(&mut self, block: Block, coords: &[Coord]) {
        for pos in coords {
            self.field[*pos] = Some(block);
        }
    }
}
