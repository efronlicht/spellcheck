#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;

const letters: &str = "abcdefghijklmnopqrstuvwxyz";
struct Dict<'a> {
    count: HashMap<&'a str, usize>,
    total: usize,
}

impl<'a> Dict<'a> {
    fn from_words<T: IntoIterator<Item = &'a str>>(t: T) -> Self {
        let mut count: HashMap<&str, usize> = HashMap::new();
        for k in t {
            *count.entry(k.clone()).or_insert(0) += 1;
        }
        let mut total = 0;
        for (k, c) in &count {
            total += *c
        }
        Dict { count, total }
    }

    fn prob(&self, word: &str) -> f64 {
        (*self.count.get(word).unwrap_or(&0) as f64) / self.total as f64
    }
}
fn splits(word: &String) -> Vec<(&str, &str)> {
    use std::iter;
    // there are len(word)-1 places we can split the word
    let mut splits: Vec<(&str, &str)> = Vec::with_capacity(s.len() - 1);
    for i in 1..word.len() {
        splits.push((&word[..i], &word[i..]));
    }
    splits
}

struct Splits<'a> {
    word: &'a String,
    i: usize,
}
struct Transposes<'a>(Splits<'a>);
struct Deletes<'a>(Splits<'a>);
struct Replaces<'a> {
    splits: Splits<'a>,
    current: Option<(&'a str, &'a str)>,
    offset: u8,
}
//Iterator over all the single-character alphabetical inserts a word;
//i.e, for for the string "foo" -> "afoo, bfoo... faoo, fboo... ....fooz"
struct Inserts<'a> {
    splits: Splits<'a>,
    offset: u8,
}

#[inline]
fn nth_letter(n: u8) -> char {
    ('a' as u8 + n) as char
}
impl<'a> Iterator for Inserts<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self.splits.next() {
            None => None,
            Some((L, R)) => {
                let mut insert = L.to_string();
                insert.push(nth_letter(self.offset));
                self.offset += 1;
                Some(&(insert + R))
            }
        }
    }
}
impl<'a> Iterator for Replaces<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let (Some((L, R)), n @ 0...24) = (self.current, self.offset) {
            self.offset += 1;
            let mut sub = L.to_string();
            sub.push(('a' as u8 + self.offset) as char);
            Some(&(sub + R));
        };
        match self.splits.next() {
            None => None,
            Some((L, R)) => {
                self.offset = 0;
                self.current = Some((L, R));
                let mut sub = L.to_string();
                sub.push(('a' as u8 + self.offset) as char);
                Some(&(sub + R))
            }
        }
    }
}
impl<'a> Iterator for Deletes<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some((L, R)) => match R.len() {
                0 => Some(L),
                _ => Some(&(L.to_string() + &R[1..])),
            },
        }
    }
}

impl<'a> From<&'a String> for Splits<'a> {
    fn from(word: &'a String) -> Self {
        Splits { word, i: 0 }
    }
}
impl<'a> Iterator for Splits<'a> {
    type Item = (&'a str, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.i;
        self.i += 1;
        if n > self.word.len() {
            None
        } else {
            Some((&self.word[..n], &self.word[self.n..]))
        }
    }
}
