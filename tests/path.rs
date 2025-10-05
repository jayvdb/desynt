use desynt::{HasRaw, StripRaw};
use syn::{Path, parse_str};

#[test]
fn strip_raw() {
    let raw_path: Path = parse_str("std::r#type::r#match").unwrap();
    let stripped = raw_path.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["std", "type", "match"]);
}

#[test]
fn has_raw() {
    let raw_path: Path = parse_str("std::r#type::normal").unwrap();
    let normal_path: Path = parse_str("std::normal::other").unwrap();

    assert!(raw_path.has_raw());
    assert!(!normal_path.has_raw());
}
