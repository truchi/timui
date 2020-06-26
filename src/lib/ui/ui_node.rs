use super::UIElement;
use crate::component::ElementObject;
use crate::paint::Canvas;
use crate::paint::Paint;
use crate::style::ColorStyleInherited;
use crate::utils::tree::Node;
use stretch::node::Stretch;

pub type UINode = Node<UIElement>;
pub struct StretchUINode(pub Stretch, pub UINode);

pub struct Context<'ui> {
    pub stretch: &'ui mut Stretch,
    pub inherited: ColorStyleInherited,
    pub canvas: Canvas,
}

impl UINode {
    pub fn before<'ui>(&self, mut ctx: Context<'ui>) -> Context<'ui> {
        println!("before");

        let mut element = self.get_value_mut();
        let layout = ctx.stretch.layout(element.id).unwrap();
        let inherited = element.style.color.inherit(ctx.inherited);
        element.paint = Paint::from((*layout, inherited));
        ctx.inherited = inherited;

        ctx
    }

    pub fn after<'ui>(&self, mut ctx: Context<'ui>) -> Context<'ui> {
        println!("after");
        self.get_value().paint.below(&mut ctx.canvas);

        ctx
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
