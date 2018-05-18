use super::edits::*;
#[test]
fn test_splits() {
    let got: Vec<(String, String)> = Splits::from("foo").collect();
    let want: Vec<(String, String)> = vec![("", "foo"), ("f", "oo"), ("fo", "o"), ("foo", "")]
        .into_iter()
        .map(|(x, y)| (x.to_string(), y.to_string()))
        .collect();
    assert_eq!(want, got)
}

#[test]

fn test_replaces() {
    let got: Vec<String> = Replaces::from("ab").collect();
    let want: Vec<String> = vec![
        "ab", "bb", "cb", "db", "eb", "fb", "gb", "hb", "ib", "jb", "kb", "lb", "mb", "nb", "ob",
        "pb", "qb", "rb", "sb", "tb", "ub", "vb", "wb", "xb", "yb", "zb", "aa", "ab", "ac", "ad",
        "ae", "af", "ag", "ah", "ai", "aj", "ak", "al", "am", "an", "ao", "ap", "aq", "ar", "as",
        "at", "au", "av", "aw", "ax", "ay", "az",
    ].into_iter()
        .map(|x| x.to_string())
        .collect();
    assert_eq!(want, got)
}

fn test_inserts() {
    let got: Vec<String> = Inserts::from("a").collect();
    let want: Vec<String> = vec![
        "aa", "ba", "ca", "da", "ea", "fa", "ga", "ha", "ia", "ja", "ka", "la", "ma", "na", "oa",
        "pa", "qa", "ra", "sa", "ta", "ua", "va", "wa", "xa", "ya", "za", "aa", "ab", "ac", "ad",
        "ae", "af", "ag", "ah", "ai", "aj", "ak", "al", "am", "an", "ao", "ap", "aq", "ar", "as",
        "at", "au", "av", "aw", "ax", "ay", "az",
    ].into_iter()
        .map(|x| x.to_string())
        .collect();
    assert_eq!(want, got)
}
#[test]
fn test_deletes() {
    let got: Vec<String> = Deletes::from("foo").collect();
    let want: Vec<String> = vec!["oo", "fo", "fo"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    assert_eq!(want, got)
}

#[test]
fn test_transposes() {
    let got: Vec<String> = Transposes::from("bar").collect();
    let want: Vec<String> = vec!["abr".to_string(), "bra".to_string()];
    assert_eq!(want, got);
}

#[test]
fn test_edits() {
    let got = Edits::from("foo");
    let want = (Deletes::from("foo")
        .chain(Transposes::from("foo"))
        .chain(Replaces::from("foo"))
        .chain(Inserts::from("foo")));
    assert!(equal_items(got, want));
}

use std::collections::HashSet;
#[test]
fn test_dist2edits() {
    let dist2: HashSet<String> = Dist2Edits::from("foo").collect();
    assert!(dist2.contains("f"));
    assert!(dist2.contains("fooaa"));
    assert!(dist2.contains("of")); // delete & transpose
    assert!(dist2.contains("foof"))
}

fn equal_items<T, A, B>(a: A, b: B) -> bool
where
    T: Eq,
    A: IntoIterator<Item = T>,
    B: IntoIterator<Item = T>,
{
    let (mut a, mut b) = (a.into_iter(), b.into_iter());
    loop {
        match (a.next(), b.next()) {
            (Some(ref x), Some(ref y)) if x == y => continue,
            (None, None) => return true,
            _ => return false,
        }
    }
}
