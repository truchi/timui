use super::element::ElementObject;

#[derive(Debug)]
pub enum ViewElement {
    None,
    Char(char),
    String(String),
    Element(ElementObject),
}

impl Default for ViewElement {
    fn default() -> Self {
        Self::None
    }
}

impl From<char> for ViewElement {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl From<String> for ViewElement {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<ElementObject> for ViewElement {
    fn from(e: ElementObject) -> Self {
        Self::Element(e)
    }
}

#[derive(Debug)]
pub struct View(pub Vec<ViewElement>);

impl Default for View {
    fn default() -> Self {
        Self(vec![])
    }
}

impl From<char> for View {
    fn from(c: char) -> Self {
        Self(vec![c.into()])
    }
}

impl From<String> for View {
    fn from(s: String) -> Self {
        Self(vec![s.into()])
    }
}

impl From<ElementObject> for View {
    fn from(e: ElementObject) -> Self {
        Self(vec![e.into()])
    }
}

impl From<Vec<ElementObject>> for View {
    fn from(v: Vec<ElementObject>) -> Self {
        Self(v.into_iter().map(|e| e.into()).collect())
    }
}

impl From<ViewElement> for View {
    fn from(e: ViewElement) -> Self {
        Self(vec![e])
    }
}

impl From<Vec<ViewElement>> for View {
    fn from(v: Vec<ViewElement>) -> Self {
        Self(v)
    }
}

impl From<View> for Vec<ViewElement> {
    fn from(v: View) -> Self {
        v.0
    }
}
