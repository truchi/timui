#[derive(Debug)]
pub enum Color {
    AnsiValue,
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
    Magenta,
    Red,
    Rgb,
    White,
    Yellow,
}

#[derive(Default, Debug)]
pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
}
