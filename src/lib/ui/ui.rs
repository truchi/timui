use super::{Context, StretchUINode, UINode};
use crate::component::ElementObject;
use crate::style::{Number, Size};
use std::fmt::{Debug, Formatter, Result};
use stretch::node::Stretch;

pub struct UI {
    root: UINode,
    stretch: Stretch,
}

impl UI {
    pub fn compute_layout(&mut self, size: Size<Number>) {
        self.stretch
            .compute_layout(self.root.get_value().id, size)
            .unwrap();
    }

    pub fn render(&mut self, size: Size<Number>) {
        self.compute_layout(size);

        let ctx = Context {
            stretch: &mut self.stretch,
            inherited: Default::default(),
        };

        self.root.recurs(ctx, UINode::before, UINode::after);
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
