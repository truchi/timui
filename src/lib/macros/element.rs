#[macro_export]
macro_rules! element {
    ($Component:ty, $props:expr, $children:expr $(,)?) => {
        (Box::new($crate::element::Element::new(
            <$Component>::default(),
            $props,
            $children,
        )) as $crate::element::ElementObject)
    };
    // NOTE default children (and props?) rather than ()?
    ($Component:ty, $props:expr $(,)?) => {
        $crate::element!($Component, $props, ());
    };
}
