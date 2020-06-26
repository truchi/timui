use crate::component::ElementObject;
use crate::paint::Paint;
use crate::style::{ColorStyleInherited, LayoutStyle};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Weak;
use stretch::node::Node as Id;
use stretch::node::Stretch;
use stretch::result::Layout;

pub struct UIElement {
    pub id: Id,
    pub element: ElementObject,
    pub layout: Option<Layout>,
    pub layout_style: LayoutStyle,
    pub color_style: ColorStyleInherited,
    pub paint: Option<Paint>,
}

impl UIElement {
    pub fn layout(&mut self, stretch: &Stretch, parent_layout: Option<Layout>) {
        let mut layout = *stretch.layout(self.id).unwrap();

        if let Some(parent_layout) = parent_layout {
            layout.location.x += parent_layout.location.x;
            layout.location.y += parent_layout.location.y;
        }

        self.layout = Some(layout);
    }

    pub fn paint(&mut self) {
        self.paint = Some((self.layout.unwrap(), self.color_style).into());
    }
}

impl Debug for UIElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <ElementObject as Debug>::fmt(&self.element, f)
    }
}
