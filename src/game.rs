use coord::Coord;
use field::Field;
use piece::{Block, Piece};

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
            field: Field::new(8, 16),
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn tick(&mut self) {
        self.try_move_piece().unwrap();
        // Delete no-gap lines.
    }

    fn try_move_piece(&mut self) -> Result<(), ()> {
        let new_pos = self.piece_pos + Coord(0, 1);
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
