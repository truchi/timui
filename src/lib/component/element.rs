use crate::component::{Component, Dimension, Layout, Size, Style};
use std::fmt::Debug;

pub type ElementObject = Box<dyn Element>;
pub type Elements = Vec<ElementObject>;

pub trait Element: Debug {
    fn layout(&self) -> Layout {
        Default::default()
    }

    fn style(&self) -> Style {
        Default::default()
    }

    fn children(&self) -> Elements {
        Default::default()
    }
}

impl Element for char {
    fn layout(&self) -> Layout {
        Layout {
            size: Size {
                width: Dimension::Points(1.0),
                height: Dimension::Points(1.0),
            },
            ..Default::default()
        }
    }
}

impl Element for String {}

impl<C: Component> Element for C
where
    <C as Component>::Props: Default,
{
    fn layout(&self) -> Layout {
        Component::layout(self, &Default::default())
    }

    fn style(&self) -> Style {
        Component::style(self, &Default::default())
    }

    fn children(&self) -> Elements {
        Component::children(self, &Default::default())
    }
}

impl<C: Component> Element for (C, <C as Component>::Props) {
    fn layout(&self) -> Layout {
        Component::layout(&self.0, &self.1)
    }

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
