#![feature(map_get_key_value)]
pub mod edits;
#[cfg(test)]
mod test_edits;

extern crate itertools;
use edits::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
pub struct Dict(HashMap<String, usize>);

impl Dict {
    pub fn from_words<'a, T: IntoIterator<Item = &'a str>>(t: T) -> Self {
        let mut count: HashMap<String, usize> = HashMap::new();
        for k in t {
            *count.entry(k.to_string()).or_insert(0) += 1;
        }
        Dict(count)
    }

    ///the most likely correction for a word, if any
    pub fn correction(&self, word: &str) -> Option<String> {
        if self.0.contains_key(word) {
            Some(word.to_string())
        } else if let Some(dist_1_correction) = Edits::from(word) 
        // if there is a distance-1 correction, we don't bother checking distance-2 corrections
            .filter(|x| self.0.contains_key(x))
            .max_by_key(|x| self.0[x])
        {
            Some(dist_1_correction)
        } else {
            Dist2Edits::from(word)
                .filter(|x| self.0.contains_key(x))
                .max_by_key(|x| self.0[x])
        }
    }
}
