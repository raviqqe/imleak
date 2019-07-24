#![feature(maybe_uninit_extra, test)]

#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate test;

mod hash_map;
mod tagged_ref;
mod vector;

pub use hash_map::HashMap;
pub use vector::Vector;
