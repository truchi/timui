use super::element::ElementObject;
use super::layout::Layout;
use super::style::Style;
use super::view::View;

#[derive(Debug)]
pub enum Content {
    Empty,
    Char(char),
    List(Vec<UI>),
}

#[derive(Debug)]
pub struct UI {
    layout: Layout,
    style: Style,
    content: Content,
}

impl From<View> for Content {
    fn from(view: View) -> Self {
        match view {
            View::Empty => Self::Empty,
            View::Char(c) => Self::Char(c),
            View::List(list) => {
                Self::List(list.into_iter().map(|element| element.into()).collect())
            }
        }
    }
}

impl From<ElementObject> for UI {
    fn from(element: ElementObject) -> Self {
        Self {
            layout: element.layout(),
            style: element.style(),
            content: element.view().into(),
        }
    }
}
