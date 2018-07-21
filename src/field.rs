use coord::Coord;
use piece::Block;
use std::iter;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

type Line = Vec<Option<Block>>;
type Cells = Vec<Line>;

pub struct Field {
    cells: Cells,
    width: usize,
    height: usize,
}

fn make_line(width: usize) -> Line {
    iter::repeat(None).take(width).collect()
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..height)
            .map(|_| make_line(width))
            .collect();
        Field {
            cells,
            width,
            height,
        }
    }

    pub fn is_in_range(&self, pos: Coord) -> bool {
        let w = self.width as i8;
        let h = self.height as i8;
        0 <= pos.0 && pos.0 < w && 0 <= pos.1 && pos.1 < h
    }

    pub fn lines_iter(&self) -> Iter<Line> {
        self.cells.iter()
    }

    pub fn delete_line(&mut self, idx: usize) {
        self.cells.remove(idx);
        self.cells.insert(0, make_line(self.width));
    }
}

impl Index<Coord> for Field {
    type Output = Option<Block>;

    fn index(&self, index: Coord) -> &Self::Output {
        if !self.is_in_range(index) {
            return &None;
        }
        let Coord(x, y) = index;
        &self.cells[y as usize][x as usize]
    }
}

impl IndexMut<Coord> for Field {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let Coord(x, y) = index;
        &mut self.cells[y as usize][x as usize]
    }
}
