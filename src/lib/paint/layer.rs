use super::Cell;

pub trait Layer {
    fn get(&self, x: u16, y: u16) -> &Cell;
    fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell;
}
