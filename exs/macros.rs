use lib::{
    component,
    element,
    element::Element,
    layout::Layout,
    render::render,
    style::{Color, Style},
    view,
};
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
    fn layout |(w, h), _| Layout::Absolute { x: 0, y: 0, w: *w, h: *h },
    fn style |_, _| Style::new().background(Color::Green)
);

#[derive(Default, Debug)]
pub struct Word;
component!(
    Word, Vec<char>; Vec<Element<Footer>>,
    fn view |chars, footers|
        chars.iter().map(|c| element!(box char, *c))
            .chain(footers.into_iter().map(|footer| footer.over_props(|_| 'b').into()))
            .collect::<Vec<_>>().into(),
    fn layout |chars, _| Layout::Absolute{ x: 1, y: 1, w: chars.len() as u16, h: 2 },
    fn style |_, _| Style::new().background(Color::Red).foreground(Color::Green)
);

#[derive(Default, Debug)]
pub struct Footer;
component!(
    Footer, char; (),
    fn view |c, _| c.into(),
    fn layout |_, _| Layout::Absolute{ x: 0, y: 0, w: 1, h: 1 },
    fn style |_, _| Style::new().background(Color::Blue).foreground(Color::Green)
);

fn main() {
    render(element![box Root, terminal_size().unwrap()]);
    println!("{}", '🎁');
}
