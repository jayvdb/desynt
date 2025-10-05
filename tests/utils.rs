use desynt::utils;

#[test]
fn is_raw_ident() {
    assert!(utils::is_raw_ident("r#type"));
    assert!(!utils::is_raw_ident("type"));
    assert!(!utils::is_raw_ident("r"));
    assert!(!utils::is_raw_ident("#type"));
}

#[test]
fn strip_raw_prefix() {
    assert_eq!(utils::strip_raw_prefix("r#type"), "type");
    assert_eq!(utils::strip_raw_prefix("type"), "type");
    assert_eq!(utils::strip_raw_prefix("r#"), "");
}

#[test]
fn ident_from_string() {
    let raw_ident = utils::ident_from_string("r#type").unwrap();
    let normal_ident = utils::ident_from_string("normal").unwrap();

    assert_eq!(raw_ident.to_string(), "r#type");
    assert_eq!(normal_ident.to_string(), "normal");
}
