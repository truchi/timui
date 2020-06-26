use super::{Cell, Layer};

#[derive(Default, Debug)]
pub struct Uniform {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub cell: Cell,
}

impl Layer for Uniform {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, _: usize, _: usize) -> &Cell {
        &self.cell
    }

    fn get_mut(&mut self, _: usize, _: usize) -> &mut Cell {
        &mut self.cell
    }
}