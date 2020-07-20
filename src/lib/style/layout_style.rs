/// Re-export from [`stretch::style::AlignContent`](https://docs.rs/stretch/latest/stretch/style/enum.AlignContent.html)
pub use stretch::style::AlignContent;
/// Re-export from [`stretch::style::AlignItems`](https://docs.rs/stretch/latest/stretch/style/enum.AlignItems.html)
pub use stretch::style::AlignItems;
/// Re-export from [`stretch::style::AlignSelf`](https://docs.rs/stretch/latest/stretch/style/enum.AlignSelf.html)
pub use stretch::style::AlignSelf;
/// Re-export from [`stretch::style::Direction`](https://docs.rs/stretch/latest/stretch/style/enum.Direction.html)
pub use stretch::style::Direction::{self, *};
/// Re-export from [`stretch::style::Display`](https://docs.rs/stretch/latest/stretch/style/enum.Display.html)
pub use stretch::style::Display::{self, Flex};
/// Re-export from [`stretch::style::FlexDirection`](https://docs.rs/stretch/latest/stretch/style/enum.FlexDirection.html)
pub use stretch::style::FlexDirection::{self, *};
/// Re-export from [`stretch::style::FlexWrap`](https://docs.rs/stretch/latest/stretch/style/enum.FlexWrap.html)
pub use stretch::style::FlexWrap::{self, *};
/// Re-export from [`stretch::style::JustifyContent`](https://docs.rs/stretch/latest/stretch/style/enum.JustifyContent.html)
pub use stretch::style::JustifyContent;
/// Re-export from [`stretch::style::Overflow`](https://docs.rs/stretch/latest/stretch/style/enum.Overflow.html)
pub use stretch::style::Overflow::{self, *};
/// Re-export from [`stretch::style::PositionType`](https://docs.rs/stretch/latest/stretch/style/enum.PositionType.html)
pub use stretch::style::PositionType::{self, *};
/// Re-export from [`stretch::style::Style`](https://docs.rs/stretch/latest/stretch/style/struct.Style.html)
pub use stretch::style::Style as LayoutStyle;

mod s {
    //! # Encapsulated re-exports from [`stretch`](https://docs.rs/stretch/latest/stretch/index.html)

    /// Re-export from [`stretch::geometry::Rect`](https://docs.rs/stretch/latest/stretch/geometry/struct.Rect.html)
    pub use stretch::geometry::Rect;
    /// Re-export from [`stretch::geometry::Size`](https://docs.rs/stretch/latest/stretch/geometry/struct.Size.html)
    pub use stretch::geometry::Size;
    /// Re-export from [`stretch::number::Number`](https://docs.rs/stretch/latest/stretch/number/enum.Number.html)
    pub use stretch::number::Number;
    /// Re-export from [`stretch::style::Dimension`](https://docs.rs/stretch/latest/stretch/style/enum.Dimension.html)
    pub use stretch::style::Dimension;
}

/// For `Dimension::Auto`, `AlignSelf`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Auto;
/// For `AlignContent`, `AlignItems`, `AlignSelf`, `JustifyContent`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct FlexStart;
/// For `AlignContent`, `AlignItems`, `AlignSelf`, `JustifyContent`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct FlexEnd;
/// For `AlignContent`, `AlignItems`, `AlignSelf`, `JustifyContent`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Center;
/// For `AlignItems`, `AlignSelf`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Baseline;
/// For `AlignContent`, `AlignItems`, `AlignSelf`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Stretch;
/// For `AlignContent`, `JustifyContent`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct SpaceBetween;
/// For `AlignContent`, `JustifyContent`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct SpaceAround;
/// For `AlignItems`, `JustifyContent`
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct SpaceEvenly;

