use super::super::component::Component;
use super::super::component::WithChildren;
use super::super::component::WithProps;
use super::super::ui::UI;
use super::super::{children, props};

pub struct Char {
    props: char,
    children: (),
}

props!(Char, char);
children!(Char, ());

impl Component for Char {
    fn ui(&self, props: &Self::Props, _children: &Self::Children) -> UI {
        UI::from(props)
    }
}
