#![feature(map_get_key_value)]
pub mod dict;
pub mod edits;

#[cfg(test)]
mod test_dict;
#[cfg(test)]
mod test_edits;

extern crate itertools;
pub use dict::Dict;
