//! # `Style`

use super::{color::*, color_style::*, layout_style::*};

/// # `Style`
#[derive(Copy, Clone, Default, Debug)]
pub struct Style {
    /// Layouting styles
    pub(crate) layout: LayoutStyle,
    /// Visual styles
    pub(crate) color:  ColorStyle,
}

/// `Style` constructor
impl Style {
    /// Returns `Default` `Style`
    pub fn new() -> Self {
        Default::default()
    }
}

/// Macro for `Style` methods
macro_rules! style_methods {
    (impl doc $doc:expr, $item:item) => {
        #[doc = $doc]
        $item
    };
    (enum $setter:ident: $enum:ty; $($fn:ident: $variant:ident,)*) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($enum), "`"),
            pub fn $setter(mut self, $setter: $enum) -> Self {
                self.layout.$setter = $setter;
                self
            }
        );

        $(style_methods!(impl doc concat!(" Sets `", stringify!($enum), "::", stringify!($variant), "`"),
            pub fn $fn(self) -> Self {
                self.$setter(<$enum>::$variant)
            }
        );)*
    };
    (rect $name:ident: $start:ident $end:ident $top:ident $bottom:ident) => {
        style_methods!(impl rect $name);
        style_methods!(impl rect $name: $start, start);
        style_methods!(impl rect $name: $end, end);
        style_methods!(impl rect $name: $top, top);
        style_methods!(impl rect $name: $bottom, bottom);
    };
    (impl rect $name:ident) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($name), "`"),
            pub fn $name(mut self, $name: Rect<Dimension>) -> Self {
                self.layout.$name = $name;
                self
            }
        );
    };
    (impl rect $name:ident: $fn:ident, $place:ident) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($name), ".", stringify!($place), "`"),
            pub fn $fn(mut self, $place: Dimension) -> Self {
                self.layout.$name.$place = $place;
                self
            }
        );
    };
    (size $size:ident $width:ident $height:ident) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($size), "`"),
            pub fn $size(mut self, $size: Size<Dimension>) -> Self {
                self.layout.$size = $size;
                self
            }
        );

        style_methods!(impl doc concat!(" Sets `", stringify!($size), ".width`"),
            pub fn $width(mut self, $width: Dimension) -> Self {
                self.layout.$size.width = $width;
                self
            }
        );

        style_methods!(impl doc concat!(" Sets `", stringify!($size), ".height`"),
            pub fn $height(mut self, $height: Dimension) -> Self {
                self.layout.$size.height = $height;
                self
            }
        );
    };
}

/// Convenient methods for building a `Style`'s display
impl Style {
    style_methods!(enum display: Display;
        flex: Flex,
        none: None,
    );

    style_methods!(enum position_type: PositionType;
        absolute: Absolute,
        relative: Relative,
    );

    style_methods!(enum direction: Direction;
        inherit_direction: Inherit,
        ltr: LTR,
        rtl: RTL,
    );

    style_methods!(enum flex_direction: FlexDirection;
        row: Row,
        row_reverse: RowReverse,
        column: Column,
        column_reverse: ColumnReverse,
    );

    style_methods!(enum flex_wrap: FlexWrap;
        no_wrap: NoWrap,
        wrap: Wrap,
        wrap_reverse: WrapReverse,
    );

    style_methods!(enum overflow: Overflow;
        visible: Visible,
        hidden: Hidden,
        scroll: Scroll,
    );

    style_methods!(enum align_items: AlignItems;
        items_start: FlexStart,
        items_end: FlexEnd,
        items_center: Center,
        items_baseline: Baseline,
        items_stretch: Stretch,
    );

    style_methods!(enum align_self: AlignSelf;
        self_auto: Auto,
        self_start: FlexStart,
        self_end: FlexEnd,
        self_center: Center,
        self_baseline: Baseline,
        self_stretch: Stretch,
    );

    style_methods!(enum align_content: AlignContent;
        content_start: FlexStart,
        content_end: FlexEnd,
        content_center: Center,
        content_stretch: Stretch,
        content_around: SpaceAround,
        content_between: SpaceBetween,
    );

    style_methods!(enum justify_content: JustifyContent;
        justify_start: FlexStart,
        justify_end: FlexEnd,
        justify_center: Center,
        justify_around: SpaceAround,
        justify_between: SpaceBetween,
        justify_evenly: SpaceEvenly,
    );

    style_methods!(rect position: start end top bottom);

    style_methods!(rect margin: margin_start margin_end margin_top margin_bottom);

    style_methods!(rect padding: padding_start padding_end padding_top padding_bottom);

    style_methods!(rect border: border_start border_end border_top border_bottom);

    style_methods!(size size width height);

    style_methods!(size min_size min_width min_height);

    style_methods!(size max_size max_width max_height);

    // FLEX GROW/SHRINK/BASIS

    /// Sets `flex_grow`
    pub fn grow(mut self, grow: f32) -> Self {
        self.layout.flex_grow = grow;
        self
    }

    /// Sets `flex_shrink`
    pub fn shrink(mut self, shrink: f32) -> Self {
        self.layout.flex_shrink = shrink;
        self
    }

    /// Sets `flex_basis`
    pub fn basis(mut self, basis: Dimension) -> Self {
        self.layout.flex_basis = basis;
        self
    }

    // ASPECT RATIO

    /// Sets `aspect_ratio`
    pub fn ratio(mut self, ratio: Number) -> Self {
        self.layout.aspect_ratio = ratio;
        self
    }
}

/// Convenient methods for building a `Style`'s colors
impl Style {
    // FORE/BACK-GROUND

    /// Sets `foreground`
    pub fn foreground(mut self, color: Color) -> Self {
        self.color.foreground = Some(color);
        self
    }

    /// Sets inherited `foreground`
    pub fn inherit_foreground(mut self) -> Self {
        self.color.foreground = None;
        self
    }

    /// Sets `background`
    pub fn background(mut self, color: Color) -> Self {
        self.color.background = color;
        self
    }

    // BOLD

    /// Sets `bold`
    pub fn bold(mut self) -> Self {
        self.color.bold = Some(true);
        self
    }

    /// Unsets `bold`
    pub fn no_bold(mut self) -> Self {
        self.color.bold = Some(false);
        self
    }

    /// Sets inherited `bold`
    pub fn inherit_bold(mut self) -> Self {
        self.color.bold = None;
        self
    }

    // ITALIC

    /// Sets `italic`
    pub fn italic(mut self) -> Self {
        self.color.italic = Some(true);
        self
    }

    /// Unsets `italic`
    pub fn no_italic(mut self) -> Self {
        self.color.italic = Some(false);
        self
    }

    /// Sets inherited `italic`
    pub fn inherit_italic(mut self) -> Self {
        self.color.italic = None;
        self
    }

    // UNDERLINE

    /// Sets `underline`
    pub fn underline(mut self) -> Self {
        self.color.underline = true;
        self
    }

    /// Unsets `underline`
    pub fn no_underline(mut self) -> Self {
        self.color.underline = false;
        self
    }
}
