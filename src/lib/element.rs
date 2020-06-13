use super::attributes::Attributes;
use super::component::Component;
use std::fmt::Debug;
use std::rc::Rc;

pub type ElementObject = Box<dyn Element>;
pub type Elements = Vec<ElementObject>;

// ========================

#[derive(Debug)]
pub enum Children {
    Char(char),
    String(Rc<String>),
    Elements(Elements),
}

impl Default for Children {
    fn default() -> Self {
        Self::Elements(Default::default())
    }
}

impl From<char> for Children {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl From<Rc<String>> for Children {
    fn from(s: Rc<String>) -> Self {
        Self::String(s)
    }
}

impl From<ElementObject> for Children {
    fn from(e: ElementObject) -> Self {
        Self::Elements(vec![e])
    }
}

impl From<Elements> for Children {
    fn from(e: Elements) -> Self {
        Self::Elements(e)
    }
}

// ==================

pub trait Element: Debug {
    fn attributes(&self) -> Attributes;

    fn children(&self) -> Children;
}

impl Element for char {
    fn attributes(&self) -> Attributes {
        Default::default()
    }

    fn children(&self) -> Children {
        (*self).into()
    }
}

impl Element for Rc<String> {
    fn attributes(&self) -> Attributes {
        Default::default()
    }

    fn children(&self) -> Children {
        (Rc::clone(self)).into()
    }
}

impl<C> Element for (C, <C as Component>::Props)
where
    C: Component,
{
    fn attributes(&self) -> Attributes {
        let (component, props) = self;
        component.attributes(&props)
    }

    fn children(&self) -> Children {
        let (component, props) = self;
        component.children(props).into()
    }
}

impl<C> Element for C
where
    C: Component,
    <C as Component>::Props: Default,
{
    fn attributes(&self) -> Attributes {
        <C as Component>::attributes(self, &Default::default())
    }

    fn children(&self) -> Children {
        <C as Component>::children(self, &Default::default()).into()
    }
}

impl<E> From<E> for ElementObject
where
    E: Element + 'static,
{
    fn from(e: E) -> Self {
        Box::new(e)
    }
}
