use super::{Cell, Layer, Rect};

/// A uniform `Layer`: same cell all over the layer
#[derive(Default, Debug)]
pub struct Uniform {
    /// The `Rect` bounds of the `Layer`
    rect: Rect,
    /// The `Cell` filling the `Layer`
    cell: Cell,
}

impl Uniform {
    /// Creates a `Uniform`
    pub fn new(rect: impl Into<Rect>, cell: Cell) -> Self {
        Self {
            rect: rect.into(),
            cell,
        }
    }
}

impl Layer for Uniform {
    fn rect(&self) -> Rect {
        self.rect
    }

    /// Returns `cell`, ignores arguments
    fn get(&self, _: u16, _: u16) -> &Cell {
        &self.cell
    }

    /// Returns `cell`, ignores arguments
    fn get_mut(&mut self, _: u16, _: u16) -> &mut Cell {
        &mut self.cell
    }
}
