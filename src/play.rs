use crate::block::Block;
use crate::color::Color;
use crate::coord::{Coord, Dir, Dirs, RotateDir};
use crate::elapsed::Elapsed;
use crate::field::Field;
use crate::tetromino::{Tetromino, Tetrominos, N_TETROS};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

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
        Coord(self.rng.gen_range(0, width as i8), 0)
    }
}

pub struct Play {
    random: Random<ThreadRng>,
    block_map: HashMap<Tetromino, Block>,
    tetro: Tetromino,
    next_tetro: Tetromino,
    tetro_dir: Dir,
    tetro_stopped: bool,
    tetro_pos: Coord,
    deletables: Option<Vec<usize>>,
    field: Field,
    elapsed: Elapsed,
    score: usize,
}

impl Default for Play {
    fn default() -> Self {
        let mut random = Random::new(thread_rng());
        let next_tetro = random.random_tetro();

        let mut play = Play {
            random,
            block_map: Play::default_block_map(),
            tetro: Tetromino::I, // temp
            next_tetro,
            tetro_dir: Default::default(),
            tetro_stopped: false,
            tetro_pos: Default::default(),
            deletables: None,
            field: Field::new(16, 16),
            elapsed: Elapsed::new(),
            score: 0,
        };
        play.drop_tetro();
        play
    }
}

impl Play {
    pub fn new() -> Self {
        Play::default()
    }

    fn default_block_map() -> HashMap<Tetromino, Block> {
        let bm = HashMap::with_capacity(N_TETROS);
        Tetromino::all().iter().fold(bm, |mut bm, &t| {
            bm.insert(t, t.default_block());
            bm
        })
    }

    fn drop_tetro(&mut self) {
        self.tetro = self.next_tetro;
        self.next_tetro = self.random.random_tetro();
        self.tetro_dir = self.random.random_tetro_dir();
        self.tetro_pos = self.random.random_tetro_pos(self.field.width());

        let dir = if self.tetro_pos.x() < (self.field.width() as i8) / 2 {
            Dir::Right
        } else {
            Dir::Left
        };
        let adjustment = dir.to_coord();

        // Find renderable position.
        loop {
            let coords = self.tetro.make_coords(self.tetro_pos, self.tetro_dir);
            if self.field.is_movable(&coords) {
                let block = self.block();
                self.field.render_blocks(block, &coords);
                break;
            } else {
                self.tetro_pos += adjustment;
            }
        }
    }

    fn block(&self) -> Block {
        *self.block_map.get(&self.tetro).unwrap()
    }

    pub fn next_tetro_hint(&self) -> Block {
        *self.block_map.get(&self.next_tetro).unwrap()
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn elapsed(&self) -> &Elapsed {
        &self.elapsed
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn update(&mut self) -> Result<(), ()> {
        if let Some(deletables) = self.deletables.take() {
            for &i in deletables.iter() {
                self.field.delete_line(i);
            }
            self.score += deletables.len();
            self.deletables = None;
            self.drop_tetro();
        }

        if self.tetro_stopped {
            self.tetro_stopped = false;
            return Ok(());
        }

        match self.move_tetro(Dir::Down) {
            Ok(_) => {}
            Err(_) => {
                if self.field.is_reached() {
                    return Err(());
                }

                self.tetro_stopped = true;
                match self.mark_deletable_lines() {
                    None => {
                        self.drop_tetro();
                    }
                    some => {
                        self.deletables = some;
                    }
                };
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
        if self.tetro_stopped {
            return Ok(());
        }

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

    fn mark_deletable_lines(&mut self) -> Option<Vec<usize>> {
        let targets: Vec<usize> = self
            .field
            .lines_iter()
            .enumerate()
            .filter(|(_i, line)| line.iter().all(|cell| cell.is_some()))
            .map(|(i, _line)| i)
            .collect();

        for &y in targets.iter() {
            let marked_line = (0..self.field.width())
                .map(|_| Some(Block::new('X', Color::white())))
                .collect();
            self.field.set_line(y, marked_line);
        }

        Some(targets)
    }
}
