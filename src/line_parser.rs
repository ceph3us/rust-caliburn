// Types for storing parsed messages compliant with RFC2812
// Copyright (c) 2015, Michael Holmes <holmesmich@gmail.com>. All right reserved.
// See included LICENSE.md in for licensing info.

#[derive(Debug, PartialEq, Eq)]
pub enum Command<'a> {
    Verb(&'a str),
    Numeric(u16)
}

#[derive(Debug,  PartialEq, Eq)]
pub enum Prefix<'a> {
    Server(&'a str),
    User { nickname: &'a str, user: Option<&'a str>, host: Option<&'a str> },
    NoPrefix
}

#[derive(Debug,  PartialEq,  Eq)]
pub struct Message<'a> {
    pub prefix: Prefix<'a>,
    pub command: Command<'a>,
    pub params: Vec<String>
}
