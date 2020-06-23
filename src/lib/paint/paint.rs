use super::{Cell, Layer, Uniform};
use crate::style::ColorStyleInherited;
use stretch::result::Layout;

// #[derive(Default)]
// pub struct Borders {
// border_start: Layer,
// border_end: Layer,
// border_top: Layer,
// border_bottom: Layer,
// border_start_top: Layer,
// border_start_bottom: Layer,
// border_end_top: Layer,
// border_end_bottom: Layer,
// }

#[derive(Default)]
pub struct Paint {
    background: Layer,
    // borders: Borders,
}

impl From<(Layout, ColorStyleInherited)> for Paint {
    fn from((layout, style): (Layout, ColorStyleInherited)) -> Self {
        Self {
            background: Layer::Uniform(Uniform {
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
                    c: (),
                },
            }),
        }
    }
}
