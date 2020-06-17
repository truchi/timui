use super::{Elements, Layout, Style};
use std::fmt::Debug;

pub trait Component: Debug {
    type Props: Debug;

    fn layout(&self, _props: &Self::Props) -> Layout {
        Default::default()
    }

    fn style(&self, _props: &Self::Props) -> Style {
        Default::default()
    }

    fn children(&self, _props: &Self::Props) -> Elements {
        Default::default()
    }
}

impl Component for () {
    type Props = ();
}
