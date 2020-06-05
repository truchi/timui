use lib::element::Element;
use lib::layout::Layout;
use lib::render::render;
use lib::style::Color;
use lib::style::Style;
use lib::{component, element, view};
use termion::terminal_size;

#[derive(Default, Debug)]
pub struct Root;
component!(
    Root, (u16, u16); (),
    fn view |_, _| view![
        (Word, vec!['F', 'U', 'C', 'K'];
            vec![element!(Footer, 'a'; ())]
        )
        (char, 'C')
    ],
);

#[derive(Default, Debug)]
pub struct Word;
component!(
    Word, Vec<char>; Vec<Element<Footer>>,
    fn view |chars, footers|
        chars.iter().map(|c| element!(box char, *c))
            .chain(footers.into_iter().map(|footer| footer.over_props(|_| 'b').into()))
            .collect::<Vec<_>>().into(),
);

#[derive(Default, Debug)]
pub struct Footer;
component!(
    Footer, char; (),
    fn view |c, _| c.into(),
);

fn main() {
    render(element![box Root, terminal_size().unwrap()]);
    println!("{}", '🎁');
}
