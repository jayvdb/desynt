use desynt::utils;
use rstest::rstest;

#[rstest]
#[case::raw_type("r#type", true)]
#[case::normal_type("type", false)]
#[case::just_r("r", false)]
#[case::hash_type("#type", false)]
fn is_raw_ident(#[case] input: &str, #[case] expected: bool) {
    assert_eq!(utils::is_raw_ident(input), expected);
}

#[rstest]
#[case::raw_type_to_type("r#type", "type")]
#[case::normal_type_unchanged("type", "type")]
#[case::raw_empty("r#", "")]
fn strip_raw_prefix(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(utils::strip_raw_prefix(input), expected);
}

#[test]
fn ident_from_string() {
    let raw_ident = utils::ident_from_string("r#type").unwrap();
    let normal_ident = utils::ident_from_string("normal").unwrap();

    assert_eq!(raw_ident.to_string(), "r#type");
    assert_eq!(normal_ident.to_string(), "normal");
}
