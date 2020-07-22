use crate::{component::ElementObject, paint::Paint, style::Style};
use std::fmt::{Debug, Formatter, Result};
use stretch::{
    node::{Node as Id, Stretch},
    result::Layout,
};

pub struct UIElement {
    pub id:      Id,
    pub element: ElementObject,
    pub style:   Style,
    pub layout:  Option<Layout>,
    pub paint:   Option<Paint>,
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

    pub fn style(&mut self, parent_style: Option<Style>) {
        if let Some(parent_style) = parent_style {
            self.style.color = self.style.color.inherit(&parent_style.color);
        }
    }

    pub fn paint(&mut self) {
        let layout = self.layout.unwrap();
        let style = self.style.color;
        let content = self.element.content();

        self.paint = Some((layout, style, content).into());
    }
}

impl Debug for UIElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <ElementObject as Debug>::fmt(&self.element, f)
    }
}
