use super::layout::Layout;
use super::style::Style;
use super::view::View;
use std::fmt::Debug;

pub trait Component: Default {
    type Props: Default + Debug;
    type Children: Default + Debug;

    fn layout(&self, _props: &Self::Props, _children: &Self::Children) -> Layout {}

    fn style(&self, _props: &Self::Props, _children: &Self::Children) -> Style {
        Style::default()
    }

    fn view(&self, _props: Self::Props, _children: Self::Children) -> View {
        View::default()
    }
}
