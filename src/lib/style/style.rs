use super::{color::Color, color_style::ColorStyle, layout_style::*};

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
    (enum into $setter:ident: $enum:ty; $($fn:ident: $variant:ident,)*) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($enum), "`"),
            pub fn $setter(mut self, $setter: impl Into<$enum>) -> Self {
                self.layout.$setter = $setter.into();
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
            pub fn $name(mut self, $name: impl Into<Rect>) -> Self {
                self.layout.$name = $name.into().into();
                self
            }
        );
    };
    (impl rect $name:ident: $fn:ident, $place:ident) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($name), ".", stringify!($place), "`"),
            pub fn $fn(mut self, $place: impl Into<Dimension>) -> Self {
                self.layout.$name.$place = $place.into().into();
                self
            }
        );
    };
    (size $size:ident $width:ident $height:ident) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($size), "`"),
            pub fn $size(mut self, $size: impl Into<Size>) -> Self {
                self.layout.$size = $size.into().into();
                self
            }
        );

        style_methods!(impl doc concat!(" Sets `", stringify!($size), ".width`"),
            pub fn $width(mut self, $width: impl Into<Dimension>) -> Self {
                self.layout.$size.width = $width.into().into();
                self
            }
        );

        style_methods!(impl doc concat!(" Sets `", stringify!($size), ".height`"),
            pub fn $height(mut self, $height: impl Into<Dimension>) -> Self {
                self.layout.$size.height = $height.into().into();
                self
            }
        );
    };
    (option color $method:ident $inherit:ident) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($method), "`"),
            pub fn $method(mut self, color: Color) -> Self {
                self.color.$method = Some(color);
                self
            }
        );

        style_methods!(impl doc concat!(" Inherits `", stringify!($method), "`"),
            pub fn $inherit(mut self) -> Self {
                self.color.$method = None;
                self
            }
        );
    };
    (option bool $method:ident $no:ident $inherit:ident) => {
        style_methods!(impl doc concat!(" Sets `", stringify!($method), "`"),
            pub fn $method(mut self) -> Self {
                self.color.$method = Some(true);
                self
            }
        );

        style_methods!(impl doc concat!(" Unsets `", stringify!($method), "`"),
            pub fn $no(mut self) -> Self {
                self.color.$method = Some(false);
                self
            }
        );

        style_methods!(impl doc concat!(" Inherits `", stringify!($method), "`"),
            pub fn $inherit(mut self) -> Self {
                self.color.$method = None;
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

    style_methods!(enum into align_items: AlignItems;
        items_start: FlexStart,
        items_end: FlexEnd,
        items_center: Center,
        items_baseline: Baseline,
        items_stretch: Stretch,
    );

    style_methods!(enum into align_self: AlignSelf;
        self_auto: Auto,
        self_start: FlexStart,
        self_end: FlexEnd,
        self_center: Center,
        self_baseline: Baseline,
        self_stretch: Stretch,
    );

    style_methods!(enum into align_content: AlignContent;
        content_start: FlexStart,
        content_end: FlexEnd,
        content_center: Center,
        content_stretch: Stretch,
        content_around: SpaceAround,
        content_between: SpaceBetween,
    );

    style_methods!(enum into justify_content: JustifyContent;
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
        self.layout.flex_basis = basis.into();
        self
    }

    /// Sets `aspect_ratio`
    pub fn ratio(mut self, ratio: Number) -> Self {
        self.layout.aspect_ratio = ratio.into();
        self
    }
}

/// Convenient methods for building a `Style`'s colors
impl Style {
    style_methods!(option color foreground inherit_foreground);

    style_methods!(option color background inherit_background);

    style_methods!(option bool bold no_bold inherit_bold);

    style_methods!(option bool italic no_italic inherit_italic);

    style_methods!(option bool underline no_underline inherit_underline);
}