macro_rules! flex {
    (impl doc $doc:expr, $item:item) => { #[doc = $doc] $item };
    ($(enum $enum:ident: $($variant:ident,)*;)*) => {
        $($(flex!(impl doc concat!("`", stringify!($variant), "` is `", stringify!($enum), "::", stringify!($variant), "`"),
            impl From<$variant> for $enum {
                fn from(_: $variant) -> Self { Self::$variant }
            }
        );)*)*
    }
}

flex!(
    enum AlignContent: FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,;
    enum AlignItems: FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,;
    enum AlignSelf: Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,;
    enum JustifyContent: FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,;
);

// ======== //
//  Number  //
// ======== //

/// Mimics [`stretch::number::Number`](https://docs.rs/stretch/latest/stretch/number/enum.Number.html)
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Number {
    Defined(f32),
    Undefined,
}

/// Default is `Number::Undefined`
impl Default for Number {
    fn default() -> Self {
        Self::Undefined
    }
}

/// `()` is `Number::Undefined`
impl From<()> for Number {
    fn from(_: ()) -> Self {
        Self::Undefined
    }
}

/// `i32` is `Number::Defined(f32)`
impl From<i32> for Number {
    fn from(number: i32) -> Self {
        Self::Defined(number as f32)
    }
}

/// `f32` is `Number::Defined(f32)`
impl From<f32> for Number {
    fn from(number: f32) -> Self {
        Self::Defined(number)
    }
}

/// Convert back to `strecth`'s `Number`
impl From<Number> for s::Number {
    fn from(number: Number) -> Self {
        match number {
            Number::Defined(number) => Self::Defined(number),
            Number::Undefined => Self::Undefined,
        }
    }
}

// =========== //
//  Dimension  //
// =========== //

/// Mimics [`stretch::style::Dimension`](https://docs.rs/stretch/latest/stretch/style/enum.Dimension.html)
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension {
    Undefined,
    Auto,
    Points(i16),
    Percent(f32),
}

/// Default is `Dimension::Undefined`
impl Default for Dimension {
    fn default() -> Self {
        Self::Undefined
    }
}

/// `()` is `Dimension::Undefined`
impl From<()> for Dimension {
    fn from(_: ()) -> Self {
        Self::Undefined
    }
}

/// `Auto` is `Dimension::Auto`
impl From<Auto> for Dimension {
    fn from(_: Auto) -> Self {
        Self::Auto
    }
}

/// `i16` is `Dimension::Points(i16)`
impl From<i16> for Dimension {
    fn from(points: i16) -> Self {
        Self::Points(points)
    }
}

/// `f32` is `Dimension::Percent(f32)`
impl From<f32> for Dimension {
    fn from(percent: f32) -> Self {
        Self::Percent(percent)
    }
}

/// Convert back to `strecth`'s `Dimension`
impl From<Dimension> for s::Dimension {
    fn from(dimension: Dimension) -> Self {
        match dimension {
            Dimension::Undefined => Self::Undefined,
            Dimension::Auto => Self::Auto,
            Dimension::Points(points) => Self::Points(points as f32),
            Dimension::Percent(percent) => Self::Percent(percent),
        }
    }
}

// ====== //
//  Rect  //
// ====== //

/// Mimics [`stretch::geometry::Rect`](https://docs.rs/stretch/latest/stretch/geometry/struct.Rect.html)
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    start:  Dimension,
    end:    Dimension,
    top:    Dimension,
    bottom: Dimension,
}

/// `Dimension` is a `Rect`
impl<T> From<T> for Rect
where
    T: Into<Dimension> + Copy,
{
    fn from(dimension: T) -> Self {
        Self {
            start:  dimension.into(),
            end:    dimension.into(),
            top:    dimension.into(),
            bottom: dimension.into(),
        }
    }
}

/// `(Dimension, Dimension)` is a `Rect`
impl<T, U> From<(T, U)> for Rect
where
    T: Into<Dimension> + Copy,
    U: Into<Dimension> + Copy,
{
    fn from((t, u): (T, U)) -> Self {
        Self {
            start:  t.into(),
            end:    t.into(),
            top:    u.into(),
            bottom: u.into(),
        }
    }
}

/// `(Dimension, Dimension, Dimension, Dimension)` is a `Rect`
impl<S, E, T, B> From<(S, E, T, B)> for Rect
where
    S: Into<Dimension> + Copy,
    E: Into<Dimension> + Copy,
    T: Into<Dimension> + Copy,
    B: Into<Dimension> + Copy,
{
    fn from((s, e, t, b): (S, E, T, B)) -> Self {
        Self {
            start:  s.into(),
            end:    e.into(),
            top:    t.into(),
            bottom: b.into(),
        }
    }
}

/// Convert back to `strecth`'s `Rect<Dimension>`
impl From<Rect> for s::Rect<s::Dimension> {
    fn from(rect: Rect) -> Self {
        Self {
            start:  rect.start.into(),
            end:    rect.end.into(),
            top:    rect.top.into(),
            bottom: rect.bottom.into(),
        }
    }
}

// ====== //
//  Size  //
// ====== //

/// Mimics [`stretch::geometry::Size`](https://docs.rs/stretch/latest/stretch/geometry/struct.Size.html)
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Size {
    width:  Dimension,
    height: Dimension,
}

/// `Dimension` is a `Size`
impl<T> From<T> for Size
where
    T: Into<Dimension> + Copy,
{
    fn from(dimension: T) -> Self {
        Self {
            width:  dimension.into(),
            height: dimension.into(),
        }
    }
}

/// `(Dimension, Dimension)` is a `Size`
impl<T, U> From<(T, U)> for Size
where
    T: Into<Dimension> + Copy,
    U: Into<Dimension> + Copy,
{
    fn from((t, u): (T, U)) -> Self {
        Self {
            width:  t.into(),
            height: u.into(),
        }
    }
}

/// Convert back to `strecth`'s `Size<Dimension>`
impl From<Size> for s::Size<s::Dimension> {
    fn from(rect: Size) -> Self {
        Self {
            width:  rect.width.into(),
            height: rect.height.into(),
        }
    }
}

