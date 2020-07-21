use super::{Cell, Rect};

/// `Layer` trait providing function to retrieve `Cell`s
pub trait Layer {
    fn rect(&self) -> Rect;
    fn get(&self, x: u16, y: u16) -> &Cell;
    fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell;
}
