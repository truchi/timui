use super::Cell;

pub trait Layer {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> &Cell;
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell;

    fn intersect(&self, other: &impl Layer) -> (usize, usize, usize, usize) {
        let (self_x1, self_y1, self_w, self_h) = (self.x(), self.y(), self.width(), self.height());
        let (other_x1, other_y1, other_w, other_h) =
            (other.x(), other.y(), other.width(), other.height());
        let self_x2 = self_x1 + self_w;
        let self_y2 = self_y1 + self_h;
        let other_x2 = other_x1 + other_w;
        let other_y2 = other_y1 + other_h;

        let x = other_x1.max(self_x1).min(self_x2);
        let y = other_y1.max(self_y1).min(self_y2);
        let w = (self_x2.min(other_x2) - x).max(0);
        let h = (self_y2.min(other_y2) - y).max(0);

        (x, y, w, h)
    }
}
