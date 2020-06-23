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

    pub fn set_value(&self, value: T) {
        self.borrow_mut().value = value;
    }

    pub fn get_parent(&self) -> Ref<Parent<T>> {
        Ref::map(self.borrow(), |node| &node.parent)
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

    pub fn recurs<C>(&self, mut ctx: C, before: fn(C, &Self) -> C, after: fn(C, &Self) -> C) -> C {
        ctx = before(ctx, self);
        for child in self.borrow().children.iter() {
            ctx = child.recurs(ctx, before, after);
        }
        after(ctx, self)
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
