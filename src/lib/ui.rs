use super::element::ElementObject;
use super::layout::Layout;
use super::style::Style;
use super::utils::tree::Tree;
use super::view::ViewElement;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub type UITree = Tree<UIElement>;
pub type UINode = Rc<RefCell<UITree>>;
pub type UIParent = Weak<RefCell<UITree>>;

#[derive(Debug)]
pub enum UIElement {
    Char(char),
    String(String),
    Element { layout: Layout, style: Style },
}

#[derive(Debug)]
pub struct UI(pub UINode);

fn from(parent: UIParent, view_element: ViewElement) -> UINode {
    let (value, view_elements) = match view_element {
        ViewElement::Char(c) => (UIElement::Char(c), vec![]),
        ViewElement::String(s) => (UIElement::String(s), vec![]),
        ViewElement::Element(view_element) => (
            UIElement::Element {
                layout: view_element.layout(),
                style: view_element.style(),
            },
            view_element.view().into(),
        ),
    };

    let node = Rc::new(RefCell::new(UITree::new(value, parent)));
    {
        node.borrow_mut().children = view_elements
            .into_iter()
            .map(|view_element| from(Rc::downgrade(&node), view_element))
            .collect::<Vec<_>>();
    }

    node
}

impl From<ElementObject> for UI {
    fn from(element_object: ElementObject) -> Self {
        UI(from(
            UIParent::default(),
            ViewElement::Element(element_object),
        ))
    }
}
