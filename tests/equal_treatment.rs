use desynt::DynamicPathResolver;
use rstest::rstest;
use syn::Path;

#[rstest]
// Standard library type - should work
#[case::std_option_direct("std::option::Option", Some("StdOption"))]
#[case::option_generic("Option<T>", Some("StdOption"))] // Single segment generic
#[case::std_option_generic("std::option::Option<T>", Some("StdOption"))] // Full path with generic
// Custom type 1 - should work exactly the same
#[case::butane_autopk_direct("butane::AutoPk", Some("AutoPk"))]
#[case::autopk_generic("AutoPk<i64>", Some("AutoPk"))] // Single segment generic - this was the issue
#[case::butane_autopk_generic("butane::AutoPk<i64>", Some("AutoPk"))] // Full path with generic
// Custom type 2 - should also work the same
#[case::custom_type_direct("my::custom::Type", Some("CustomType"))]
#[case::type_generic("Type<String>", Some("CustomType"))] // Single segment generic
#[case::custom_type_generic("my::custom::Type<String>", Some("CustomType"))] // Full path with generic
fn equal_treatment_of_stdlib_and_custom_types(#[case] input: &str, #[case] expected: Option<&str>) {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add both stdlib and custom mappings
    resolver.add_mapping("std::option::Option", "StdOption");
    resolver.add_mapping("butane::AutoPk", "AutoPk");
    resolver.add_mapping("my::custom::Type", "CustomType");

    let path: Path = syn::parse_str(input).unwrap();
    let result = resolver.resolve(&path);
    assert_eq!(result, expected, "Failed for: {}", input);
    println!("✓ {} -> {:?}", input, result);
}
