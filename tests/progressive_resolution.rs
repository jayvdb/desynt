use desynt::DynamicPathResolver;
use rstest::rstest;
use syn::Path;

#[rstest]
// Case 1: Direct mapping should work
#[case::std_option_direct("std::option::Option", Some("Option"))]
#[case::std_vec_direct("std::vec::Vec", Some("Vec"))]
// Case 2: Generic types should resolve to base type
#[case::std_option_with_string("std::option::Option<String>", Some("Option"))]
#[case::std_vec_with_i32("std::vec::Vec<i32>", Some("Vec"))]
#[case::std_hashmap_with_types("std::collections::HashMap<String, i32>", Some("HashMap"))]
// Case 3: Progressive path shortening for single base types
#[case::option_butane_foreign_key("Option<butane::ForeignKey<Foo>>", Some("Option"))]
#[case::vec_string("Vec<String>", Some("Vec"))]
#[case::hashmap_generic("HashMap<K, V>", Some("HashMap"))]
// Case 4: Progressive path shortening for qualified paths
#[case::option_qualified("option::Option<T>", Some("Option"))]
#[case::vec_qualified("vec::Vec<T>", Some("Vec"))]
#[case::collections_hashmap("collections::HashMap<K, V>", Some("HashMap"))]
// Case 5: Nested generics should resolve to outermost type
#[case::nested_option_vec_hashmap("Option<Vec<HashMap<String, i32>>>", Some("Option"))]
#[case::nested_vec_option("Vec<Option<String>>", Some("Vec"))]
// Case 6: Should NOT resolve arbitrary types with same suffix
#[case::unknown_type_none("unknown::Type", None)] // Should NOT resolve to "CustomType"
#[case::other_option_none("other::Option", None)] // Should NOT resolve because not std library
// Case 7: Custom types should work normally
#[case::custom_type_exact("my::custom::Type", Some("CustomType"))]
#[case::custom_type_generic("my::custom::Type<T>", Some("CustomType"))]
fn progressive_path_resolution(#[case] input: &str, #[case] expected: Option<&str>) {
    let mut resolver = DynamicPathResolver::with_primitives();

    // with_primitives() already includes stdlib mappings
    // Add only custom mappings for testing
    resolver.add_mapping("my::custom::Type", "CustomType");

    let path: Path = syn::parse_str(input).unwrap();
    let result = resolver.resolve(&path);
    assert_eq!(result, expected, "Failed for: {}", input);
    println!("✓ {} -> {:?}", input, result);
}

#[rstest]
// These should work because they follow std library patterns
#[case::option_generic_std("Option<T>", Some("StdOption"))] // Should find std::option::Option
#[case::option_qualified_std("option::Option<T>", Some("StdOption"))] // Should find std::option::Option
// These should NOT create false positives
#[case::unknown_option_none("unknown::Option<T>", None)] // Should NOT resolve to MyOption
#[case::random_type_none("random::Type<T>", None)] // Should NOT resolve to AnotherType or SpecialType
#[case::other_option_direct_none("other::Option", None)] // Should NOT resolve to any Option mapping
// Exact matches should still work
#[case::my_special_option_exact("my::special::Option", Some("MyOption"))]
#[case::my_special_type_exact("my::special::Type", Some("SpecialType"))]
#[case::another_type_exact("another::Type", Some("AnotherType"))]
// Generics of exact matches should work
#[case::my_special_option_generic("my::special::Option<T>", Some("MyOption"))]
#[case::my_special_type_generic("my::special::Type<T>", Some("SpecialType"))]
#[case::another_type_generic("another::Type<T>", Some("AnotherType"))]
fn conservative_suffix_matching(#[case] input: &str, #[case] expected: Option<&str>) {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add mappings that could create false positives
    resolver.add_mapping("std::option::Option", "StdOption");
    resolver.add_mapping("my::special::Option", "MyOption");
    resolver.add_mapping("another::Type", "AnotherType");
    resolver.add_mapping("my::special::Type", "SpecialType");

    let path: Path = syn::parse_str(input).unwrap();
    let result = resolver.resolve(&path);
    assert_eq!(result, expected, "Failed for: {}", input);
    println!("✓ {} -> {:?}", input, result);
}
