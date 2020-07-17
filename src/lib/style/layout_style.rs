//! # Re-exports from [`stretch`](https://docs.rs/stretch/latest/stretch/index.html)

/// Re-export from [`stretch::geometry::Rect`](https://docs.rs/stretch/latest/stretch/geometry/struct.Rect.html)
pub use stretch::geometry::Rect;
/// Re-export from [`stretch::geometry::Size`](https://docs.rs/stretch/latest/stretch/geometry/struct.Size.html)
pub use stretch::geometry::Size;
/// Re-export from
/// [`stretch::number::Number`](https://docs.rs/stretch/latest/stretch/number/enum.Number.html)
pub use stretch::number::Number::{self, Defined}; // Undefined clashes with
                                                  // Dimension::Undefined
/// Re-export from [`stretch::style::AlignContent`](https://docs.rs/stretch/latest/stretch/style/enum.AlignContent.html)
pub use stretch::style::AlignContent::{self, *};
/// Re-export from [`stretch::style::AlignItems`](https://docs.rs/stretch/latest/stretch/style/enum.AlignItems.html)
pub use stretch::style::AlignItems::{self, *};
/// Re-export from [`stretch::style::AlignSelf`](https://docs.rs/stretch/latest/stretch/style/enum.AlignSelf.html)
pub use stretch::style::AlignSelf::{self, *};
/// Re-export from [`stretch::style::Dimension`](https://docs.rs/stretch/latest/stretch/style/enum.Dimension.html)
pub use stretch::style::Dimension::{self, *};
/// Re-export from [`stretch::style::Direction`](https://docs.rs/stretch/latest/stretch/style/enum.Direction.html)
pub use stretch::style::Direction::{self, *};
/// Re-export from [`stretch::style::Display`](https://docs.rs/stretch/latest/stretch/style/enum.Display.html)
pub use stretch::style::Display::{self, Flex}; // None clashes with Option::None
/// Re-export from [`stretch::style::FlexDirection`](https://docs.rs/stretch/latest/stretch/style/enum.FlexDirection.html)
pub use stretch::style::FlexDirection::{self, *};
/// Re-export from [`stretch::style::FlexWrap`](https://docs.rs/stretch/latest/stretch/style/enum.FlexWrap.html)
pub use stretch::style::FlexWrap::{self, *};
/// Re-export from [`stretch::style::JustifyContent`](https://docs.rs/stretch/latest/stretch/style/enum.JustifyContent.html)
pub use stretch::style::JustifyContent::{self, *};
/// Re-export from [`stretch::style::Overflow`](https://docs.rs/stretch/latest/stretch/style/enum.Overflow.html)
pub use stretch::style::Overflow::{self, *};
/// Re-export from [`stretch::style::PositionType`](https://docs.rs/stretch/latest/stretch/style/enum.PositionType.html)
pub use stretch::style::PositionType::{self, *};
/// Re-export from [`stretch::style::Style`](https://docs.rs/stretch/latest/stretch/style/struct.Style.html)
pub use stretch::style::Style as LayoutStyle;
