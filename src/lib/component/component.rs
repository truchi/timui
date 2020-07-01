//! # `Component`

use super::Elements;
use crate::style::Style;
use std::fmt::Debug;

/// # `Component` trait
///
/// Trait to implement for your UI `Component`s
pub trait Component: Debug {
    /// Properties of the `Component`
    type Props: Debug;

    /// Returns the `Style` for this `Component`
    fn style(&self, _props: &Self::Props) -> Style {
        Default::default()
    }

    /// Returns the children `Elements` of this `Component`
    fn children(&self, _props: &Self::Props) -> Elements {
        Default::default()
    }
}
