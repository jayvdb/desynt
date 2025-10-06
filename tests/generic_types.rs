use desynt::DynamicPathResolver;
use rstest::rstest;
use syn::Path;

#[test]
fn generic_type_resolution() {
    let resolver = DynamicPathResolver::with_primitives();

    // Test if with_primitives() alone can handle Option<T> -> Option resolution

    // Test parsing a path with generic arguments
    let path_str = "Option<butane::ForeignKey<Foo>>";
    let path: Path = syn::parse_str(path_str).unwrap();

    println!("Original path: {}", path_str);
    println!(
        "Parsed path segments: {:?}",
        path.segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
    );

    // This should resolve to "Option" now
    let result = resolver.resolve(&path);
    println!("Resolver result: {:?}", result);

    // It should now resolve to Some("Option")
    assert_eq!(result, Some("Option"));
}

#[rstest]
#[case::vec_string("Vec<String>", Some("Vec"))]
#[case::hashmap_string_i32("HashMap<String, i32>", Some("HashMap"))]
#[case::nested_option_vec_hashmap("Option<Vec<HashMap<String, i32>>>", Some("Option"))]
#[case::fully_qualified_std_paths("std::option::Option<std::vec::Vec<String>>", Some("Option"))]
fn complex_generic_types(#[case] input: &str, #[case] expected: Option<&str>) {
    let resolver = DynamicPathResolver::with_primitives();

    let path: Path = syn::parse_str(input).unwrap();
    let result = resolver.resolve(&path);
    println!("Testing: {} -> {:?}", input, result);
    assert_eq!(result, expected, "Failed for case: {}", input);
}

#[rstest]
#[case::full_path_resolution("std::option::Option", Some("Option"))]
#[case::base_type_with_generics("std::option::Option<String>", Some("Option"))]
#[case::short_base_type("Result<String, Error>", Some("Result"))]
#[case::unknown_type("UnknownType<T>", None)]
fn generic_resolution_strategies(
    #[case] input: &str,
    #[case] expected: Option<&str>,
) {
    let resolver = DynamicPathResolver::with_primitives();

    let path: Path = syn::parse_str(input).unwrap();
    let result = resolver.resolve(&path);
    assert_eq!(result, expected, "Failed for: {}", input);
}