// ========================================================================= //
//                                   TESTS                                   //
// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn enums() {
        assert_eq!(AlignContent::from(FlexStart), AlignContent::FlexStart);
        assert_eq!(AlignContent::from(FlexEnd), AlignContent::FlexEnd);
        assert_eq!(AlignContent::from(Center), AlignContent::Center);
        assert_eq!(AlignContent::from(Stretch), AlignContent::Stretch);
        assert_eq!(AlignContent::from(SpaceBetween), AlignContent::SpaceBetween);
        assert_eq!(AlignContent::from(SpaceAround), AlignContent::SpaceAround);
        assert_eq!(AlignItems::from(FlexStart), AlignItems::FlexStart);
        assert_eq!(AlignItems::from(FlexEnd), AlignItems::FlexEnd);
        assert_eq!(AlignItems::from(Center), AlignItems::Center);
        assert_eq!(AlignItems::from(Baseline), AlignItems::Baseline);
        assert_eq!(AlignItems::from(Stretch), AlignItems::Stretch);
        assert_eq!(AlignSelf::from(Auto), AlignSelf::Auto);
        assert_eq!(AlignSelf::from(FlexStart), AlignSelf::FlexStart);
        assert_eq!(AlignSelf::from(FlexEnd), AlignSelf::FlexEnd);
        assert_eq!(AlignSelf::from(Center), AlignSelf::Center);
        assert_eq!(AlignSelf::from(Baseline), AlignSelf::Baseline);
        assert_eq!(JustifyContent::from(FlexStart), JustifyContent::FlexStart);
        assert_eq!(JustifyContent::from(FlexEnd), JustifyContent::FlexEnd);
        assert_eq!(JustifyContent::from(Center), JustifyContent::Center);
        assert_eq!(
            JustifyContent::from(SpaceBetween),
            JustifyContent::SpaceBetween
        );
        assert_eq!(
            JustifyContent::from(SpaceAround),
            JustifyContent::SpaceAround
        );
        assert_eq!(
            JustifyContent::from(SpaceEvenly),
            JustifyContent::SpaceEvenly
        );
    }

    #[test]
    fn number() {
        assert_eq!(Number::from(()), Number::Undefined);
        assert_eq!(Number::from(12), Number::Defined(12.));
        assert_eq!(Number::from(13.1), Number::Defined(13.1));
        assert_eq!(
            s::Number::from(Number::Defined(14.5)),
            s::Number::Defined(14.5)
        );
        assert_eq!(s::Number::from(Number::Undefined), s::Number::Undefined);
    }

    #[test]
    fn dimension() {
        assert_eq!(Dimension::from(()), Dimension::Undefined);
        assert_eq!(Dimension::from(Auto), Dimension::Auto);
        assert_eq!(Dimension::from(12), Dimension::Points(12));
        assert_eq!(Dimension::from(13.1), Dimension::Percent(13.1));
        assert_eq!(
            s::Dimension::from(Dimension::Undefined),
            s::Dimension::Undefined
        );
        assert_eq!(s::Dimension::from(Dimension::Auto), s::Dimension::Auto);
        assert_eq!(
            s::Dimension::from(Dimension::Points(13)),
            s::Dimension::Points(13.)
        );
        assert_eq!(
            s::Dimension::from(Dimension::Percent(0.2)),
            s::Dimension::Percent(0.2)
        );
    }

    #[test]
    fn rect() {
        assert_eq!(Rect::from(Dimension::Undefined), Rect {
            start:  Dimension::Undefined,
            end:    Dimension::Undefined,
            top:    Dimension::Undefined,
            bottom: Dimension::Undefined,
        });
        assert_eq!(
            Rect::from((Dimension::Points(11), Dimension::Percent(10.8))),
            Rect {
                start:  Dimension::Points(11),
                end:    Dimension::Points(11),
                top:    Dimension::Percent(10.8),
                bottom: Dimension::Percent(10.8),
            }
        );
        assert_eq!(
            Rect::from((
                Dimension::Percent(4.7),
                Dimension::Points(3),
                Dimension::Auto,
                Dimension::Undefined
            )),
            Rect {
                start:  Dimension::Percent(4.7),
                end:    Dimension::Points(3),
                top:    Dimension::Auto,
                bottom: Dimension::Undefined,
            }
        );
        assert_eq!(
            s::Rect::<s::Dimension>::from(Rect {
                start:  Dimension::Undefined,
                end:    Dimension::Auto,
                top:    Dimension::Percent(1.2),
                bottom: Dimension::Points(2),
            }),
            s::Rect::<s::Dimension> {
                start:  s::Dimension::Undefined,
                end:    s::Dimension::Auto,
                top:    s::Dimension::Percent(1.2),
                bottom: s::Dimension::Points(2.),
            }
        );
    }

    #[test]
    fn size() {
        assert_eq!(Size::from(Dimension::Undefined), Size {
            width:  Dimension::Undefined,
            height: Dimension::Undefined,
        });
        assert_eq!(
            Size::from((Dimension::Points(6), Dimension::Percent(3.3))),
            Size {
                width:  Dimension::Points(6),
                height: Dimension::Percent(3.3),
            }
        );
        assert_eq!(
            s::Size::<s::Dimension>::from(Size {
                width:  Dimension::Percent(3.9),
                height: Dimension::Points(4),
            }),
            s::Size::<s::Dimension> {
                width:  s::Dimension::Percent(3.9),
                height: s::Dimension::Points(4.),
            }
        );
    }
}
