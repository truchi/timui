use super::style::Style;
use super::ui::UI;

pub trait WithProps {
    type Props;

    fn get_props(&self) -> &Self::Props;
    fn set_props(&mut self, props: Self::Props);
}

pub trait WithChildren {
    type Children;

    fn get_children(&self) -> &Self::Children;
    fn set_children(&mut self, children: Self::Children);
}

pub trait Component: WithProps + WithChildren {
    fn ui(&self, _props: &Self::Props, _children: &Self::Children) -> UI {
        UI::default()
    }

    fn style(&self, _props: &Self::Props, _children: &Self::Children) -> Style {
        Style::default()
    }

    fn layout(&self, _props: &Self::Props, _children: &Self::Children) {}
}

pub trait AnyComponent {
    fn ui(&self) -> UI;
    fn style(&self) -> Style;
    fn layout(&self);
}

impl<C> From<C> for Box<dyn AnyComponent>
where
    C: Component + 'static, // NOTE 'static?
{
    fn from(component: C) -> Self {
        Box::new(component)
    }
}

impl<Props, ComponentObject: Component<Props = Props>> AnyComponent for ComponentObject {
    fn ui(&self) -> UI {
        self.ui(self.get_props(), self.get_children())
    }

    fn style(&self) -> Style {
        self.style(self.get_props(), self.get_children())
    }

    fn layout(&self) {
        self.layout(self.get_props(), self.get_children())
    }
}

#[macro_export]
macro_rules! props {
    ($Component:ty, $Props:ty) => {
        impl $crate::component::WithProps for $Component {
            type Props = $Props;

            fn get_props(&self) -> &Self::Props {
                &self.props
            }

            fn set_props(&mut self, props: Self::Props) {
                self.props = props
            }
        }
    };
}

#[macro_export]
macro_rules! children {
    ($Component:ty, $Children:ty) => {
        impl $crate::component::WithChildren for $Component {
            type Children = $Children;

            fn get_children(&self) -> &Self::Children {
                &self.children
            }

            fn set_children(&mut self, children: Self::Children) {
                self.children = children
            }
        }
    };
}

#[macro_export]
macro_rules! component {
    ($v:vis $Component:ident ($Props:ty, $Children:ty)) => {
        #[derive(Debug)]
        $v struct $Component {
            pub props: $Props,
            pub children: $Children,
        }
        $crate::props!($Component, $Props);
        $crate::children!($Component, $Children);
    };
    (
        $v:vis $Component:ident ($Props:ty, $Children:ty)
        $(, fn ui     |$ui_props:tt    , $ui_children:tt    | $ui_body:expr     )?
        $(, fn style  |$style_props:tt , $style_children:tt | $style_body:expr  )?
        $(, fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr )?
        $(,)?
    ) => {
        $crate::component!($v $Component ($Props, $Children));

        impl $crate::component::Component for $Component {
            $(fn ui(&self, $ui_props: &Self::Props, $ui_children: &Self::Children) -> $crate::ui::UI {
                $ui_body
            })?
            $(fn style(&self, $style_props: &Self::Props, $style_children: &Self::Children) -> $crate::style::Style {
                $style_body
            })?
            $(fn layout(&self, $layout_props: &Self::Props, $layout_children: &Self::Children)  {
                $layout_body
            })?
        }
    };
}
