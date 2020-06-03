use super::component::AnyComponent;
use super::component::Component;
use super::components::Char;

pub enum UI {
    None,
    Char(char),
    Component(Box<dyn AnyComponent>),
    Fragment(Vec<Box<dyn AnyComponent>>),
}

impl Default for UI {
    fn default() -> Self {
        UI::None
    }
}

impl<C> From<C> for UI
where
    C: Component + 'static, // NOTE ?
{
    fn from(component: C) -> Self {
        Self::Component(component.into())
    }
}

impl From<Vec<UI>> for UI {
    fn from(v: Vec<UI>) -> Self {
        Self::Fragment(
            v.into_iter()
                .flat_map(|ui| {
                    match ui {
                        UI::None => vec![],
                        UI::Char(c) => vec![Char {
                            props: c,
                            children: (),
                        }
                        .into()],
                        UI::Component(component) => vec![component],
                        UI::Fragment(vec) => vec,
                    }
                    .into_iter()
                })
                .collect(),
        )
    }
}

impl From<char> for UI {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl From<()> for UI {
    fn from(_: ()) -> Self {
        Self::None
    }
}

#[macro_export]
macro_rules! ui {
    ($props:expr $(,)?) => {
        $props
    };
    ($Component:ident, $props:expr, $children:expr $(,)?) => {
        $Component {
            props: $props,
            children: $children,
        }
        .into()
    };
    ($Component:ident, $props:expr $(,)?) => {
        $crate::ui!($Component, $props, ())
    };
}
