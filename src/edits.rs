use std::collections::HashSet;
///Iterator over all the splits at a single position of a word
///i.e, for "foo", ("", "foo"), ("f", "oo"), ("fo", "o"), ("foo", "")
pub struct Splits {
    word: String,
    i: usize,
}
pub struct Transposes(Splits);
///Iterator over all the words formed from the delition of a single character from a word
/// i.e, "foo" -> "oo", "fo", "fo"
pub struct Deletes(Splits);

//Iterator over all the words formed from the replacement of a character in a word with a letter in a...z
pub struct Replaces {
    splits: Splits,
    current: Option<(String, String)>,
    offset: u8,
}

//Iterator over all the words formed from the insertion of a letter in a...z in any position in a word, including before the first character and after the last
pub struct Inserts {
    splits: Splits,
    current: Option<(String, String)>,
    offset: u8,
}

///Iterator over all the distance-1 edits of a word; deletion, insertion, or replacement of one character,
///or transposition of two characters
pub struct Edits {
    deletes: Deletes,
    transposes: Transposes,
    replaces: Replaces,
    inserts: Inserts,
}

pub struct Dist2Edits {
    current: Edits,
    dist1: Vec<String>,
    i: usize,
}

impl<'a, 'b> From<&'a str> for Dist2Edits {
    fn from(word: &'a str) -> Self {
        let unique_edits: HashSet<String> = Edits::from(word).collect();
        let dist1: Vec<String> = unique_edits.into_iter().collect();
        Dist2Edits {
            current: Edits::from(word),
            dist1,
            i: 0,
        }
    }
}

impl Iterator for Dist2Edits {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(edit) = self.current.next() {
            Some(edit)
        } else if self.i < self.dist1.len() {
            self.current = Edits::from((&self.dist1[self.i]).to_string());
            self.i += 1;
            self.next()
        } else {
            None
        }
    }
}

impl From<String> for Dist2Edits {
    fn from(s: String) -> Self {
        let unique_edits: HashSet<String> = Edits::from(s.clone()).collect();
        let dist1: Vec<String> = unique_edits.into_iter().collect();
        Dist2Edits {
            current: Edits::from(s),
            dist1,
            i: 0,
        }
    }
}
impl From<String> for Edits {
    fn from(s: String) -> Self {
        Edits {
            deletes: Deletes::from(s.clone()),
            transposes: Transposes::from(s.clone()),
            replaces: Replaces::from(s.clone()),
            inserts: Inserts::from(s.clone()),
        }
    }
}
impl<'a> From<&'a str> for Inserts {
    fn from(word: &str) -> Self {
        Inserts {
            splits: Splits::from(word),
            offset: 0,
            current: None,
        }
    }
}

impl From<String> for Splits {
    fn from(s: String) -> Self {
        Splits { word: s, i: 0 }
    }
}

impl From<String> for Transposes {
    fn from(s: String) -> Self {
        Transposes(Splits::from(s))
    }
}

impl From<String> for Deletes {
    fn from(s: String) -> Self {
        Deletes(Splits::from(s))
    }
}

impl From<String> for Replaces {
    fn from(s: String) -> Self {
        Replaces {
            splits: Splits::from(s),
            current: None,
            offset: 0,
        }
    }
}

impl From<String> for Inserts {
    fn from(s: String) -> Self {
        Inserts {
            splits: Splits::from(s),
            current: None,
            offset: 0,
        }
    }
}
impl<'a> From<&'a str> for Transposes {
    fn from(word: &'a str) -> Self {
        Transposes(Splits::from(word))
    }
}

impl<'a> From<&'a str> for Splits {
    fn from(word: &'a str) -> Self {
        return Splits {
            word: word.to_string(),
            i: 0,
        };
    }
}
impl<'a> From<&'a str> for Replaces {
    fn from(word: &'a str) -> Self {
        Replaces {
            splits: Splits::from(word),
            current: None,
            offset: 0,
        }
    }
}

impl<'a> From<&'a str> for Deletes {
    fn from(word: &'a str) -> Self {
        Deletes(Splits::from(word))
    }
}

impl Iterator for Edits {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(delete) = self.deletes.next() {
            Some(delete)
        } else if let Some(transpose) = self.transposes.next() {
            Some(transpose)
        } else if let Some(replace) = self.replaces.next() {
            Some(replace)
        } else if let Some(insert) = self.inserts.next() {
            Some(insert)
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for Edits {
    fn from(word: &'a str) -> Self {
        Edits {
            deletes: Deletes::from(word),
            transposes: Transposes::from(word),
            inserts: Inserts::from(word),
            replaces: Replaces::from(word),
        }
    }
}

#[inline]
fn nth_letter(n: u8) -> char {
    ('a' as u8 + n) as char
}
impl Iterator for Inserts {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let (Some((ref a, ref b)), n @ 0...25) = (&self.current, self.offset) {
            self.offset += 1;
            let mut insert = a.to_string();
            insert.push(nth_letter(n));
            return Some(insert + b);
        };
        match self.splits.next() {
            None => None,
            Some((a, b)) => {
                self.offset = 0;
                self.current = Some((a, b));
                self.next()
            }
        }
    }
}

impl Iterator for Replaces {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match (&self.current, self.offset) {
            (Some((ref a, ref b)), n @ 0...25) if a.len() > 0 => {
                let mut replace = a.to_string();
                replace.pop();
                replace.push(nth_letter(n));
                self.offset += 1;
                return Some(replace + &b);
            }
            _ => {}
        };

        match self.splits.next() {
            None => None,
            Some((a, b)) => {
                self.offset = 0;
                self.current = Some((a, b));
                self.next()
            }
        }
    }
}

impl Iterator for Deletes {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some((a, b)) => match b.len() {
                0 => self.next(),
                _ => Some(a + &b[1..]),
            },
        }
    }
}

impl Iterator for Transposes {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some((ref a, ref b)) if b.len() < 2 => self.next(), //we can only transpose the rightmost two characters if there are two or more
            Some((a, b)) => {
                let mut transpose = a.to_string();
                let chars: Vec<char> = b.chars().collect();
                transpose.push(chars[1]);
                transpose.push(chars[0]);
                transpose.extend(&chars[2..]);
                Some(transpose)
            }
        }
    }
}

impl Iterator for Splits {
    type Item = (String, String);
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.i;
        self.i += 1;
        if n > self.word.len() {
            None
        } else {
            Some((String::from(&self.word[..n]), String::from(&self.word[n..])))
        }
    }
}
