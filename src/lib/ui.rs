use super::element::Children;
use super::element::ElementObject;
use super::utils::tree::Tree;
use std::cell::RefCell;
use std::rc::Rc;

pub type UITree = Tree<UIElement>;
pub type UI = Rc<RefCell<UITree>>;

#[derive(Debug)]
pub enum UIElement {
    Char(char),
    String(Rc<String>),
    Element(ElementObject),
}

impl From<ElementObject> for UI {
    fn from(element: ElementObject) -> Self {
        let children = element.children();
        let node = Rc::new(RefCell::new(UITree::new(
            UIElement::Element(element),
            Default::default(),
        )));

        node.borrow_mut().children = match children {
            Children::Char(c) => vec![Rc::new(RefCell::new(UITree::new(
                UIElement::Char(c),
                Rc::downgrade(&node),
            )))],
            Children::String(s) => vec![Rc::new(RefCell::new(UITree::new(
                UIElement::String(s),
                Rc::downgrade(&node),
            )))],
            Children::Elements(elements) => elements
                .into_iter()
                .map(|element| {
                    let node: UI = element.into();
                    node.borrow_mut().parent = Rc::downgrade(&node);
                    node
                })
                .collect::<Vec<_>>(),
        };

        node
    }
}
