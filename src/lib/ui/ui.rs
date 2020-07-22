use super::UIElement;
use crate::{component::ElementObject, paint::Canvas, utils::tree::Node};
use std::{
    cell::Ref,
    fmt::{Debug, Formatter, Result},
};
use stretch::node::Stretch;

type UINode = Node<UIElement>;
struct Context<'ui> {
    stretch: &'ui mut Stretch,
    canvas:  Canvas,
}

pub struct UI {
    root:    UINode,
    stretch: Stretch,
}

impl UI {
    pub fn new(element: ElementObject) -> Self {
        (element, Stretch::new()).into()
    }

    pub fn render(&mut self, width: u16, height: u16) {
        self.compute_layout(width, height);

        let ctx = Context {
            stretch: &mut self.stretch,
            canvas:  Canvas::with_background((0, 0, width, height), None),
        };

        let ctx = self
            .root
            .recurs(ctx, Self::render_before, Self::render_after);

        println!("{}{}", termion::clear::All, ctx.canvas);
    }

    fn compute_layout(&mut self, width: u16, height: u16) {
        self.stretch
            .compute_layout(self.root.get_value().id, stretch::geometry::Size {
                width:  stretch::number::Number::Defined(width as f32),
                height: stretch::number::Number::Defined(height as f32),
            })
            .unwrap();
    }

    fn render_before<'ui>(node: &UINode, ctx: Context<'ui>) -> Context<'ui> {
        let parent_layout = node
            .get_parent()
            .map(|parent| parent.get_value().layout.unwrap());
        let parent_style = node.get_parent().map(|parent| parent.get_value().style);
        let mut element = node.get_value_mut();

        // println!("BEFORE");
        element.layout(ctx.stretch, parent_layout);
        element.style(parent_style);
        element.paint();
        // println!("{:#?}", element.layout);

        ctx
    }

    fn render_after<'ui>(node: &UINode, mut ctx: Context<'ui>) -> Context<'ui> {
        // println!("AFTER");
        Ref::map(node.get_value(), |element| {
            if let Some(paint) = &element.paint {
                paint.below(&mut ctx.canvas);
            }
            &()
        });

        ctx
    }
}

impl From<(ElementObject, Stretch)> for UI {
    fn from((element, mut stretch): (ElementObject, Stretch)) -> Self {
        let children = element.children();
        let style = element.style();
        let id = stretch.new_node(style.layout, vec![]).unwrap();
        let node = UINode::new(UIElement::new(id, element, style));

        let len = children.len();
        let (mut stretch, children, children_ids) = children.into_iter().fold(
            (stretch, Vec::with_capacity(len), Vec::with_capacity(len)), // TODO only one alloc?
            |(stretch, mut children, mut children_ids), element| {
                let Self {
                    root: child,
                    stretch,
                } = (element, stretch).into();

                child.set_parent(&node);
                children_ids.push(child.get_value().id);
                children.push(child);

                (stretch, children, children_ids)
            },
        );

        node.set_children(children);
        stretch.set_children(id, children_ids).unwrap();

        Self {
            root: node,
            stretch,
        }
    }
}

impl Debug for UI {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <UINode as Debug>::fmt(&self.root, f)
    }
}
