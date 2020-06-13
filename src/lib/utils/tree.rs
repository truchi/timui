use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub type Parent<T> = Weak<RefCell<Tree<T>>>;
pub type Node<T> = Rc<RefCell<Tree<T>>>;
pub type Children<T> = Vec<Node<T>>;

#[derive(Debug)]
pub struct Tree<T> {
    pub value: T,
    pub parent: Parent<T>,
    pub children: Children<T>,
}

impl<T> Tree<T> {
    pub fn new(value: T, parent: Parent<T>) -> Self {
        Self {
            value,
            parent,
            children: Children::default(),
        }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}
