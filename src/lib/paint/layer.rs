use crate::style::Color;

#[derive(Default)]
pub struct Cell<T> {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub c: T,
}

#[derive(Default)]
pub struct Surface {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub vec: Vec<Cell<char>>,
}

#[derive(Default)]
pub struct Uniform {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub cell: Cell<()>,
}

pub enum Layer {
    Uniform(Uniform),
    Surface(Surface),
}

impl Default for Layer {
    fn default() -> Self {
        Self::Uniform(Default::default())
    }
}

impl Layer {}
