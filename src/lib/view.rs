use super::element::ElementObject;

pub type View = Vec<ElementObject>;

// pub type ViewElements = Vec<ViewElement>;
// pub type ElementObjects = Vec<ElementObject>;
//
// #[derive(Debug)]
// pub enum ViewElement {
// Char(char),
// String(String),
// Element(ElementObject),
// }
//
// impl From<char> for ViewElement {
// fn from(c: char) -> Self {
// Self::Char(c)
// }
// }
//
// impl From<String> for ViewElement {
// fn from(s: String) -> Self {
// Self::String(s)
// }
// }
//
// impl From<ElementObject> for ViewElement {
// fn from(e: ElementObject) -> Self {
// Self::Element(e)
// }
// }
//
// #[derive(Debug)]
// pub struct View(pub ViewElements);
//
// impl Default for View {
// fn default() -> Self {
// Self(vec![])
// }
// }
//
// impl From<char> for View {
// fn from(c: char) -> Self {
// Self(vec![c.into()])
// }
// }
//
// impl From<String> for View {
// fn from(s: String) -> Self {
// Self(vec![s.into()])
// }
// }
//
// impl From<ElementObject> for View {
// fn from(e: ElementObject) -> Self {
// Self(vec![e.into()])
// }
// }
//
// impl From<ElementObjects> for View {
// fn from(v: ElementObjects) -> Self {
// Self(v.into_iter().map(|e| e.into()).collect())
// }
// }
//
// impl From<ViewElement> for View {
// fn from(e: ViewElement) -> Self {
// Self(vec![e])
// }
// }
//
// impl From<ViewElements> for View {
// fn from(v: ViewElements) -> Self {
// Self(v)
// }
// }
//
// impl From<View> for ViewElements {
// fn from(v: View) -> Self {
// v.0
// }
// }
