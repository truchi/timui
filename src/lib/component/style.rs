#[derive(Debug)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Transparent,
    Magenta,
    Red,
    Rgb,
    White,
    Yellow,
}

#[derive(Debug)]
pub struct Style {
    foreground: Color,
    background: Color,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Transparent,
            bold: Default::default(),
            italic: Default::default(),
            underline: Default::default(),
        }
    }
}
