#[macro_export]
macro_rules! component {
    // =========================================================================
    // 1 TRAIT
    // =========================================================================

    // Implements layout for $Component with closure
    // Default style & view
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
        );
    };

    // Implements style for $Component with closure
    // Default layout & view
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn style |$style_props, $style_children| $style_body,
        );
    };

    // Implements view for $Component with closure
    // Default layout & style
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn view |$view_props, $view_children| $view_body,
        );
    };

    // =========================================================================
    // 2 TRAITS
    // =========================================================================

    // Implements style & view for $Component with closures
    // Default layout
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn style |$style_props, $style_children| $style_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn style |$style_props, $style_children| $style_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };

    // Implements layout & view for $Component with closures
    // Default style
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };

    // Implements layout & style for $Component with closures
    // Default view
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn style |$style_props, $style_children| $style_body,
        );
    };
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn style |$style_props, $style_children| $style_body,
        );
    };

    // =========================================================================
    // 3 TRAITS
    // =========================================================================

    // Implements layout, style & view for $Component with closures
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn style |$style_props, $style_children| $style_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn style |$style_props, $style_children| $style_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn style |$style_props, $style_children| $style_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr $(,)?
    ) => {
        $crate::component!(
            impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn style |$style_props, $style_children| $style_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr $(,)?
    ) => {
        $crate::component!(impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn style |$style_props, $style_children| $style_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };
    (
        $Component:ty, $Props:ty; $Children:ty,
        fn view |$view_props:tt, $view_children:tt| $view_body:expr,
        fn style |$style_props:tt, $style_children:tt| $style_body:expr,
        fn layout |$layout_props:tt, $layout_children:tt| $layout_body:expr $(,)?
    ) => {
        $crate::component!(impl $Component, $Props; $Children,
            fn layout |$layout_props, $layout_children| $layout_body,
            fn style |$style_props, $style_children| $style_body,
            fn view |$view_props, $view_children| $view_body,
        );
    };

    // =========================================================================
    // LOCAL
    // =========================================================================

    (
        impl $Component:ty, $Props:ty; $Children:ty,
        $(fn layout |$layout_props:tt, $layout_children:tt| $layout:expr,)?
        $(fn style |$style_props:tt, $style_children:tt| $style:expr,)?
        $(fn view |$view_props:tt, $view_children:tt| $view:expr,)?
    ) => {
        impl $crate::component::Component for $Component {
            type Props = $Props;
            type Children = $Children;

            $(fn layout(&self, props: &$Props, children: &$Children) -> $crate::layout::Layout {
                (|$layout_props: &$Props, $layout_children: &$Children| $layout)(props, children)
            })?

            $(fn style(&self, props: &$Props, children: &$Children) -> $crate::style::Style {
                (|$style_props: &$Props, $style_children: &$Children| $style)(props, children)
            })?

            $(fn view(&self, props: $Props, children: $Children) -> $crate::view::View {
                (|$view_props: $Props, $view_children: $Children| $view)(props, children)
            })?
        }
    };
}
