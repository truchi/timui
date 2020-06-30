use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::rc::Weak;

pub type Parent<T> = Weak<RefCell<Tree<T>>>;

#[derive(Debug)]
pub struct Node<T>(Rc<RefCell<Tree<T>>>);

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(Tree {
            value,
            parent: Default::default(),
            children: Default::default(),
        })))
    }

    pub fn get_value(&self) -> Ref<T> {
        Ref::map(self.borrow(), |node| &node.value)
    }

    pub fn get_value_mut(&self) -> RefMut<T> {
        RefMut::map(self.borrow_mut(), |node| &mut node.value)
    }

    pub fn set_value(&self, value: T) {
        self.borrow_mut().value = value;
    }

    pub fn get_parent(&self) -> Option<Self> {
        Weak::upgrade(&self.borrow().parent).map(|parent| Self(parent))
    }

    pub fn set_parent(&self, node: &Self) {
        self.borrow_mut().parent = node.as_parent();
    }

    pub fn get_children(&self) -> Ref<Vec<Self>> {
        Ref::map(self.borrow(), |node| &node.children)
    }

    pub fn set_children(&self, children: Vec<Node<T>>) {
        self.borrow_mut().children = children;
    }

    pub fn recurs<C>(&self, mut ctx: C, before: fn(&Self, C) -> C, after: fn(&Self, C) -> C) -> C {
        ctx = before(self, ctx);
        for child in self.borrow().children.iter().rev() {
            ctx = child.recurs(ctx, before, after);
        }
        after(self, ctx)
    }

    fn borrow(&self) -> Ref<Tree<T>> {
        self.0.borrow()
    }

    fn borrow_mut(&self) -> RefMut<Tree<T>> {
        self.0.borrow_mut()
    }

    fn as_parent(&self) -> Parent<T> {
        Rc::downgrade(&self.0)
    }
}

#[derive(Debug)]
pub struct Tree<T> {
    value: T,
    parent: Parent<T>,
    children: Vec<Node<T>>,
}
