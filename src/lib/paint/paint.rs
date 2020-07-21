use super::{Canvas, Uniform};
use crate::{component::Content, style::ColorStyle};
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

/// A `Paint`ed `Component` (background, content)
#[derive(Default)]
pub struct Paint {
    background: Uniform,
    content:    Canvas,
    // borders: Borders,
}

impl Paint {
    /// Paint `self` above `canvas`
    pub fn above(&self, canvas: &mut Canvas) {
        canvas.above(&self.background);
        canvas.above(&self.content);
    }

    /// Paint `self` below `canvas`
    pub fn below(&self, canvas: &mut Canvas) {
        canvas.below(&self.background);
        canvas.below(&self.content);
    }
}

/// Creates a `Paint`
impl From<(Layout, ColorStyle, Content)> for Paint {
    fn from((layout, style, content): (Layout, ColorStyle, Content)) -> Self {
        let content = match content {
            Content::None => Default::default(),
            Content::Char(char) => (layout, style, char).into(),
            Content::String(string) => (layout, style, string).into(),
        };
        let background = (layout, style.into()).into();

        Self {
            content,
            background,
        }
    }
}
