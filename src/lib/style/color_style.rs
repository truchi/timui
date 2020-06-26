use super::{Color, ColorStyleInherited};

#[derive(Clone, Copy, Debug, Default)]
pub struct ColorStyle {
    pub foreground: Option<Color>,
    pub background: Color,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: bool,
}

impl ColorStyle {
    pub fn inherit(&self, inherited: ColorStyleInherited) -> ColorStyleInherited {
        ColorStyleInherited {
            foreground: self.foreground.unwrap_or(inherited.foreground),
            background: self.background,
            bold: self.bold.unwrap_or(inherited.bold),
            italic: self.italic.unwrap_or(inherited.italic),
            underline: self.underline,
        }
    }
}
