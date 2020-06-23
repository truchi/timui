use crate::component::ElementObject;
use crate::paint::Paint;
use crate::style::{ColorStyleInherited, Number, Size, Style};
use crate::utils::tree::Node;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;
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

impl Debug for UIElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <ElementObject as Debug>::fmt(&self.element, f)
    }
}

struct Context<'ui> {
    stretch: &'ui mut Stretch,
    inherited: ColorStyleInherited,
}

impl UINode {
    fn before<'ui>(&self, ctx: Context<'ui>) -> Context<'ui> {
        println!("before");

        let layout = ctx.stretch.layout(self.get_value().id).unwrap();
        let style = self.get_value().style.color.inherit(ctx.inherited);
        self.get_value_mut().paint = Paint::from((*layout, style));

        ctx
    }

    fn after<'ui>(&self, ctx: Context<'ui>) -> Context<'ui> {
        println!("after");
        ctx
    }
}

pub struct UI {
    root: UINode,
    stretch: Stretch,
}

impl UI {
    pub fn compute_layout(&mut self, size: Size<Number>) {
        self.stretch
            .compute_layout(self.root.get_value().id, size)
            .unwrap();
    }

    pub fn render(&mut self, size: Size<Number>) {
        self.compute_layout(size);

        let ctx = Context {
            stretch: &mut self.stretch,
            inherited: Default::default(),
        };

        self.root.recurs(ctx, UINode::before, UINode::after);
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
