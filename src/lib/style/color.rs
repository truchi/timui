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

impl Default for Color {
    fn default() -> Self {
        Self::Transparent
    }
}
