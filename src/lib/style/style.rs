use stretch::geometry::Rect as StretchRect;
use stretch::geometry::Size as StretchSize;
use stretch::number::Number;
pub use stretch::style::AlignContent;
pub use stretch::style::AlignItems;
pub use stretch::style::AlignSelf;
use stretch::style::Dimension as StretchDimension;
pub use stretch::style::Direction;
pub use stretch::style::Display;
pub use stretch::style::FlexDirection;
pub use stretch::style::FlexWrap;
pub use stretch::style::JustifyContent;
pub use stretch::style::Overflow;
pub use stretch::style::PositionType;
use stretch::style::Style as StretchStyle;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension {
    Undefined,
    Auto,
    Points(usize),
    Percent(usize),
}

impl Default for Dimension {
    fn default() -> Self {
        Self::Undefined
    }
}

impl From<Dimension> for StretchDimension {
    fn from(dimension: Dimension) -> Self {
        match dimension {
            Dimension::Undefined => Self::Undefined,
            Dimension::Auto => Self::Auto,
            Dimension::Points(v) => Self::Points(v as f32),
            Dimension::Percent(v) => Self::Percent((v as f32) / 100.0),
        }
    }
}

pub trait Rect {
    fn rect(self) -> StretchRect<StretchDimension>;
}

impl Rect for () {
    fn rect(self) -> StretchRect<StretchDimension> {
        Default::default()
    }
}

impl Rect for Dimension {
    fn rect(self) -> StretchRect<StretchDimension> {
        StretchRect {
            start: self.into(),
            end: self.into(),
            top: self.into(),
            bottom: self.into(),
        }
    }
}

impl Rect for (Dimension, Dimension) {
    fn rect(self) -> StretchRect<StretchDimension> {
        StretchRect {
            start: self.0.into(),
            end: self.0.into(),
            top: self.1.into(),
            bottom: self.1.into(),
        }
    }
}

impl Rect for [Dimension; 2] {
    fn rect(self) -> StretchRect<StretchDimension> {
        StretchRect {
            start: self[0].into(),
            end: self[0].into(),
            top: self[1].into(),
            bottom: self[1].into(),
        }
    }
}

impl Rect for (Dimension, Dimension, Dimension, Dimension) {
    fn rect(self) -> StretchRect<StretchDimension> {
        StretchRect {
            start: self.0.into(),
            end: self.1.into(),
            top: self.2.into(),
            bottom: self.3.into(),
        }
    }
}

impl Rect for [Dimension; 4] {
    fn rect(self) -> StretchRect<StretchDimension> {
        StretchRect {
            start: self[0].into(),
            end: self[1].into(),
            top: self[2].into(),
            bottom: self[3].into(),
        }
    }
}

pub trait Size {
    fn size(self) -> StretchSize<StretchDimension>;
}

impl Size for () {
    fn size(self) -> StretchSize<StretchDimension> {
        Default::default()
    }
}

impl Size for Dimension {
    fn size(self) -> StretchSize<StretchDimension> {
        StretchSize {
            width: self.into(),
            height: self.into(),
        }
    }
}

impl Size for (Dimension, Dimension) {
    fn size(self) -> StretchSize<StretchDimension> {
        StretchSize {
            width: self.0.into(),
            height: self.1.into(),
        }
    }
}

impl Size for [Dimension; 2] {
    fn size(self) -> StretchSize<StretchDimension> {
        StretchSize {
            width: self[0].into(),
            height: self[1].into(),
        }
    }
}

pub trait Bounds {
    fn bounds(self) -> (Dimension, Dimension);
}

impl Bounds for () {
    fn bounds(self) -> (Dimension, Dimension) {
        Default::default()
    }
}

impl Bounds for Dimension {
    fn bounds(self) -> (Dimension, Dimension) {
        (self, self)
    }
}

impl Bounds for (Dimension, Dimension) {
    fn bounds(self) -> (Dimension, Dimension) {
        (self.0, self.1)
    }
}

