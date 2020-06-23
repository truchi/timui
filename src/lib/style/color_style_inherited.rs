use super::Color;

#[derive(Clone, Copy, Debug)]
pub struct ColorStyleInherited {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for ColorStyleInherited {
    fn default() -> Self {
        Self {
            foreground: Color::Black,
            background: Color::White,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}
