use desynt::DynamicPathResolver;
use rstest::rstest;
use syn::Path;

#[test]
fn specific_requested_case() {
    let resolver = DynamicPathResolver::with_primitives();

    // with_primitives() already includes Option mapping

    // Test the exact case from the user request
    let path_str = "Option<butane::ForeignKey<Foo>>";
    let path: Path = syn::parse_str(path_str).unwrap();

    let result = resolver.resolve(&path);

    // This should now resolve to Some("Option") as requested
    assert_eq!(result, Some("Option"));
    println!("✓ {} resolves to {:?}", path_str, result);
}

#[rstest]
// Simple generic types (using built-in stdlib mappings)
#[case::option_string("Option<String>", "Option")]
#[case::vec_i32("Vec<i32>", "Vec")]
#[case::hashmap_string_i32("HashMap<String, i32>", "HashMap")]
// Nested generic types
#[case::option_vec_string("Option<Vec<String>>", "Option")]
#[case::vec_option_i32("Vec<Option<i32>>", "Vec")]
// Complex nested types like the user's case
#[case::option_butane_foreign_key("Option<butane::ForeignKey<User>>", "Option")]
#[case::vec_diesel_query_result("Vec<diesel::result::QueryResult<User>>", "Vec")]
// Fully qualified paths with generics
#[case::std_option_string("std::option::Option<String>", "Option")]
#[case::std_vec_user("std::vec::Vec<User>", "Vec")]
#[case::std_hashmap_string_value("std::collections::HashMap<String, Value>", "HashMap")]
fn more_realistic_cases(#[case] input: &str, #[case] expected: &str) {
    let resolver = DynamicPathResolver::with_primitives();

    let path: Path = syn::parse_str(input).unwrap();
    let result = resolver.resolve(&path);
    assert_eq!(result, Some(expected), "Failed for: {}", input);
    println!("✓ {} -> {}", input, expected);
}