impl Bounds for [Dimension; 2] {
    fn bounds(self) -> (Dimension, Dimension) {
        (self[0], self[1])
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InheritBoolean {
    Inherit,
    True,
    False,
}

impl Default for InheritBoolean {
    fn default() -> Self {
        Self::Inherit
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
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
    Transparent,
    Magenta,
    Red,
    Rgb,
    White,
    Yellow,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorStyle {
    foreground: Color,
    background: Color,
    bold: InheritBoolean,
    italic: InheritBoolean,
    underline: InheritBoolean,
}

impl Default for ColorStyle {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Transparent,
            bold: Default::default(),
            italic: Default::default(),
            underline: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Style {
    layout: StretchStyle,
    color: ColorStyle,
}

impl Style {
    pub fn new() -> Self {
        Default::default()
    }

    // DISPLAY

    pub fn display(mut self, display: Display) -> Self {
        self.layout.display = display;
        self
    }

    pub fn flex(self) -> Self {
        self.display(Display::Flex)
    }

    pub fn none(self) -> Self {
        self.display(Display::None)
    }

    // POSITION TYPE

    pub fn position_type(mut self, position: PositionType) -> Self {
        self.layout.position_type = position;
        self
    }

    pub fn absolute(self) -> Self {
        self.position_type(PositionType::Absolute)
    }

    pub fn relative(self) -> Self {
        self.position_type(PositionType::Relative)
    }

    // DIRECTION

    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout.direction = direction;
        self
    }

    pub fn inherit_direction(self) -> Self {
        self.direction(Direction::Inherit)
    }

    pub fn ltr(self) -> Self {
        self.direction(Direction::LTR)
    }

    pub fn rtl(self) -> Self {
        self.direction(Direction::RTL)
    }

    // FLEX DIRECTION

    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.layout.flex_direction = direction;
        self
    }

    pub fn row(self) -> Self {
        self.flex_direction(FlexDirection::Row)
    }

    pub fn column(self) -> Self {
        self.flex_direction(FlexDirection::Column)
    }

    pub fn row_reverse(self) -> Self {
        self.flex_direction(FlexDirection::RowReverse)
    }

    pub fn column_reverse(self) -> Self {
        self.flex_direction(FlexDirection::ColumnReverse)
    }

    // FLEX WRAP

    pub fn flex_wrap(mut self, wrap: FlexWrap) -> Self {
        self.layout.flex_wrap = wrap;
        self
    }

    pub fn no_wrap(self) -> Self {
        self.flex_wrap(FlexWrap::NoWrap)
    }

    pub fn wrap(self) -> Self {
        self.flex_wrap(FlexWrap::Wrap)
    }

    pub fn wrap_reverse(self) -> Self {
        self.flex_wrap(FlexWrap::WrapReverse)
    }

    // OVERFLOW

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.layout.overflow = overflow;
        self
    }

    pub fn visible(self) -> Self {
        self.overflow(Overflow::Visible)
    }

    pub fn hidden(self) -> Self {
        self.overflow(Overflow::Hidden)
    }

    pub fn scroll(self) -> Self {
        self.overflow(Overflow::Scroll)
    }

    // ALIGN ITEMS

    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.layout.align_items = align;
        self
    }

    pub fn items_start(self) -> Self {
        self.align_items(AlignItems::FlexStart)
    }

    pub fn items_end(self) -> Self {
        self.align_items(AlignItems::FlexEnd)
    }

    pub fn items_center(self) -> Self {
        self.align_items(AlignItems::Center)
    }

    pub fn items_baseline(self) -> Self {
        self.align_items(AlignItems::Baseline)
    }

    pub fn items_stretch(self) -> Self {
        self.align_items(AlignItems::Stretch)
    }

    // ALIGN SELF

    pub fn align_self(mut self, align: AlignSelf) -> Self {
        self.layout.align_self = align;
        self
    }

    pub fn self_start(self) -> Self {
        self.align_self(AlignSelf::FlexStart)
    }

    pub fn self_end(self) -> Self {
        self.align_self(AlignSelf::FlexEnd)
    }

    pub fn self_center(self) -> Self {
        self.align_self(AlignSelf::Center)
    }

    pub fn self_baseline(self) -> Self {
        self.align_self(AlignSelf::Baseline)
    }

    pub fn self_stretch(self) -> Self {
        self.align_self(AlignSelf::Stretch)
    }

    // ALIGN CONTENT

    pub fn align_content(mut self, align: AlignContent) -> Self {
        self.layout.align_content = align;
        self
    }

    pub fn content_start(self) -> Self {
        self.align_content(AlignContent::FlexStart)
    }

    pub fn content_end(self) -> Self {
        self.align_content(AlignContent::FlexEnd)
    }

    pub fn content_center(self) -> Self {
        self.align_content(AlignContent::Center)
    }

    pub fn content_stretch(self) -> Self {
        self.align_content(AlignContent::Stretch)
    }

    pub fn content_between(self) -> Self {
        self.align_content(AlignContent::SpaceBetween)
    }

    pub fn content_around(self) -> Self {
        self.align_content(AlignContent::SpaceAround)
    }

    // JUSTIFY CONTENT

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.layout.justify_content = justify;
        self
    }

    pub fn justify_start(self) -> Self {
        self.justify_content(JustifyContent::FlexStart)
    }

    pub fn justify_end(self) -> Self {
        self.justify_content(JustifyContent::FlexEnd)
    }

    pub fn justify_center(self) -> Self {
        self.justify_content(JustifyContent::Center)
    }

    pub fn justify_between(self) -> Self {
        self.justify_content(JustifyContent::SpaceBetween)
    }

    pub fn justify_around(self) -> Self {
        self.justify_content(JustifyContent::SpaceAround)
    }

    pub fn justify_envenly(self) -> Self {
        self.justify_content(JustifyContent::SpaceEvenly)
    }

    // POSITION

    pub fn position(mut self, position: impl Rect) -> Self {
        self.layout.position = position.rect();
        self
    }

    pub fn start(mut self, start: Dimension) -> Self {
        self.layout.position.start = start.into();
        self
    }

