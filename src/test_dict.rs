use dict::*;

#[test]
fn test_dict() {
    let custom_dict: Dict = "the quick brown fox jumped over the lazy dog"
        .split_whitespace()
        .collect();
    assert_eq!(custom_dict.correction("brown"), Some("brown".to_string())); // distance 0
    assert_eq!(custom_dict.correction("fxo"), Some("fox".to_string())); // distance 1
    assert_eq!(custom_dict.correction("thnn"), Some("the".to_string())); // distance 2
    assert_eq!(custom_dict.correction("amklsdmalskdnasklfn"), None); // distance... a lot
}
