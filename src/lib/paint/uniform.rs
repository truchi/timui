use super::{Cell, Layer};
use crate::utils::rect::Rect;

/// A uniform `Layer`: same cell all over the layer
#[derive(Default, Debug)]
pub struct Uniform {
    /// The `Rect` bounds of the `Layer`
    rect: Rect,
    /// The `Cell` filling the `Layer`
    cell: Cell,
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

/// Creates a `Uniform`
impl<T: Into<Rect>> From<(T, Cell)> for Uniform {
    fn from((rect, cell): (T, Cell)) -> Self {
        let rect = rect.into();

        Self {
            rect: rect.into(),
            cell,
        }
    }
}
