use super::element::ElementObject;
use super::utils::tree::Tree;
use std::cell::RefCell;
use std::rc::Rc;

pub type UITree = Tree<ElementObject>;
pub type UI = Rc<RefCell<UITree>>;

impl From<ElementObject> for UI {
    fn from(element: ElementObject) -> Self {
        let children = element.children();
        let node = Rc::new(RefCell::new(UITree::new(element, Default::default())));

        node.borrow_mut().children = children
            .into_iter()
            .map(|element| {
                let node: UI = element.into();
                node.borrow_mut().parent = Rc::downgrade(&node);
                node
            })
            .collect::<Vec<_>>();

        node
    }
}
