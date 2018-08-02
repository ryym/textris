use block::Block;
use coord::{Coord, Dir, Dirs, RotateDir};
use elapsed::Elapsed;
use field::Field;
use rand::{thread_rng, Rng, ThreadRng};
use std::collections::HashMap;
use tetromino::{Tetromino, Tetrominos, N_TETROS};

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

    pub fn random_tetro_pos(&mut self, width: usize) -> Coord {
        // In cases of some Tetrominos and its orientation,
        // we cannot put it at the leftmost or rightmost cell.
        let right_limit = width - 2;
        Coord(self.rng.gen_range(2, right_limit as i8), 0)
    }
}

pub struct Play {
    random: Random<ThreadRng>,
    block_map: HashMap<Tetromino, Block>,
    tetro: Tetromino,
    tetro_dir: Dir,
    tetro_stopped: bool,
    tetro_pos: Coord,
    field: Field,
    elapsed: Elapsed,
    score: u64,
}

impl Play {
    pub fn new() -> Self {
        let mut play = Play {
            random: Random::new(thread_rng()),
            block_map: Play::default_block_map(),
            tetro: Tetromino::I, // temp
            tetro_dir: Default::default(),
            tetro_stopped: false,
            tetro_pos: Default::default(),
            field: Field::new(16, 16),
            elapsed: Elapsed::new(),
            score: 0,
        };
        play.drop_tetro();
        play
    }

    fn default_block_map() -> HashMap<Tetromino, Block> {
        let bm = HashMap::with_capacity(N_TETROS);
        Tetromino::all().into_iter().fold(bm, |mut bm, &t| {
            bm.insert(t, t.default_block());
            bm
        })
    }

    fn drop_tetro(&mut self) {
        self.tetro = self.random.random_tetro();
        self.tetro_dir = self.random.random_tetro_dir();
        self.tetro_pos = self.random.random_tetro_pos(self.field.width());

        let coords = self.tetro.make_coords(self.tetro_pos, self.tetro_dir);
        let block = self.block();
        self.field.render_blocks(block, &coords);
    }

    fn block(&self) -> Block {
        *self.block_map.get(&self.tetro).unwrap()
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

        match self.move_tetro(Dir::Down) {
            Ok(_) => {}
            Err(_) => {
                let n_deleted = self.delete_completed_lines();
                self.score += n_deleted;

                if self.field.is_reached() {
                    return Err(());
                }

                self.drop_tetro();
                self.tetro_stopped = true;
            }
        };
        Ok(())
    }

    pub fn tick(&mut self) {
        self.elapsed.add_secs(1);
    }

    pub fn slide_tetro(&mut self, dir: Dir) {
        if dir != Dir::Up {
            let _ = self.move_tetro(dir);
        }
    }

    pub fn rotate_tetro(&mut self, rotate_dir: RotateDir) {
        let current_coords = self.tetro.make_coords(self.tetro_pos, self.tetro_dir);
        self.field.clear_blocks(&current_coords);

        let dir = rotate_dir.rotate(self.tetro_dir);
        let coords = self.tetro.make_coords(self.tetro_pos, dir);
        let block = self.block();

        if self.field.is_movable(&coords) {
            self.tetro_dir = dir;
            self.field.render_blocks(block, &coords);
        } else {
            self.field.render_blocks(block, &current_coords);
        }
    }

    fn move_tetro(&mut self, dir: Dir) -> Result<(), ()> {
        let current_coords = self.tetro.make_coords(self.tetro_pos, self.tetro_dir);
        self.field.clear_blocks(&current_coords);

        let new_pos = self.tetro_pos + dir.to_coord();
        let coords = self.tetro.make_coords(new_pos, self.tetro_dir);
        let block = self.block();

        if self.field.is_movable(&coords) {
            self.field.render_blocks(block, &coords);
            self.tetro_pos = new_pos;
            Ok(())
        } else {
            self.field.render_blocks(block, &current_coords);
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
