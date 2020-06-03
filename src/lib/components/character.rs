use super::super::component;

component!(
    pub Char(char, ()),
    fn ui |c, _| c.clone().into()
);
