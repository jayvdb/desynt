use desynt::{HasRaw, StripRaw};
use syn::{Path, parse_str};

#[test]
fn std_primitive_types() {
    let std_f64: Path = parse_str("::std::num::f64").unwrap();
    let stripped = std_f64.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["std", "num", "f64"]);
    assert!(!std_f64.has_raw());
}

#[test]
fn core_primitive_types() {
    let core_f64: Path = parse_str("::core::num::f64").unwrap();
    let stripped = core_f64.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["core", "num", "f64"]);
    assert!(!core_f64.has_raw());
}

#[test]
fn std_with_raw_in_middle() {
    let std_raw_middle: Path = parse_str("::std::r#num::f64").unwrap();
    let stripped = std_raw_middle.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["std", "num", "f64"]);
    assert!(std_raw_middle.has_raw());
}

#[test]
fn core_with_raw_in_middle() {
    let core_raw_middle: Path = parse_str("::core::r#num::f64").unwrap();
    let stripped = core_raw_middle.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["core", "num", "f64"]);
    assert!(core_raw_middle.has_raw());
}

#[test]
fn raw_at_beginning() {
    let raw_beginning: Path = parse_str("::r#std::num::f64").unwrap();
    let stripped = raw_beginning.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["std", "num", "f64"]);
    assert!(raw_beginning.has_raw());
}

#[test]
fn raw_at_end() {
    let raw_end: Path = parse_str("::std::num::r#f64").unwrap();
    let stripped = raw_end.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["std", "num", "f64"]);
    assert!(raw_end.has_raw());
}

#[test]
fn multiple_raw_identifiers() {
    let multiple_raw: Path = parse_str("::r#std::r#num::r#f64").unwrap();
    let stripped = multiple_raw.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["std", "num", "f64"]);
    assert!(multiple_raw.has_raw());
}

#[test]
fn complex_primitive_path_with_raw() {
    let complex_path: Path = parse_str("::std::r#primitive::r#types::i32").unwrap();
    let stripped = complex_path.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["std", "primitive", "types", "i32"]);
    assert!(complex_path.has_raw());
}

#[test]
fn various_primitive_types() {
    let primitives = vec![
        "::std::num::i8",
        "::std::num::i16",
        "::std::num::i32",
        "::std::num::i64",
        "::std::num::i128",
        "::std::num::u8",
        "::std::num::u16",
        "::std::num::u32",
        "::std::num::u64",
        "::std::num::u128",
        "::std::num::f32",
        "::std::num::f64",
        "::core::str",
        "::std::string::String",
    ];

    for primitive in primitives {
        let path: Path = parse_str(primitive).unwrap();
        let stripped = path.strip_raw();

        // Should not have raw identifiers in these basic cases
        assert!(!path.has_raw());

        // Stripping should not change anything for non-raw paths
        let original_segments: Vec<String> = path
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect();
        let stripped_segments: Vec<String> = stripped
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect();

        assert_eq!(original_segments, stripped_segments);
    }
}

#[test]
fn mixed_raw_and_normal_segments() {
    let mixed_path: Path = parse_str("normal::r#raw::normal::r#another").unwrap();
    let stripped = mixed_path.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["normal", "raw", "normal", "another"]);
    assert!(mixed_path.has_raw());
}
