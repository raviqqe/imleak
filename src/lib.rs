#![feature(maybe_uninit_extra, maybe_uninit_ref, test)]

#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate test;

mod hash_map;
mod tagged_ref;
mod vec;

pub use hash_map::HashMap;
pub use vec::Vec;
