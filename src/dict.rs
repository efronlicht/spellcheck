use edits::*;
use std::collections::HashMap;
use std::iter::FromIterator;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dict(HashMap<String, usize>);
impl Dict {
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

impl<'a> FromIterator<&'a str> for Dict {
    fn from_iter<I: IntoIterator<Item = &'a str>>(it: I) -> Self {
        let mut count: HashMap<String, usize> = HashMap::new();
        for k in it {
            *count.entry(k.to_string()).or_insert(0) += 1;
        }
        Dict(count)
    }
}
impl FromIterator<String> for Dict {
    fn from_iter<I: IntoIterator<Item = String>>(it: I) -> Self {
        let mut count: HashMap<String, usize> = HashMap::new();
        for k in it {
            *count.entry(k).or_insert(0) += 1;
        }
        Dict(count)
    }
}
