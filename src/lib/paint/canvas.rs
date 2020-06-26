use super::{Cell, Layer};
use crate::style::{Color, ColorStyleInherited};
use std::fmt::{Display, Error, Formatter};
use stretch::result::Layout;

#[derive(Default, Debug)]
pub struct Canvas {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub vec: Vec<Cell>,
}

impl Layer for Canvas {
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

    fn get(&self, x: usize, y: usize) -> &Cell {
        self.vec.get(self.width * y + x).unwrap()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        self.vec.get_mut(self.width * y + x).unwrap()
    }
}

impl Canvas {
    pub fn with_background(x: usize, y: usize, width: usize, height: usize, color: Color) -> Self {
        let mut vec = Vec::with_capacity(width * height);

        for _ in 0..(width * height) {
            vec.push(Cell::with_background(color));
        }

        Self {
            x,
            y,
            width,
            height,
            vec,
        }
    }

    pub fn above(&mut self, above: &impl Layer) {
        let (x, y, w, h) = self.intersect(above);
        let dx = x - self.x;
        let dy = y - self.y;

        for i in 0..w {
            for j in 0..h {
                let cell = self.get_mut(i + dx, j + dy);
                *cell = cell.above(&above.get(i, j));
            }
        }
    }

    pub fn below(&mut self, below: &impl Layer) {
        let (x, y, w, h) = self.intersect(below);
        let dx = x - self.x;
        let dy = y - self.y;

        for i in 0..w {
            for j in 0..h {
                let cell = self.get_mut(i + dx, j + dy);
                let above = below.get(i, j);
                let merged = cell.below(&above);
                *cell = merged;
            }
        }
    }
}

impl From<(Layout, ColorStyleInherited, char)> for Canvas {
    fn from((layout, mut style, c): (Layout, ColorStyleInherited, char)) -> Self {
        style.background = Color::Transparent;

        Self {
            x: layout.location.x as usize,
            y: layout.location.y as usize,
            width: 1,
            height: 1,
            vec: vec![(style, c).into()],
        }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Ok(for cell in self.vec.iter() {
            cell.fmt(f).unwrap();
        })
    }
}
