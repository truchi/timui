use super::attributes::Attributes;
use super::element::Elements;
use std::fmt::Debug;

pub trait Component: Debug {
    type Props: Debug;

    fn attributes(&self, _props: &Self::Props) -> Attributes {
        Attributes::default()
    }

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![]
    }
}
