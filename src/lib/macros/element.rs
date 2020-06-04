#[macro_export]
macro_rules! element {
    ($Component:ty, $props:expr, $children:expr $(,)?) => {
        $crate::element::Element::new(<$Component>::default(), $props, $children).into()
    };
    // NOTE default children (and props?) rather than ()?
    ($Component:ty, $props:expr $(,)?) => {
        $crate::element!($Component, $props, ());
    };
}
