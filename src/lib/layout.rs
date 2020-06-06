#[derive(Debug)]
pub enum Layout {
    Auto,
    Absolute { x: u16, y: u16, w: u16, h: u16 },
}

impl Default for Layout {
    fn default() -> Self {
        Self::Auto
    }
}
