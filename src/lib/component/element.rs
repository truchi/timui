use super::Component;
use crate::style::{Percent, Style};
use std::fmt::Debug;
use std::rc::Rc;

pub type ElementObject = Box<dyn Element>;
pub type Elements = Vec<ElementObject>;

pub enum Content {
    Char(char),
    String(Rc<String>),
    None,
}

pub trait Element: Debug {
    fn style(&self) -> Style {
        Default::default()
    }

    fn children(&self) -> Elements {
        Default::default()
    }

    fn content(&self) -> Content {
        Content::None
    }
}

impl Element for () {
    fn style(&self) -> Style {
        Style::new().none()
    }
}

impl Element for char {
    fn style(&self) -> Style {
        Style::new().width(Percent(1.0)).height(Percent(1.0))
    }

    fn content(&self) -> Content {
        Content::Char(*self)
    }
}

impl Element for Rc<String> {
    fn style(&self) -> Style {
        Style::new()
            .width(Percent(1.0))
            .height(Percent(1.0))
            .background(crate::style::Color::Green)
    }

    fn content(&self) -> Content {
        Content::String(Rc::clone(self))
    }
}

impl<C: Component> Element for C
where
    <C as Component>::Props: Default,
{
    fn style(&self) -> Style {
        Component::style(self, &Default::default())
    }

    fn children(&self) -> Elements {
        Component::children(self, &Default::default())
    }
}

impl<C: Component> Element for (C, <C as Component>::Props) {
    fn style(&self) -> Style {
        Component::style(&self.0, &self.1)
    }

    fn children(&self) -> Elements {
        Component::children(&self.0, &self.1)
    }
}

impl<E: Element + 'static> From<E> for ElementObject {
    fn from(element: E) -> Self {
        Box::new(element)
    }
}
