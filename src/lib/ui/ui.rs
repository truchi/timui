use super::{Context, StretchUINode, UINode};
use crate::{component::ElementObject, paint::Canvas};
use std::fmt::{Debug, Formatter, Result};
use stretch::node::Stretch;

pub struct UI {
    root:    UINode,
    stretch: Stretch,
}

impl UI {
    pub fn compute_layout(&mut self, width: u16, height: u16) {
        self.stretch
            .compute_layout(self.root.get_value().id, stretch::geometry::Size {
                width:  stretch::number::Number::Defined(width as f32),
                height: stretch::number::Number::Defined(height as f32),
            })
            .unwrap();
    }

    pub fn render(&mut self, width: u16, height: u16) {
        self.compute_layout(width, height);

        let ctx = Context {
            stretch: &mut self.stretch,
            canvas:  Canvas::with_background((0, 0, width, height), None),
        };

        let ctx = self
            .root
            .recurs(ctx, UINode::render_before, UINode::render_after);

        println!("{}{}", termion::clear::All, ctx.canvas);
    }
}

impl From<ElementObject> for UI {
    fn from(element: ElementObject) -> Self {
        let StretchUINode(stretch, root) = (Stretch::new(), element).into();

        Self { root, stretch }
    }
}

impl Debug for UI {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <UINode as Debug>::fmt(&self.root, f)
    }
}
