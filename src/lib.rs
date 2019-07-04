#![feature(test)]

#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate test;

mod bucket;
mod hamt;
mod map;
mod node;

pub use map::Map;
