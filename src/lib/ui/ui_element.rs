use crate::component::ElementObject;
use crate::paint::Paint;
use crate::style::{ColorStyleInherited, LayoutStyle};
use std::fmt::{Debug, Formatter, Result};
use stretch::node::Node as Id;
use stretch::result::Layout;

pub struct UIElement {
    pub id: Id,
    pub element: ElementObject,
    pub layout: Option<Layout>,
    pub layout_style: LayoutStyle,
    pub color_style: ColorStyleInherited,
    pub paint: Option<Paint>,
}

impl Debug for UIElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <ElementObject as Debug>::fmt(&self.element, f)
    }
}
