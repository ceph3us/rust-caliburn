#![feature(plugin)]
#![feature(core)]
#![feature(collections)]
#![feature(test)]

#![plugin(peg_syntax_ext)]

extern crate test;

mod rfc2812_types;
peg_file! rfc2812("rfc2812.rustpeg");

#[cfg(test)]
mod tests;
