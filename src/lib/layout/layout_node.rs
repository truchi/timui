use std::cell::RefCell;
use std::rc::Rc;
use stretch::geometry::Size;
use stretch::node::{Node, Stretch};
use stretch::number::Number;
use stretch::result::Layout;
use stretch::style::Style;

pub type NodeId = Node;

pub struct LayoutNode {
    id: NodeId,
    stretch: Rc<RefCell<Stretch>>,
}

impl LayoutNode {
    pub fn new() -> Self {
        let stretch = Rc::new(RefCell::new(Stretch::new()));
        let id = stretch
            .borrow_mut()
            .new_node(Default::default(), Default::default())
            .unwrap();

        Self { id, stretch }
    }

    pub fn new_child(&self) -> Self {
        let stretch = Rc::clone(&self.stretch);
        let id = stretch
            .borrow_mut()
            .new_node(Default::default(), Default::default())
            .unwrap();

        Self { id, stretch }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn set_style(&self, style: Style) {
        self.stretch.borrow_mut().set_style(self.id, style).unwrap();
    }

    pub fn set_children(&self, children: Vec<NodeId>) {
        self.stretch
            .borrow_mut()
            .set_children(self.id, children)
            .unwrap();
    }

    pub fn compute_layout(&self, size: Size<Number>) {
        self.stretch
            .borrow_mut()
            .compute_layout(self.id, size)
            .unwrap();
    }

    pub fn layout(&self) -> Layout {
        *self.stretch.borrow_mut().layout(self.id).unwrap()
    }
}
