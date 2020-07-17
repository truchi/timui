//! # `Cell`

use crate::style::{Color, ColorStyle};
use std::fmt::{Display, Error, Formatter};

/// A terminal cell
#[derive(Copy, Clone, Default, Debug)]
pub struct Cell {
    /// The char
    pub c:          char,
    /// Foreground color
    pub foreground: Color,
    /// Background color
    pub background: Color,
    /// Wether char is bold
    pub bold:       bool,
    /// Wether char is italic
    pub italic:     bool,
    /// Wether char is underlined
    pub underline:  bool,
}

impl Cell {
    /// Creates an 'empty' (transparent foreground, no char) `Cell` with
    /// `background` color
    pub fn with_background(background: Color) -> Self {
        Self {
            foreground: Color::Transparent,
            background,
            bold: false,
            italic: false,
            underline: false,
            c: ' ',
        }
    }

    /// Merges `above` above this `Cell`
    pub fn above(&self, above: &Self) -> Self {
        let mut merged = *self;

        if above.background != Color::Transparent {
            merged.background = above.background;
        };

        if above.c != ' ' && above.foreground != Color::Transparent {
            merged.foreground = above.foreground;
            merged.bold = above.bold;
            merged.italic = above.italic;
            merged.underline = above.underline;
            merged.c = above.c;
        }

        merged
    }

    /// Merges `below` below this `Cell`
    pub fn below(&self, below: &Self) -> Self {
        below.above(self)
    }
}

impl From<(ColorStyle, char)> for Cell {
    fn from((style, c): (ColorStyle, char)) -> Self {
        Self {
            foreground: style.foreground,
            background: style.background,
            bold: style.bold,
            italic: style.italic,
            underline: style.underline,
            c,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use termion::style;

        let foreground = self.foreground.fg_str();
        let background = if self.background == Color::Transparent {
            Color::Black
        } else {
            self.background
        }
        .bg_str();
        let bold = if self.bold {
            <style::Bold as AsRef<str>>::as_ref(&style::Bold)
        } else {
            <style::NoBold as AsRef<str>>::as_ref(&style::NoBold)
        };
        let italic = if self.italic {
            <style::Italic as AsRef<str>>::as_ref(&style::Italic)
        } else {
            <style::NoItalic as AsRef<str>>::as_ref(&style::NoItalic)
        };
        let underline = if self.underline {
            <style::Underline as AsRef<str>>::as_ref(&style::Underline)
        } else {
            <style::NoUnderline as AsRef<str>>::as_ref(&style::NoUnderline)
        };
        let c = if self.foreground == Color::Transparent {
            ' '
        } else {
            self.c
        };

        write!(
            f,
            "{}{}{}{}{}{}",
            foreground, background, bold, italic, underline, c
        )
    }
}
