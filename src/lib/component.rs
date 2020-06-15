use super::element::Elements;
use std::fmt::Debug;

pub trait Component: Debug {
    type Props: Debug;

    fn children(&self, _props: &Self::Props) -> Elements {
        Default::default()
    }
}

impl Component for () {
    type Props = ();
}
