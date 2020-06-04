use super::super::component::Component;
use super::super::view::View;

impl Component for char {
    type Props = char;
    type Children = ();

    fn view(&self, props: char, _: ()) -> View {
        props.into()
    }
}
