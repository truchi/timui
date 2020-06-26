use super::{Context, StretchUINode, UINode};
use crate::component::ElementObject;
use crate::paint::Canvas;
use crate::style::{Color, Defined, Size};
use std::fmt::{Debug, Formatter, Result};
use stretch::node::Stretch;

pub struct UI {
    root: UINode,
    stretch: Stretch,
}

impl UI {
    pub fn compute_layout(&mut self, width: usize, height: usize) {
        self.stretch
            .compute_layout(
                self.root.get_value().id,
                Size {
                    width: Defined(width as f32),
                    height: Defined(height as f32),
                },
            )
            .unwrap();
    }

    pub fn render(&mut self, width: usize, height: usize) {
        self.compute_layout(width, height);

        let ctx = Context {
            stretch: &mut self.stretch,
            canvas: Canvas::with_background(0, 0, width, height, Color::Transparent),
        };

        let ctx = self.root.recurs(ctx, UINode::before, UINode::after);
        // println!("{}{}", termion::clear::All, ctx.canvas);
    }
}

impl From<ElementObject> for UI {
    fn from(element: ElementObject) -> Self {
        let StretchUINode(stretch, root) = (Stretch::new(), Default::default(), element).into();

        Self { root, stretch }
    }
}

impl Debug for UI {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <UINode as Debug>::fmt(&self.root, f)
    }
}
