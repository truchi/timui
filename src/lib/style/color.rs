/// A `Color`
///
/// Convertible to terminal escape sequence thanks to [`termion`](https://docs.rs/termion/latest/termion/).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    /// Black,
    Black,
    /// Blue,
    Blue,
    /// Cyan,
    Cyan,
    /// Green,
    Green,
    /// Magenta,
    Magenta,
    /// Red,
    Red,
    /// White,
    White,
    /// Yellow,
    Yellow,
    /// Light black,
    LightBlack,
    /// Light blue,
    LightBlue,
    /// Light cyan,
    LightCyan,
    /// Light green,
    LightGreen,
    /// Light magenta,
    LightMagenta,
    /// Light red,
    LightRed,
    /// Light white,
    LightWhite,
    /// Light yellow,
    LightYellow,
}

/// Macro for `Color` methods
macro_rules! derive_colors {
    ($($color:ident,)*) => {
        impl Color {
            /// Returns the foreground escape sequence for this `Color`
            pub fn fg_str(&self) -> &'static str {
                match self {
                    $(Self::$color => termion::color::$color.fg_str(),)*
                }
            }

            /// Returns the background escape sequence for this `Color`
            pub fn bg_str(&self) -> &'static str {
                match self {
                    $(Self::$color => termion::color::$color.bg_str(),)*
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

// ========================================================================= //
//                                   TESTS                                   //
// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn colors() {
        assert_eq!(Color::Red.fg_str(), termion::color::Red.fg_str());
        assert_eq!(Color::Red.bg_str(), termion::color::Red.bg_str());

        assert_eq!(Color::Green.fg_str(), termion::color::Green.fg_str());
        assert_eq!(Color::Green.bg_str(), termion::color::Green.bg_str());

        assert_eq!(Color::Blue.fg_str(), termion::color::Blue.fg_str());
        assert_eq!(Color::Blue.bg_str(), termion::color::Blue.bg_str());
    }
}
