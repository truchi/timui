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

    pub fn get<U, F: FnOnce(&T) -> U>(&self, f: F) -> U {
        f(&self.borrow().value)
    }

    pub fn get_mut<U, F: FnOnce(&mut T) -> U>(&self, f: F) -> U {
        f(&mut self.borrow_mut().value)
    }

    pub fn set(&self, value: T) {
        self.borrow_mut().value = value;
    }

    pub fn map<F: FnOnce(&mut T) -> T>(&self, f: F) {
        self.set(self.get_mut(f));
    }

    pub fn set_parent(&self, node: &Self) {
        self.borrow_mut().parent = node.as_parent();
    }

    pub fn set_children(&self, children: Vec<Node<T>>) {
        self.borrow_mut().children = children;
    }

    pub fn borrow(&self) -> Ref<Tree<T>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Tree<T>> {
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
    pub children: Vec<Node<T>>,
}
