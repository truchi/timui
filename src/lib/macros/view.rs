#[macro_export]
macro_rules! view {
    () => {
        $crate::view::View::default()
    };
    ($( ($Component:ty $(, $props:expr)? $(; $children:expr)?) )+) => {
        {
            let view: $crate::view::View = vec![
                $({
                    let view_element: $crate::view::ViewElement =
                        $crate::element!(box $Component $(, $props)? $(; $children)?)
                        .into();
                    view_element
                },)+
            ].into();
            view
        }
    };
    ($Component:ty $(, $props:expr)? $(; $children:expr)?) => {
        $crate::view!(($Component $(, $props)? $(; $children)?))
    };
}