    pub fn end(mut self, end: Dimension) -> Self {
        self.layout.position.end = end.into();
        self
    }

    pub fn top(mut self, top: Dimension) -> Self {
        self.layout.position.top = top.into();
        self
    }

    pub fn bottom(mut self, bottom: Dimension) -> Self {
        self.layout.position.bottom = bottom.into();
        self
    }

    // MARGIN

    pub fn margin(mut self, margin: impl Rect) -> Self {
        self.layout.margin = margin.rect();
        self
    }

    pub fn margin_start(mut self, start: Dimension) -> Self {
        self.layout.margin.start = start.into();
        self
    }

    pub fn margin_end(mut self, end: Dimension) -> Self {
        self.layout.margin.end = end.into();
        self
    }

    pub fn margin_top(mut self, top: Dimension) -> Self {
        self.layout.margin.top = top.into();
        self
    }

    pub fn margin_bottom(mut self, bottom: Dimension) -> Self {
        self.layout.margin.bottom = bottom.into();
        self
    }

    // PADDING

    pub fn padding(mut self, padding: impl Rect) -> Self {
        self.layout.margin = padding.rect();
        self
    }

    pub fn padding_start(mut self, start: Dimension) -> Self {
        self.layout.padding.start = start.into();
        self
    }

    pub fn padding_end(mut self, end: Dimension) -> Self {
        self.layout.padding.end = end.into();
        self
    }

    pub fn padding_top(mut self, top: Dimension) -> Self {
        self.layout.padding.top = top.into();
        self
    }

    pub fn padding_bottom(mut self, bottom: Dimension) -> Self {
        self.layout.padding.bottom = bottom.into();
        self
    }

    // BORDER

    pub fn border(mut self, border: impl Rect) -> Self {
        self.layout.border = border.rect();
        self
    }

    pub fn border_start(mut self, start: Dimension) -> Self {
        self.layout.border.start = start.into();
        self
    }

    pub fn border_end(mut self, end: Dimension) -> Self {
        self.layout.border.end = end.into();
        self
    }

    pub fn border_top(mut self, top: Dimension) -> Self {
        self.layout.border.top = top.into();
        self
    }

    pub fn border_bottom(mut self, bottom: Dimension) -> Self {
        self.layout.border.bottom = bottom.into();
        self
    }

    // FLEX GROW/SHRINK/BASIS

    pub fn grow(mut self, grow: f32) -> Self {
        self.layout.flex_grow = grow;
        self
    }

    pub fn shrink(mut self, shrink: f32) -> Self {
        self.layout.flex_shrink = shrink;
        self
    }

    pub fn basis(mut self, basis: Dimension) -> Self {
        self.layout.flex_basis = basis.into();
        self
    }

    // MIN/MAX SIZE

    pub fn min(mut self, size: impl Size) -> Self {
        self.layout.min_size = size.size();
        self
    }

    pub fn max(mut self, size: impl Size) -> Self {
        self.layout.max_size = size.size();
        self
    }

    pub fn min_width(mut self, width: Dimension) -> Self {
        self.layout.min_size.width = width.into();
        self
    }

    pub fn min_height(mut self, height: Dimension) -> Self {
        self.layout.min_size.height = height.into();
        self
    }

    pub fn max_width(mut self, width: Dimension) -> Self {
        self.layout.max_size.width = width.into();
        self
    }

    pub fn max_height(mut self, height: Dimension) -> Self {
        self.layout.max_size.height = height.into();
        self
    }

    pub fn width(self, bounds: impl Bounds) -> Self {
        let (min, max) = bounds.bounds();
        self.min_width(min).max_width(max)
    }

    pub fn height(self, bounds: impl Bounds) -> Self {
        let (min, max) = bounds.bounds();
        self.min_height(min).max_height(max)
    }

    // ASPECT RATIO

    pub fn ratio(mut self, ratio: Number) -> Self {
        self.layout.aspect_ratio = ratio;
        self
    }

    // BOLD

    pub fn bold(mut self) -> Self {
        self.color.bold = InheritBoolean::True;
        self
    }

    pub fn no_bold(mut self) -> Self {
        self.color.bold = InheritBoolean::False;
        self
    }

    pub fn inherit_bold(mut self) -> Self {
        self.color.bold = InheritBoolean::Inherit;
        self
    }

    // ITALIC

    pub fn italic(mut self) -> Self {
        self.color.italic = InheritBoolean::True;
        self
    }

    pub fn no_italic(mut self) -> Self {
        self.color.italic = InheritBoolean::False;
        self
    }

    pub fn inherit_italic(mut self) -> Self {
        self.color.italic = InheritBoolean::Inherit;
        self
    }

    // UNDERLINE

    pub fn underline(mut self) -> Self {
        self.color.underline = InheritBoolean::True;
        self
    }

    pub fn no_underline(mut self) -> Self {
        self.color.underline = InheritBoolean::False;
        self
    }

    pub fn inherit_underline(mut self) -> Self {
        self.color.underline = InheritBoolean::Inherit;
        self
    }
}
