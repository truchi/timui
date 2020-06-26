use super::UIElement;
use crate::component::ElementObject;
use crate::paint::Canvas;
use crate::paint::Paint;
use crate::style::ColorStyleInherited;
use crate::utils::tree::Node;
use std::cell::Ref;
use stretch::node::Stretch;

pub type UINode = Node<UIElement>;
pub struct StretchUINode(pub Stretch, pub UINode);

pub struct Context<'ui> {
    pub stretch: &'ui mut Stretch,
    pub canvas: Canvas,
}

impl UINode {
    pub fn before<'ui>(&self, ctx: Context<'ui>) -> Context<'ui> {
        let mut element = self.get_value_mut();
        let layout = *ctx.stretch.layout(element.id).unwrap();
        let paint = Paint::from((layout, element.color_style));
        element.layout = Some(layout);
        element.paint = Some(paint);

        ctx
    }

    pub fn after<'ui>(&self, mut ctx: Context<'ui>) -> Context<'ui> {
        Ref::map(self.get_value(), |element| {
            if let Some(paint) = &element.paint {
                paint.below(&mut ctx.canvas);
            }
            &()
        });

        ctx
    }
}

impl From<(Stretch, ColorStyleInherited, ElementObject)> for StretchUINode {
    fn from(
        (mut stretch, inherited, element): (Stretch, ColorStyleInherited, ElementObject),
    ) -> Self {
        let style = element.style();
        let elements = element.children();
        let layout_style = style.layout;
        let color_style = style.color.inherit(inherited);
        let id = stretch.new_node(layout_style, Default::default()).unwrap();
        let layout = None;
        let paint = None;
        let node = UINode::new(UIElement {
            id,
            element,
            layout,
            layout_style,
            color_style,
            paint,
        });

        let len = elements.len();
        let (mut stretch, children, children_ids) = elements.into_iter().fold(
            (stretch, Vec::with_capacity(len), Vec::with_capacity(len)),
            |(stretch, mut children, mut children_ids), element| {
                let Self(stretch, child) = (stretch, color_style, element).into();

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
