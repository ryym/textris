use coord::{Coord, Dir, Dirs};
use elapsed::Elapsed;
use field::Field;
use piece::Piece;
use rand::{thread_rng, Rng, ThreadRng};
use tetromino::{Tetromino, Tetrominos};

struct Random<R: Rng> {
    rng: R,
    tetros: Tetrominos,
    dirs: Dirs,
}

impl<R> Random<R>
where
    R: Rng,
{
    pub fn new(rng: R) -> Self {
        Random {
            rng,
            tetros: Tetromino::all(),
            dirs: Dir::all(),
        }
    }

    pub fn random_tetro(&mut self) -> Tetromino {
        *self.rng.choose(&self.tetros).unwrap()
    }

    pub fn random_tetro_dir(&mut self) -> Dir {
        *self.rng.choose(&self.dirs).unwrap()
    }

    pub fn random_piece_pos(&mut self, width: usize) -> Coord {
        // In cases of some Tetrominos and its orientation,
        // we cannot put it at the leftmost or rightmost cell.
        let right_limit = width - 2;
        Coord(self.rng.gen_range(2, right_limit as i8), 0)
    }
}

pub struct Play {
    random: Random<ThreadRng>,
    tetro: Tetromino,
    tetro_dir: Dir,
    tetro_stopped: bool,
    piece_pos: Coord,
    field: Field,
    elapsed: Elapsed,
    score: u64,
}

impl Play {
    pub fn new() -> Self {
        let mut play = Play {
            random: Random::new(thread_rng()),
            tetro: Tetromino::I, // temp
            tetro_dir: Default::default(),
            tetro_stopped: false,
            piece_pos: Default::default(),
            field: Field::new(16, 16),
            elapsed: Elapsed::new(),
            score: 0,
        };
        play.drop_tetro();
        play
    }

    fn drop_tetro(&mut self) {
        let random = &mut self.random;
        self.tetro = random.random_tetro();
        self.tetro_dir = random.random_tetro_dir();
        self.piece_pos = random.random_piece_pos(self.field.width());
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn elapsed(&self) -> &Elapsed {
        &self.elapsed
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn update(&mut self) -> Result<(), ()> {
        if self.tetro_stopped {
            self.tetro_stopped = false;
            return Ok(());
        }

        match self.move_piece(Dir::Down) {
            Ok(_) => {}
            Err(_) => {
                self.drop_tetro();
                self.tetro_stopped = true;
                let n_deleted = self.delete_completed_lines();

                // The score is just a number of deleted lines.
                self.score += n_deleted;

                if self.field.is_reached() {
                    return Err(());
                }
            }
        };
        Ok(())
    }

    pub fn tick(&mut self) {
        self.elapsed.add_secs(1);
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

    fn delete_completed_lines(&mut self) -> u64 {
        let targets: Vec<usize> = self.field
            .lines_iter()
            .enumerate()
            .filter(|(_i, line)| line.iter().all(|cell| cell.is_some()))
            .map(|(i, _line)| i)
            .collect();

        for &i in targets.iter() {
            self.field.delete_line(i);
        }

        targets.len() as u64
    }
}
