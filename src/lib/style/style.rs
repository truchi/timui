use super::color_style::*;
use super::layout_style::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct Style {
    pub layout: LayoutStyle,
    pub color: ColorStyle,
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

    pub fn self_auto(self) -> Self {
        self.align_self(AlignSelf::Auto)
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

    pub fn justify_evenly(self) -> Self {
        self.justify_content(JustifyContent::SpaceEvenly)
    }

    // POSITION

    pub fn position(mut self, position: Rect<Dimension>) -> Self {
        self.layout.position = position;
        self
    }

    pub fn start(mut self, start: Dimension) -> Self {
        self.layout.position.start = start;
        self
    }

    pub fn end(mut self, end: Dimension) -> Self {
        self.layout.position.end = end;
        self
    }

    pub fn top(mut self, top: Dimension) -> Self {
        self.layout.position.top = top;
        self
    }

    pub fn bottom(mut self, bottom: Dimension) -> Self {
        self.layout.position.bottom = bottom;
        self
    }

    // MARGIN

    pub fn margin(mut self, margin: Rect<Dimension>) -> Self {
        self.layout.margin = margin;
        self
    }

    pub fn margin_start(mut self, start: Dimension) -> Self {
        self.layout.margin.start = start;
        self
    }

    pub fn margin_end(mut self, end: Dimension) -> Self {
        self.layout.margin.end = end;
        self
    }

    pub fn margin_top(mut self, top: Dimension) -> Self {
        self.layout.margin.top = top;
        self
    }

    pub fn margin_bottom(mut self, bottom: Dimension) -> Self {
        self.layout.margin.bottom = bottom;
        self
    }

    // PADDING

    pub fn padding(mut self, padding: Rect<Dimension>) -> Self {
        self.layout.margin = padding;
        self
    }

    pub fn padding_start(mut self, start: Dimension) -> Self {
        self.layout.padding.start = start;
        self
    }

    pub fn padding_end(mut self, end: Dimension) -> Self {
        self.layout.padding.end = end;
        self
    }

    pub fn padding_top(mut self, top: Dimension) -> Self {
        self.layout.padding.top = top;
        self
    }

    pub fn padding_bottom(mut self, bottom: Dimension) -> Self {
        self.layout.padding.bottom = bottom;
        self
    }

    // BORDER

    pub fn border(mut self, border: Rect<Dimension>) -> Self {
        self.layout.border = border;
        self
    }

    pub fn border_start(mut self, start: Dimension) -> Self {
        self.layout.border.start = start;
        self
    }

    pub fn border_end(mut self, end: Dimension) -> Self {
        self.layout.border.end = end;
        self
    }

    pub fn border_top(mut self, top: Dimension) -> Self {
        self.layout.border.top = top;
        self
    }

    pub fn border_bottom(mut self, bottom: Dimension) -> Self {
        self.layout.border.bottom = bottom;
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
        self.layout.flex_basis = basis;
        self
    }

    // SIZE

    pub fn size(mut self, size: Size<Dimension>) -> Self {
        self.layout.size = size;
        self
    }

    pub fn width(mut self, width: Dimension) -> Self {
        self.layout.size.width = width;
        self
    }

    pub fn height(mut self, height: Dimension) -> Self {
        self.layout.size.height = height;
        self
    }

    // MIN SIZE

    pub fn min_size(mut self, size: Size<Dimension>) -> Self {
        self.layout.min_size = size;
        self
    }

    pub fn min_width(mut self, width: Dimension) -> Self {
        self.layout.min_size.width = width;
        self
    }

    pub fn min_height(mut self, height: Dimension) -> Self {
        self.layout.min_size.height = height;
        self
    }

    // MAX SIZE

    pub fn max_size(mut self, size: Size<Dimension>) -> Self {
        self.layout.max_size = size;
        self
    }

    pub fn max_width(mut self, width: Dimension) -> Self {
        self.layout.max_size.width = width;
        self
    }

    pub fn max_height(mut self, height: Dimension) -> Self {
        self.layout.max_size.height = height;
        self
    }

    // ASPECT RATIO

    pub fn ratio(mut self, ratio: Number) -> Self {
        self.layout.aspect_ratio = ratio;
        self
    }

    // FORE/BACK-GROUND

    pub fn foreground(mut self, color: Color) -> Self {
        self.color.foreground = color;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.color.background = color;
        self
    }

    // BOLD

    pub fn bold(mut self) -> Self {
        self.color.bold = Some(true);
        self
    }

    pub fn no_bold(mut self) -> Self {
        self.color.bold = Some(false);
        self
    }

    pub fn inherit_bold(mut self) -> Self {
        self.color.bold = None;
        self
    }

    // ITALIC

    pub fn italic(mut self) -> Self {
        self.color.italic = Some(true);
        self
    }

    pub fn no_italic(mut self) -> Self {
        self.color.italic = Some(false);
        self
    }

    pub fn inherit_italic(mut self) -> Self {
        self.color.italic = None;
        self
    }

    // UNDERLINE

    pub fn underline(mut self) -> Self {
        self.color.underline = Some(true);
        self
    }

    pub fn no_underline(mut self) -> Self {
        self.color.underline = Some(false);
        self
    }

    pub fn inherit_underline(mut self) -> Self {
        self.color.underline = None;
        self
    }
}
