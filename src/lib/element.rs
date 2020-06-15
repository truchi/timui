use super::component::Component;
use std::fmt::Debug;

pub type ElementObject = Box<dyn Element>;
pub type Elements = Vec<ElementObject>;

pub trait Element: Debug {
    fn children(&self) -> Elements {
        Default::default()
    }
}

impl Element for char {}

impl Element for String {}

impl<C: Component> Element for C
where
    <C as Component>::Props: Default,
{
    fn children(&self) -> Elements {
        Component::children(self, &Default::default())
    }
}

impl<C: Component> Element for (C, <C as Component>::Props) {
    fn children(&self) -> Elements {
        Component::children(&self.0, &self.1)
    }
}

impl<E: Element + 'static> From<E> for ElementObject {
    fn from(element: E) -> Self {
        Box::new(element)
    }
}
