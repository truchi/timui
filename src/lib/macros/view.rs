#[macro_export]
macro_rules! view {
    () => { $crate::view::view::default() };
    ($( ($Component:ty, $props:expr $(, $children:expr)? $(,)?) $(,)? )+) => {
        $crate::view::View::List(
            vec![
                $($crate::element!($Component, $props $(, $children)?),)+
            ]
        )
    };
    ($Component:ty, $props:expr $(, $children:expr)? $(,)?) => {
        $crate::view!(($Component, $props $(, $children)?))
    };
}
