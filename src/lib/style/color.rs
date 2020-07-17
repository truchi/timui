//! # `Color`

/// `Color`
///
/// Represents a terminal color. Defaults to `Transparent`. Convertible to
/// terminal escape sequence thanks to `termion`. No alpha (for now).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    /// Transparent,
    Transparent,
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
    /* Rgb { r: u8, g: u8, b: u8 },
     * Rgba { r: u8, g: u8, b: u8, a: u8 }, */
}

impl Default for Color {
    /// `Color::Transparent`
    fn default() -> Self {
        Self::Transparent
    }
}

/// Macro for `Color` methods
macro_rules! derive_colors {
    ($($color:ident,)*) => {
        impl Color {
            /// Returns the foreground escape sequence for this `Color`
            pub fn fg(&self) -> &'static str {
                match self {
                    Self::Transparent => "",
                    $(Self::$color => termion::color::$color.fg_str(),)*
                }
            }

            /// Returns the background escape sequence for this `Color`
            pub fn bg(&self) -> &'static str {
                match self {
                    Self::Transparent => "",
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
    use super::Color;

    #[test]
    fn transparent_is_default() {
        assert_eq!(Color::default(), Color::Transparent);
    }

    #[test]
    fn transparent_has_no_escape_sequences() {
        assert_eq!(Color::Transparent.fg(), "");
        assert_eq!(Color::Transparent.bg(), "");
    }

    #[test]
    fn colors_have_correct_escape_sequences() {
        assert_eq!(Color::Red.fg(), termion::color::Red.fg_str());
        assert_eq!(Color::Red.bg(), termion::color::Red.bg_str());

        assert_eq!(Color::Green.fg(), termion::color::Green.fg_str());
        assert_eq!(Color::Green.bg(), termion::color::Green.bg_str());

        assert_eq!(Color::Blue.fg(), termion::color::Blue.fg_str());
        assert_eq!(Color::Blue.bg(), termion::color::Blue.bg_str());
    }
}
