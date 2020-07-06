//! # `Element`

use super::Component;
use crate::style::{Percent, Style};
use std::{fmt::Debug, rc::Rc};

/// `Element` trait object
pub type ElementObject = Box<dyn Element>;
/// `Vec` of `Element` trait objects
pub type Elements = Vec<ElementObject>;

/// Inner `Content` for `Component`s
pub enum Content {
    /// `char`
    Char(char),
    /// `Rc<String>`
    String(Rc<String>),
    /// `None`
    None,
}

/// # `Element` trait
///
/// The result of applying its `Props` to a `Component`.
pub trait Element: Debug {
    /// Returns the `Style` for this `Element`
    fn style(&self) -> Style {
        Default::default()
    }

    /// Returns the children `Elements` of this `Element`
    fn children(&self) -> Elements {
        Default::default()
    }

    /// Returns the inner `Content` of this `Element`
    fn content(&self) -> Content {
        Content::None
    }
}

/// `Unit` is an empty element
impl Element for () {
    /// `Unit` is `Display::None`
    fn style(&self) -> Style {
        Style::new().none()
    }
}

/// `char` is an `Element`
impl Element for char {
    /// `char` is 100% width, 100% height
    fn style(&self) -> Style {
        Style::new().width(Percent(1.0)).height(Percent(1.0))
    }

    /// `char`'s content is this `char`
    fn content(&self) -> Content {
        Content::Char(*self)
    }
}

/// `Rc<String>` is an Element
impl Element for Rc<String> {
    /// `Rc<String>` is 100% width, 100% height
    fn style(&self) -> Style {
        Style::new().width(Percent(1.0)).height(Percent(1.0))
    }

    /// `Rc<String>`'s content is this `Rc<String>`
    fn content(&self) -> Content {
        Content::String(Rc::clone(self))
    }
}

/// `Component` with `Default` `Props` is an `Element`
impl<C: Component> Element for C
where
    <C as Component>::Props: Default,
{
    /// `Component`'s `Style` from `Component::style`
    fn style(&self) -> Style {
        Component::style(self, &Default::default())
    }

    /// `Component`'s children from `Component::children`
    fn children(&self) -> Elements {
        Component::children(self, &Default::default())
    }
}

/// `(Component, Props)` is an `Element`
impl<C: Component> Element for (C, <C as Component>::Props) {
    /// `(Component, Props)`'s `Style` from  `Component::style`
    fn style(&self) -> Style {
        Component::style(&self.0, &self.1)
    }

    /// `(Component, Props)`'s children from `Component::children`
    fn children(&self) -> Elements {
        Component::children(&self.0, &self.1)
    }
}

/// `Element` -> `ElementObject` conversion
impl<E: Element + 'static> From<E> for ElementObject {
    fn from(element: E) -> Self {
        Box::new(element)
    }
}
