use crate::style::Color;
use std::fmt::{Display, Error, Formatter};

#[derive(Copy, Clone, Default, Debug)]
pub struct Cell {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub c: char,
}

impl Cell {
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

    pub fn below(&self, below: &Self) -> Self {
        below.above(self)
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
