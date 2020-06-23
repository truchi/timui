use super::{Layer, Uniform};
use crate::style::Style;
use stretch::result::Layout;

#[derive(Default)]
pub struct Borders {
    border_start: Layer,
    border_end: Layer,
    border_top: Layer,
    border_bottom: Layer,
    border_start_top: Layer,
    border_start_bottom: Layer,
    border_end_top: Layer,
    border_end_bottom: Layer,
}

#[derive(Default)]
pub struct Paint {
    background: Layer,
    borders: Borders,
}

impl Paint {
    // pub fn new(layout: Layout, style: Style) -> Self {
    // println!("{:#?}", layout);
    // let background = Uniform::new();
    // Self {
    // background: Layer::Uniform(Uniform {
    // x: layout.location.x as usize,
    // y: layout.location.y as usize,
    // width: layout.size.width as usize,
    // height: layout.size.height as usize,
    // }),
    // ..Default::default()
    // }
    // }
}

// impl From
