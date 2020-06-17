use super::component::{ElementObject, Number, Size};
use super::utils::tree::Node;
use crate::layout::LayoutNode;
use std::fmt::{Debug, Formatter, Result};

struct Wrap(ElementObject, LayoutNode);
type UINode = Node<UIElement>;

pub struct UIElement {
    element: ElementObject,
    layout_node: LayoutNode,
}

impl UIElement {
    pub fn new(element: ElementObject, layout_node: LayoutNode) -> Self {
        Self {
            element,
            layout_node,
        }
    }
}

impl Debug for UIElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <ElementObject as Debug>::fmt(&self.element, f)
    }
}

pub struct UI {
    pub root: UINode,
}

impl UI {
    pub fn compute_layout(&self, size: Size<Number>) {
        self.root.layout_node(|x| x.compute_layout(size));
    }
}

impl From<ElementObject> for UI {
    fn from(element: ElementObject) -> Self {
        let root: UINode = Wrap(element, LayoutNode::new()).into();

        Self { root }
    }
}

impl Debug for UI {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <UINode as Debug>::fmt(&self.root, f)
    }
}

impl UINode {
    pub fn element<U, F: FnOnce(&ElementObject) -> U>(&self, f: F) -> U {
        self.get(|ui_element| f(&ui_element.element))
    }

    pub fn layout_node<U, F: FnOnce(&LayoutNode) -> U>(&self, f: F) -> U {
        self.get(|ui_element| f(&ui_element.layout_node))
    }
}

impl From<Wrap> for UINode {
    fn from(Wrap(element, layout_node): Wrap) -> Self {
        let layout = element.layout();
        let elements = element.children();
        let node = Self::new(UIElement::new(element, layout_node));
        node.layout_node(|x| x.set_style(layout));

        let len = elements.len();
        let mut children = Vec::with_capacity(len);
        let mut layout_children = Vec::with_capacity(len);
        for element in elements {
            let child: UINode = Wrap(element, node.layout_node(LayoutNode::new_child)).into();
            child.set_parent(&node);

            layout_children.push(child.layout_node(LayoutNode::id));
            children.push(child);
        }

        node.set_children(children);
        node.layout_node(|x| x.set_children(layout_children));

        node
    }
}
