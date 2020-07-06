//! # `Style` module
//!
//! `Component` styling.
//!
//! TODO document `Style` API usage

#[macro_use]
mod style_macro;

mod color;
mod color_style;
mod color_style_inherited;
mod layout_style;
mod style;

pub use color::*;
pub use color_style::*;
pub use color_style_inherited::*;
pub use layout_style::*;
pub use style::*;
