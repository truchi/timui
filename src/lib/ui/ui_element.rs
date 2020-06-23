use crate::component::ElementObject;
use crate::paint::Paint;
use crate::style::Style;
use std::fmt::{Debug, Formatter, Result};
use stretch::node::Node as Id;

pub struct UIElement {
    pub id: Id,
    pub element: ElementObject,
    pub style: Style,
    pub paint: Paint,
}

impl Debug for UIElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <ElementObject as Debug>::fmt(&self.element, f)
    }
}
