use super::element::ElementObject;

#[derive(Debug)]
pub enum View {
    Empty,
    Char(char),
    List(Vec<ElementObject>),
}

impl Default for View {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<()> for View {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}

impl From<char> for View {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}
