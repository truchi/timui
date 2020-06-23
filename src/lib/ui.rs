use crate::component::ElementObject;
use crate::paint::Paint;
use crate::style::Style;
use crate::style::{Number, Size};
use crate::utils::tree::Node;
use std::fmt::{Debug, Formatter, Result};
use stretch::node::Node as Id;
use stretch::node::Stretch;

pub type UINode = Node<UIElement>;
struct StretchUINode(Stretch, UINode);

pub struct UIElement {
    id: Id,
    element: ElementObject,
    style: Style,
    paint: Paint,
}

impl UIElement {
    // pub fn new(element: ElementObject, layout_node: LayoutNode, paint: Paint) -> Self {
    // Self {
    // element,
    // layout_node,
    // paint,
    // }
    // }
}

impl Debug for UIElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <ElementObject as Debug>::fmt(&self.element, f)
    }
}

pub struct UI {
    root: UINode,
    stretch: Stretch,
}

impl UI {
    pub fn compute_layout(&self, size: Size<Number>) {
        // self.root.layout_node(|x| x.compute_layout(size));
    }

    pub fn render(&self, size: Size<Number>) {
        self.compute_layout(size);

        fn before(i: usize, node: &UINode) -> usize {
            println!("I: {}", i);
            // println!("I: {}, BEFORE {:#?}", i, node.element(|x| x.style()));
            ();
            // let style = node.element(|x| x.style());
            // let layout = node.layout_node(|x| x.layout());
            // node.get_mut(|ui_element| ui_element.paint = Paint::new(layout, style));
            i + 1
        }

        fn after(i: usize, node: &UINode) -> usize {
            println!("I: {}", i);
            // println!("I: {}, AFTER {:#?}", i, node.element(|x| x.style()));
            i - 1
        }

        self.root.recurs(0, before, after);
    }
}

impl From<ElementObject> for UI {
    fn from(element: ElementObject) -> Self {
        let StretchUINode(stretch, root) = (Stretch::new(), element).into();

        Self { root, stretch }
    }
}

impl Debug for UI {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <UINode as Debug>::fmt(&self.root, f)
    }
}

impl UINode {
    // pub fn element<U, F: FnOnce(&ElementObject) -> U>(&self, f: F) -> U {
    // self.get(|ui_element| f(&ui_element.element))
    // }
    //
    // pub fn layout_node<U, F: FnOnce(&LayoutNode) -> U>(&self, f: F) -> U {
    // self.get(|ui_element| f(&ui_element.layout_node))
    // }
    //
    // pub fn paint<U, F: FnOnce(&Paint) -> U>(&self, f: F) -> U {
    // self.get(|ui_element| f(&ui_element.paint))
    // }

    // pub fn new(element: ElementObject) ->Self {
    //
    // }
}

impl From<(Stretch, ElementObject)> for StretchUINode {
    fn from((mut stretch, element): (Stretch, ElementObject)) -> Self {
        let style = element.style();
        let elements = element.children();
        let id = stretch.new_node(style.layout, Default::default()).unwrap();
        let node = UINode::new(UIElement {
            id,
            element,
            style,
            paint: Default::default(),
        });

        let len = elements.len();
        let (mut stretch, children, children_ids) = elements.into_iter().fold(
            (stretch, Vec::with_capacity(len), Vec::with_capacity(len)),
            |(stretch, mut children, mut children_ids), element| {
                let Self(stretch, child) = (stretch, element).into();

                child.set_parent(&node);
                children_ids.push(child.get_value().id);
                children.push(child);

                (stretch, children, children_ids)
            },
        );

        node.set_children(children);
        stretch.set_children(id, children_ids).unwrap();

        Self(stretch, node)
    }
}
