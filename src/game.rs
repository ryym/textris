use coord::{Coord, Dir, Dirs};
use field::Field;
use piece::Piece;
use rand::{thread_rng, Rng, ThreadRng};
use tetromino::{Tetromino, Tetrominos};

pub struct Game {
    rng: ThreadRng,
    tetros: Tetrominos,
    dirs: Dirs,
    tetro: Tetromino,
    tetro_dir: Dir,
    tetro_stopped: bool,
    piece_pos: Coord,
    field: Field,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            rng: thread_rng(),
            tetros: Tetromino::all(),
            dirs: Dir::all(),
            tetro: Tetromino::I, // temp
            tetro_dir: Default::default(),
            tetro_stopped: false,
            piece_pos: Default::default(),
            field: Field::new(16, 16),
        };
        game.drop_tetro();
        game
    }

    fn random_tetro(&mut self) -> Tetromino {
        *self.rng.choose(&self.tetros).unwrap()
    }

    fn random_tetro_dir(&mut self) -> Dir {
        *self.rng.choose(&self.dirs).unwrap()
    }

    fn random_piece_pos(&mut self) -> Coord {
        // In cases of some Tetrominos and its orientation,
        // we cannot put it at the leftmost or rightmost cell.
        let right_limit = self.field.width() - 2;
        Coord(self.rng.gen_range(2, right_limit as i8), 0)
    }

    fn drop_tetro(&mut self) {
        self.tetro = self.random_tetro();
        self.tetro_dir = self.random_tetro_dir();
        self.piece_pos = self.random_piece_pos();
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn tick(&mut self) -> Result<(), ()> {
        if self.tetro_stopped {
            self.tetro_stopped = false;
            return Ok(());
        }

        match self.move_piece(Dir::Down) {
            Ok(_) => {}
            Err(_) => {
                self.drop_tetro();
                self.tetro_stopped = true;
                self.delete_completed_lines();

                if self.field.is_reached() {
                    return Err(());
                }
            }
        };
        Ok(())
    }

    pub fn slide_piece(&mut self, dir: Dir) {
        if dir != Dir::Up {
            let _ = self.move_piece(dir);
        }
    }

    pub fn rotate_piece(&mut self, clockwise: bool) {
        let current_coords = self.make_piece().coords(self.piece_pos);
        self.field.clear_blocks(&current_coords);

        let dir = self.tetro_dir.next_dir(clockwise);
        let piece = self.tetro.make_piece(dir);
        let coords = piece.coords(self.piece_pos);

        if self.field.is_movable(&coords) {
            self.tetro_dir = dir;
            self.field.render_blocks(piece.block(), &coords);
        } else {
            self.field.render_blocks(piece.block(), &current_coords);
        }
    }

    fn make_piece(&self) -> Piece {
        self.tetro.make_piece(self.tetro_dir)
    }

    fn move_piece(&mut self, dir: Dir) -> Result<(), ()> {
        let piece = self.make_piece();
        let current_coords = piece.coords(self.piece_pos);
        self.field.clear_blocks(&current_coords);

        let new_pos = self.piece_pos + dir.to_coord();
        let coords = piece.coords(new_pos);

        if self.field.is_movable(&coords) {
            self.field.render_blocks(piece.block(), &coords);
            self.piece_pos = new_pos;
            Ok(())
        } else {
            self.field.render_blocks(piece.block(), &current_coords);
            Err(())
        }
    }

    fn delete_completed_lines(&mut self) {
        let targets: Vec<usize> = self.field
            .lines_iter()
            .enumerate()
            .filter(|(_i, line)| line.iter().all(|cell| cell.is_some()))
            .map(|(i, _line)| i)
            .collect();

        for i in targets.into_iter() {
            self.field.delete_line(i);
        }
    }
}
