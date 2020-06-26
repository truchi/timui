#[derive(Clone, Copy, PartialEq, Debug)]
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
    // Rgb { r: u8, g: u8, b: u8 },
    // Rgba { r: u8, g: u8, b: u8, a: u8 },
    White,
    Yellow,
}

impl Default for Color {
    fn default() -> Self {
        Self::Transparent
    }
}

macro_rules! derive_colors {
    ($($color:ident,)*) => {
        impl Color {
            pub fn fg_str(&self) -> &'static str {
                use termion::color;
                match self {
                    Self::Transparent => "",
                    $(Self::$color => color::$color.fg_str(),)*
                }
            }
            pub fn bg_str(&self) -> &'static str {
                use termion::color;
                match self {
                    Self::Transparent => "",
                    $(Self::$color => color::$color.bg_str(),)*
                }
            }
        }
    };
}

derive_colors!(
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
    White,
    Yellow,
);
