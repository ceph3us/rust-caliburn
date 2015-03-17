// rust-caliburn - a strictly compliant RFC2812 parser written in Rust
// Copyright (c) 2015, Michael Holmes <holmesmich@gmail.com>. All right reserved.
// See included LICENSE.md in for licensing info.

#![unstable]

#![feature(plugin)]
#![feature(core)]
#![feature(collections)]
#![feature(test)]

#![plugin(peg_syntax_ext)]

extern crate test;

mod line_parser;
peg_file! rfc2812("rfc2812.rustpeg");

#[cfg(test)]
mod tests;
