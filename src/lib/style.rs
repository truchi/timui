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

#[derive(Debug)]
pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    bold: bool,
    italic: bool,
    underline: bool,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.foreground = Some(c);
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.background = Some(c);
        self
    }

    pub fn bold(mut self, b: bool) -> Self {
        self.bold = b;
        self
    }

    pub fn italic(mut self, b: bool) -> Self {
        self.italic = b;
        self
    }

    pub fn underline(mut self, b: bool) -> Self {
        self.underline = b;
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground: None,
            background: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}
