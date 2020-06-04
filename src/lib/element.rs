use super::component::Component;
use super::layout::Layout;
use super::style::Style;
use super::view::View;
use std::fmt::Debug;

pub type ElementObject = Box<dyn ElementTrait>;

pub trait ElementTrait: Debug {
    fn layout(&self) -> Layout;

    fn style(&self) -> Style;

    fn view(self: Box<Self>) -> View;
}

#[derive(Debug)]
pub struct Element<C>
where
    C: Component,
{
    component: C,
    props: <C as Component>::Props,
    children: <C as Component>::Children,
}

impl<C> Element<C>
where
    C: Component,
{
    pub fn new(
        component: C,
        props: <C as Component>::Props,
        children: <C as Component>::Children,
    ) -> Self {
        Self {
            component,
            props,
            children,
        }
    }
}

impl<C> ElementTrait for Element<C>
where
    C: Component + Debug,
{
    fn layout(&self) -> Layout {
        self.component.layout(&self.props, &self.children)
    }

    fn style(&self) -> Style {
        self.component.style(&self.props, &self.children)
    }

    fn view(self: Box<Self>) -> View {
        self.component.view(self.props, self.children)
    }
}

impl<C> From<Element<C>> for ElementObject
where
    C: Component + Debug + 'static,
{
    fn from(element: Element<C>) -> Self {
        Box::new(element)
    }
}
