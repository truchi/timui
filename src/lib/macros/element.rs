#[macro_export]
macro_rules! element {
    ($Component:ty, $props:expr, $children:expr $(,)?) => {
        $crate::element::Element::new(<$Component>::default(), $props, $children).into()
    };
    ($Component:ty, $props:expr $(,)?) => {
        $crate::element!(
            $Component,
            $props,
            <$Component as $crate::component::Component>::Children::default()
        );
    };
    ($Component:ty $(,)?) => {
        $crate::element!(
            $Component,
            <$Component as $crate::component::Component>::Props::default()
        );
    };
}
