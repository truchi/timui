use super::UIElement;
use crate::{component::ElementObject, paint::Canvas, utils::tree::Node};
use std::cell::Ref;
use stretch::node::Stretch;

pub type UINode = Node<UIElement>;
pub struct StretchUINode(pub Stretch, pub UINode);

pub struct Context<'ui> {
    pub stretch: &'ui mut Stretch,
    pub canvas:  Canvas,
}

impl UINode {
    pub fn render_before<'ui>(&self, ctx: Context<'ui>) -> Context<'ui> {
        let parent_layout = self
            .get_parent()
            .map(|parent| parent.get_value().layout.unwrap());
        let parent_style = self.get_parent().map(|parent| parent.get_value().style);

        let mut element = self.get_value_mut();

        // println!("BEFORE");
        element.layout(ctx.stretch, parent_layout);
        element.style(parent_style);
        element.paint();
        // println!("{:#?}", element.layout);

        ctx
    }

    pub fn render_after<'ui>(&self, mut ctx: Context<'ui>) -> Context<'ui> {
        // println!("AFTER");
        Ref::map(self.get_value(), |element| {
            if let Some(paint) = &element.paint {
                paint.below(&mut ctx.canvas);
            }
            &()
        });

        ctx
    }
}

impl From<(Stretch, ElementObject)> for StretchUINode {
    fn from((mut stretch, element): (Stretch, ElementObject)) -> Self {
        let children = element.children();
        let style = element.style();
        let id = stretch.new_node(style.layout, vec![]).unwrap();
        let node = UINode::new(UIElement {
            id,
            element,
            style,
            layout: None,
            paint: None,
        });

        let len = children.len();
        let (mut stretch, children, children_ids) = children.into_iter().fold(
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
