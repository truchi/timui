//! # Macro for `Style` methods

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
