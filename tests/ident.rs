#![cfg(test)]

use desynt::{HasRaw, StripRaw};
use syn::{Ident, parse_str};

#[test]
fn strip_raw() {
    let raw_ident: Ident = parse_str("r#type").unwrap();
    let stripped = raw_ident.strip_raw();
    assert_eq!(stripped.to_string(), "type");
    assert!(!stripped.to_string().starts_with("r#"));
}

#[test]
fn strip_raw_no_prefix() {
    let normal_ident: Ident = parse_str("normal").unwrap();
    let stripped = normal_ident.strip_raw();
    assert_eq!(stripped.to_string(), "normal");
}

#[test]
fn has_raw() {
    let raw_ident: Ident = parse_str("r#type").unwrap();
    let normal_ident: Ident = parse_str("normal").unwrap();

    assert!(raw_ident.has_raw());
    assert!(!normal_ident.has_raw());
}
