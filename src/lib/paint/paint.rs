use super::{Canvas, Cell, Uniform};
use crate::component::Content;
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
    content: Canvas,
    // borders: Borders,
}

impl Paint {
    pub fn above(&self, canvas: &mut Canvas) {
        canvas.above(&self.background);
        canvas.above(&self.content);
    }

    pub fn below(&self, canvas: &mut Canvas) {
        canvas.below(&self.background);
        canvas.below(&self.content);
    }
}

impl From<(Layout, ColorStyleInherited, Content)> for Paint {
    fn from((layout, style, content): (Layout, ColorStyleInherited, Content)) -> Self {
        let content = match content {
            Content::None => Default::default(),
            Content::Char(c) => (layout, style, c).into(),
            _ => Default::default(),
        };

        let background = (layout, style, ' ').into();

        Self {
            content,
            background,
        }
    }
}
