#[derive(Clone, Copy, Debug)]
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
    Rgba,
    White,
    Yellow,
}

impl Color {
    pub fn default_foreground() -> Self {
        Self::White
    }

    pub fn default_background() -> Self {
        Self::Transparent
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Transparent
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ColorStyle {
    pub foreground: Color, // TODO inheritable
    pub background: Color,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub border_color: Color,
}

impl Default for ColorStyle {
    fn default() -> Self {
        Self {
            foreground: Color::default_foreground(),
            background: Color::default_background(),
            bold: Default::default(),
            italic: Default::default(),
            underline: Default::default(),
            border_color: Color::default_background(),
        }
    }
}
