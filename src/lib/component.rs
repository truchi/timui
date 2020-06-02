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
}

pub trait AnyComponent {
    fn ui(&self) -> UI;
    fn style(&self) -> UI;
}

impl<Props, ComponentObject: Component<Props = Props>> AnyComponent for ComponentObject {
    fn ui(&self) -> UI {
        self.ui(self.get_props(), self.get_children())
    }

    fn style(&self) -> UI {
        self.ui(self.get_props(), self.get_children())
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
