pub mod edits;
#[cfg(test)]
mod test_edits;

extern crate itertools;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Dict<'a> {
    count: HashMap<&'a str, usize>,
    total: usize,
}

impl<'a> Dict<'a> {
    pub fn from_words<T: IntoIterator<Item = &'a str>>(t: T) -> Self {
        let mut count: HashMap<&str, usize> = HashMap::new();
        for k in t {
            *count.entry(k.clone()).or_insert(0) += 1;
        }
        let mut total = 0;
        for (_, c) in &count {
            total += *c
        }
        Dict { count, total }
    }

    pub fn prob(&self, word: &str) -> f64 {
        (*self.count.get(word).unwrap_or(&0) as f64) / self.total as f64
    }

    pub fn is_known(&self, word: &str) -> bool {
        self.count.contains_key(word)
    }
}
