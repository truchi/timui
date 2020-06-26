use super::{Canvas, Cell, Uniform};
use crate::style::ColorStyleInherited;
use stretch::result::Layout;

// #[derive(Default)]
// pub struct Borders {
// start: Layer,
// end: Layer,
// top: Layer,
// bottom: Layer,
// start_top: Layer,
// start_bottom: Layer,
// end_top: Layer,
// end_bottom: Layer,
// }

#[derive(Default)]
pub struct Paint {
    background: Uniform,
    // borders: Borders,
}

impl Paint {
    pub fn above(&self, canvas: &mut Canvas) {
        canvas.below(&self.background);
    }

    pub fn below(&self, canvas: &mut Canvas) {
        println!("CANVAS x {:#?}", canvas.x);
        println!("CANVAS y {:#?}", canvas.y);
        println!("CANVAS width {:#?}", canvas.width);
        println!("CANVAS height {:#?}", canvas.height);
        println!("CANVAS vec[0] {:#?}", canvas.vec[0]);
        println!("BACKGROUND {:#?}", self.background);
        canvas.above(&self.background);
    }
}

impl From<(Layout, ColorStyleInherited)> for Paint {
    fn from((layout, style): (Layout, ColorStyleInherited)) -> Self {
        Self {
            background: Uniform {
                x: layout.location.x as usize,
                y: layout.location.y as usize,
                width: layout.size.width as usize,
                height: layout.size.height as usize,
                cell: Cell {
                    foreground: style.foreground,
                    background: style.background,
                    bold: style.bold,
                    italic: style.italic,
                    underline: style.underline,
                    c: ' ',
                },
            },
        }
    }
}
