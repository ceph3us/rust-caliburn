#[derive(Debug, PartialEq, Eq)]
pub enum Command<'a> {
    Verb(&'a str),
    Numeric(u16)
}

#[derive(Debug,  PartialEq, Eq)]
pub enum Prefix<'a> {
    Server(&'a str),
    User { nickname: &'a str, user: Option<&'a str>, host: Option<&'a str> }
}

#[derive(Debug,  PartialEq,  Eq)]
pub struct Message<'a> {
    pub prefix: Option<Prefix<'a>>,
    pub command: Command<'a>,
    pub params: Option<Vec<&'a str>>
}
