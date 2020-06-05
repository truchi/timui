#[macro_export]
macro_rules! element {
    (box $Component:ty, $props:expr; $children:expr) => {
        {
            let element_object: $crate::element::ElementObject =
                $crate::element!($Component, $props; $children)
                .into();
            element_object
        }
    };
    ($Component:ty, $props:expr; $children:expr) => {
        $crate::element::Element::new(<$Component>::default(), $props, $children)
    };
    (box $Component:ty,    $props:expr     ) => { $crate::element!(box $Component, $props; <$Component as $crate::component::Component>::Children::default()) };
    (    $Component:ty,    $props:expr     ) => { $crate::element!(    $Component, $props; <$Component as $crate::component::Component>::Children::default()) };
    (box $Component:ty $(; $children:expr)?) => { $crate::element!(box $Component, <$Component as $crate::component::Component>::Props::default() $(; $children)?) };
    (    $Component:ty $(; $children:expr)?) => { $crate::element!(    $Component, <$Component as $crate::component::Component>::Props::default() $(; $children)?) };
}
