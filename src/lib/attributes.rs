use super::style::Style;
use stretch::style::Style as StretchLayout;

pub type Layout = StretchLayout;

#[derive(Default, Debug)]
pub struct Attributes {
    pub layout: Layout,
    pub style: Style,
}
