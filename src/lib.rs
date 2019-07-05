#![feature(test)]

#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate test;

mod hash_map;

pub use hash_map::HashMap;
