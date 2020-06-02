use super::component::AnyComponent;

pub enum UI {
    None,
    Char(char),
    String(String),
    Component(Box<dyn AnyComponent>),
    Fragment(Vec<Box<dyn AnyComponent>>),
}

impl Default for UI {
    fn default() -> Self {
        UI::None
    }
}

impl From<Box<dyn AnyComponent>> for UI {
    fn from(c: Box<dyn AnyComponent>) -> Self {
        Self::Component(c)
    }
}

impl From<char> for UI {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl From<&char> for UI {
    fn from(c: &char) -> Self {
        Self::Char(c.clone())
    }
}

impl From<String> for UI {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<()> for UI {
    fn from(_: ()) -> Self {
        Self::None
    }
}
