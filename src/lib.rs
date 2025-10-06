//! A library for stripping raw prefixes from syn objects.
//!
//! This library provides utilities to read syn objects like `Ident` and `Path`
//! and strip raw identifier prefixes (the `r#` prefix used for raw identifiers in Rust).
//!
//! # Examples
//!
//! ```
//! use syn::Ident;
//! use desynt::StripRaw;
//!
//! let ident: Ident = syn::parse_str("r#type").unwrap();
//! let stripped = ident.strip_raw();
//! assert_eq!(stripped.to_string(), "type");
//! ```

use std::collections::HashMap;
use syn::{Ident, Path, PathArguments, PathSegment};

#[cfg(feature = "static-resolver")]
use phf::Map;

/// A trait for mapping storage that can be used with PathResolver.
pub trait MappingStorage {
    /// Get a canonical type name for a given normalized path.
    fn get(&self, path: &str) -> Option<&str>;

    /// Check if the storage contains a mapping for the given path.
    fn contains_key(&self, path: &str) -> bool;

    /// Get the number of mappings in the storage.
    fn len(&self) -> usize;

    /// Check if the storage is empty.
    fn is_empty(&self) -> bool;

    /// Get all path patterns.
    fn keys(&self) -> Box<dyn Iterator<Item = &str> + '_>;

    /// Get all canonical type names.
    fn values(&self) -> Box<dyn Iterator<Item = &str> + '_>;
}

/// Implementation of MappingStorage for HashMap (dynamic mappings)
impl MappingStorage for HashMap<String, String> {
    fn get(&self, path: &str) -> Option<&str> {
        self.get(path).map(|s| s.as_str())
    }

    fn contains_key(&self, path: &str) -> bool {
        HashMap::contains_key(self, path)
    }

    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(HashMap::keys(self).map(|s| s.as_str()))
    }

    fn values(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(HashMap::values(self).map(|s| s.as_str()))
    }
}

/// Implementation of MappingStorage for PHF Map (static mappings)
#[cfg(feature = "static-resolver")]
impl MappingStorage for Map<&'static str, &'static str> {
    fn get(&self, path: &str) -> Option<&str> {
        Map::get(self, path).copied()
    }

    fn contains_key(&self, path: &str) -> bool {
        Map::contains_key(self, path)
    }

    fn len(&self) -> usize {
        Map::len(self)
    }

    fn is_empty(&self) -> bool {
        Map::is_empty(self)
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(Map::keys(self).copied())
    }

    fn values(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(Map::values(self).copied())
    }
}

/// Implementation of MappingStorage for &PHF Map (static mappings)
#[cfg(feature = "static-resolver")]
impl MappingStorage for &Map<&'static str, &'static str> {
    fn get(&self, path: &str) -> Option<&str> {
        Map::get(*self, path).copied()
    }

    fn contains_key(&self, path: &str) -> bool {
        Map::contains_key(*self, path)
    }

    fn len(&self) -> usize {
        Map::len(*self)
    }

    fn is_empty(&self) -> bool {
        Map::is_empty(*self)
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(Map::keys(*self).copied())
    }

    fn values(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(Map::values(*self).copied())
    }
}

/// Empty storage for when no custom mappings are needed
#[derive(Debug, Clone, Copy)]
pub struct EmptyStorage;

impl MappingStorage for EmptyStorage {
    fn get(&self, _path: &str) -> Option<&str> {
        None
    }

    fn contains_key(&self, _path: &str) -> bool {
        false
    }

    fn len(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(std::iter::empty())
    }

    fn values(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(std::iter::empty())
    }
}

/// A trait for stripping raw prefixes from syn objects.
pub trait StripRaw {
    /// The type returned after stripping raw prefixes.
    type Output;

    /// Strip raw prefixes and return the cleaned object.
    fn strip_raw(&self) -> Self::Output;
}

/// A trait for checking if a syn object has raw prefixes.
pub trait HasRaw {
    /// Returns true if the object contains raw identifiers.
    fn has_raw(&self) -> bool;
}

impl StripRaw for Ident {
    type Output = Ident;

    fn strip_raw(&self) -> Self::Output {
        let ident_str = self.to_string();
        if let Some(stripped) = ident_str.strip_prefix("r#") {
            // Create a new Ident without the raw prefix
            Ident::new(stripped, self.span())
        } else {
            self.clone()
        }
    }
}

impl HasRaw for Ident {
    fn has_raw(&self) -> bool {
        self.to_string().starts_with("r#")
    }
}

impl StripRaw for PathSegment {
    type Output = PathSegment;

    fn strip_raw(&self) -> Self::Output {
        PathSegment {
            ident: self.ident.strip_raw(),
            arguments: self.arguments.clone(),
        }
    }
}

impl HasRaw for PathSegment {
    fn has_raw(&self) -> bool {
        self.ident.has_raw()
    }
}

impl StripRaw for Path {
    type Output = Path;

    fn strip_raw(&self) -> Self::Output {
        Path {
            leading_colon: self.leading_colon,
            segments: self.segments.iter().map(|seg| seg.strip_raw()).collect(),
        }
    }
}

impl HasRaw for Path {
    fn has_raw(&self) -> bool {
        self.segments.iter().any(|seg| seg.has_raw())
    }
}

/// Utility functions for working with raw identifiers.
pub mod utils {
    use syn::Ident;

    /// Check if a string represents a raw identifier.
    pub fn is_raw_ident(s: &str) -> bool {
        s.starts_with("r#")
    }

    /// Strip the raw prefix from a string if present.
    pub fn strip_raw_prefix(s: &str) -> &str {
        if is_raw_ident(s) { &s[2..] } else { s }
    }

    /// Create a new Ident from a string, automatically handling raw prefixes.
    /// If the input has a raw prefix, it creates a raw identifier.
    /// Otherwise, it creates a regular identifier.
    pub fn ident_from_string(s: &str) -> syn::Result<Ident> {
        if is_raw_ident(s) {
            syn::parse_str(&format!("r#{}", &s[2..]))
        } else {
            syn::parse_str(s)
        }
    }
}

/// A path resolver that maps various path representations to canonical types.
///
/// This allows users to define a set of canonical type paths and resolve
/// syn Path objects to those canonical forms, regardless of how they were
/// referenced in the original code (with or without raw identifiers, different
/// module prefixes, etc.).
///
/// The resolver supports three resolution strategies:
/// 1. **Exact path matching** - Direct lookup of the full path
/// 2. **Generic type resolution** - Extracts base type from generics (e.g., `Option<T>` → `Option`)
/// 3. **Progressive path resolution** - Tries shorter path variations for standard library types
///
/// ## Progressive Path Resolution
///
/// This feature automatically handles common path variations without requiring explicit mappings
/// for every variant. For example, if you map `std::option::Option` → `Option`, the resolver
/// will automatically handle:
/// - `Option<T>` → `Option` (single segment with generics)
/// - `option::Option<T>` → `Option` (shortened qualified path)
/// - `std::option::Option<T>` → `Option` (full qualified path with generics)
///
/// The progressive resolution is conservative and only applies to:
/// - Common standard library types (`Option`, `Vec`, `HashMap`, etc.)
/// - Paths that contain standard library module names (`std`, `core`, `option`, `vec`, etc.)
///
/// This prevents false matches like resolving `unknown::Option` to `Option` when you only
/// mapped `std::option::Option`.
///
/// The resolver can be created with different storage backends:
/// - `HashMap<String, String>` for dynamic runtime mappings
/// - `phf::Map<&'static str, &'static str>` for static compile-time mappings
/// - `EmptyStorage` for const resolvers with only primitive mappings
///
/// # Examples
///
/// ## Dynamic Usage with HashMap
/// ```
/// use desynt::PathResolver;
/// use std::collections::HashMap;
/// use syn::Path;
///
/// let mut mappings = HashMap::new();
/// mappings.insert("std::primitive::f64".to_string(), "f64".to_string());
/// mappings.insert("core::primitive::f64".to_string(), "f64".to_string());
///
/// let resolver = PathResolver::new(mappings, true);
///
/// let path: Path = syn::parse_str("::std::primitive::f64").unwrap();
/// if let Some(canonical) = resolver.resolve(&path) {
///     assert_eq!(canonical, "f64");
/// }
/// ```
///
/// ## Static Usage with PHF Map
/// ```
/// use desynt::{PathResolver, EmptyStorage};
/// use phf::{phf_map, Map};
///
/// // Define mappings in tests/examples where phf_map is allowed
/// // static CUSTOM_MAPPINGS: Map<&'static str, &'static str> = phf_map! {
/// //     "actix_web::HttpResponse" => "HttpResponse",
/// //     "serde_json::Value" => "JsonValue",
/// // };
///
/// // const RESOLVER: PathResolver<&'static Map<&'static str, &'static str>> =
/// //     PathResolver::new(&CUSTOM_MAPPINGS, true);
///
/// // Or with only primitives
/// const PRIMITIVE_RESOLVER: PathResolver<EmptyStorage> =
///     PathResolver::new(EmptyStorage, true);
/// ```
/// Const primitive type mappings - implementation created in get_primitive_mapping_static
#[cfg(feature = "static-resolver")]
fn get_primitive_mapping_static(path: &str) -> Option<&'static str> {
    match path {
        // Primitive integer types
        "std::primitive::i8" | "core::primitive::i8" | "std::i8" | "core::i8" => Some("i8"),
        "std::primitive::i16" | "core::primitive::i16" | "std::i16" | "core::i16" => Some("i16"),
        "std::primitive::i32" | "core::primitive::i32" | "std::i32" | "core::i32" => Some("i32"),
        "std::primitive::i64" | "core::primitive::i64" | "std::i64" | "core::i64" => Some("i64"),
        "std::primitive::i128" | "core::primitive::i128" | "std::i128" | "core::i128" => {
            Some("i128")
        }
        "std::primitive::isize" | "core::primitive::isize" | "std::isize" | "core::isize" => {
            Some("isize")
        }

        // Primitive unsigned integer types
        "std::primitive::u8" | "core::primitive::u8" | "std::u8" | "core::u8" => Some("u8"),
        "std::primitive::u16" | "core::primitive::u16" | "std::u16" | "core::u16" => Some("u16"),
        "std::primitive::u32" | "core::primitive::u32" | "std::u32" | "core::u32" => Some("u32"),
        "std::primitive::u64" | "core::primitive::u64" | "std::u64" | "core::u64" => Some("u64"),
        "std::primitive::u128" | "core::primitive::u128" | "std::u128" | "core::u128" => {
            Some("u128")
        }
        "std::primitive::usize" | "core::primitive::usize" | "std::usize" | "core::usize" => {
            Some("usize")
        }

        // Primitive floating point types
        "std::primitive::f32" | "core::primitive::f32" | "std::f32" | "core::f32" => Some("f32"),
        "std::primitive::f64" | "core::primitive::f64" | "std::f64" | "core::f64" => Some("f64"),

        // Other primitive types
        "std::primitive::bool" | "core::primitive::bool" | "std::bool" | "core::bool" => {
            Some("bool")
        }
        "std::primitive::char" | "core::primitive::char" | "std::char" | "core::char" => {
            Some("char")
        }
        "std::primitive::str" | "core::primitive::str" | "std::str" | "core::str" => Some("str"),

        // Common std types
        "std::string::String" => Some("String"),
        "std::vec::Vec" => Some("Vec"),
        "std::collections::HashMap" => Some("HashMap"),
        "std::collections::HashSet" => Some("HashSet"),
        "std::option::Option" => Some("Option"),
        "std::result::Result" => Some("Result"),

        _ => None,
    }
}

/// Fallback primitive type resolution when PHF is not available
#[cfg(not(feature = "static-resolver"))]
fn get_primitive_mapping(path: &str) -> Option<&'static str> {
    match path {
        // Primitive integer types
        "std::primitive::i8" | "core::primitive::i8" | "std::i8" | "core::i8" => Some("i8"),
        "std::primitive::i16" | "core::primitive::i16" | "std::i16" | "core::i16" => Some("i16"),
        "std::primitive::i32" | "core::primitive::i32" | "std::i32" | "core::i32" => Some("i32"),
        "std::primitive::i64" | "core::primitive::i64" | "std::i64" | "core::i64" => Some("i64"),
        "std::primitive::i128" | "core::primitive::i128" | "std::i128" | "core::i128" => {
            Some("i128")
        }
        "std::primitive::isize" | "core::primitive::isize" | "std::isize" | "core::isize" => {
            Some("isize")
        }

        // Primitive unsigned integer types
        "std::primitive::u8" | "core::primitive::u8" | "std::u8" | "core::u8" => Some("u8"),
        "std::primitive::u16" | "core::primitive::u16" | "std::u16" | "core::u16" => Some("u16"),
        "std::primitive::u32" | "core::primitive::u32" | "std::u32" | "core::u32" => Some("u32"),
        "std::primitive::u64" | "core::primitive::u64" | "std::u64" | "core::u64" => Some("u64"),
        "std::primitive::u128" | "core::primitive::u128" | "std::u128" | "core::u128" => {
            Some("u128")
        }
        "std::primitive::usize" | "core::primitive::usize" | "std::usize" | "core::usize" => {
            Some("usize")
        }

        // Primitive floating point types
        "std::primitive::f32" | "core::primitive::f32" | "std::f32" | "core::f32" => Some("f32"),
        "std::primitive::f64" | "core::primitive::f64" | "std::f64" | "core::f64" => Some("f64"),

        // Other primitive types
        "std::primitive::bool" | "core::primitive::bool" | "std::bool" | "core::bool" => {
            Some("bool")
        }
        "std::primitive::char" | "core::primitive::char" | "std::char" | "core::char" => {
            Some("char")
        }
        "std::primitive::str" | "core::primitive::str" | "std::str" | "core::str" => Some("str"),

        // Common std types
        "std::string::String" => Some("String"),
        "std::vec::Vec" => Some("Vec"),
        "std::collections::HashMap" => Some("HashMap"),
        "std::collections::HashSet" => Some("HashSet"),
        "std::option::Option" => Some("Option"),
        "std::result::Result" => Some("Result"),

        _ => None,
    }
}

const PRIMITIVE_COUNT: usize = 74;

/// Type alias for dynamic path resolvers using HashMap
pub type DynamicPathResolver = PathResolver<HashMap<String, String>>;

/// Type alias for static path resolvers using PHF Map
#[cfg(feature = "static-resolver")]
pub type StaticPathResolver<'a> = PathResolver<&'a Map<&'static str, &'static str>>;

/// Type alias for const path resolvers with only primitive mappings
pub type PrimitivePathResolver = PathResolver<EmptyStorage>;

/// A const empty path resolver.
pub const EMPTY_RESOLVER: PrimitivePathResolver = PathResolver::new(EmptyStorage, false);

/// A const path resolver with primitive type mappings.
pub const PRIMITIVE_RESOLVER: PrimitivePathResolver = PathResolver::new(EmptyStorage, true);

#[derive(Debug, Clone)]
pub struct PathResolver<M> {
    /// Maps normalized path strings to canonical type names
    mappings: M,
    /// Whether to use the built-in primitive mappings
    use_primitives: bool,
}

impl<M> PathResolver<M>
where
    M: MappingStorage,
{
    /// Create a new path resolver with the given mapping storage and primitive flag.
    pub const fn new(mappings: M, use_primitives: bool) -> Self {
        Self {
            mappings,
            use_primitives,
        }
    }

    /// Check if this resolver uses built-in primitive mappings.
    pub const fn uses_primitives(&self) -> bool {
        self.use_primitives
    }

    /// Resolve a syn Path to its canonical type name, if a mapping exists.
    ///
    /// This method uses multiple resolution strategies:
    /// 1. Try the full path as-is
    /// 2. For generic types, try the base type with progressively shorter paths
    /// 3. Try all possible path suffix combinations
    pub fn resolve(&self, path: &Path) -> Option<&str> {
        let stripped = path.strip_raw();

        // Strategy 1: Try the full normalized path first (existing behavior)
        let full_normalized = self.normalize_path(path);
        if let Some(result) = self.try_resolve_base_type(&full_normalized) {
            return Some(result);
        }

        // Strategy 2: For generic types, try progressive path resolution
        if let Some(last_segment) = stripped.segments.last() {
            // Check if this is a generic type (has type arguments)
            let has_generics = !matches!(last_segment.arguments, PathArguments::None);

            if has_generics {
                // Extract the base type name (e.g., "Option" from "Option<T>")
                let base_type = last_segment.ident.to_string();

                // Try resolving with the base type using all possible path combinations
                if let Some(result) = self.resolve_with_progressive_paths(&stripped, &base_type) {
                    return Some(result);
                }
            }
        }

        // Strategy 3: Even for non-generic types, try progressive path resolution
        if let Some(result) = self
            .resolve_with_progressive_paths(&stripped, &stripped.segments.last()?.ident.to_string())
        {
            return Some(result);
        }

        None
    }

    /// Try to resolve a path using progressive path shortening.
    /// This handles cases like:
    /// - "std::option::Option" -> looks for "std::option::Option", "option::Option", "Option"
    /// - "string::String" -> looks for "string::String", "String"
    fn resolve_with_progressive_paths(&self, path: &Path, base_type: &str) -> Option<&str> {
        let segments: Vec<String> = path
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect();

        // For a single segment with generics (like "Option<T>"), we need to check
        // if there are any mappings that end with this base type
        if segments.len() == 1 {
            // Try to find any mapping that ends with this base type
            return self.find_mapping_ending_with(base_type);
        }

        // For multi-segment paths, try progressively shorter paths from the end
        // Example: for "a::b::c::Type<T>", try:
        // - "a::b::c::Type"
        // - "b::c::Type"
        // - "c::Type"
        // - "Type"
        for start_idx in 0..segments.len() {
            let candidate_segments: Vec<String> = segments[start_idx..]
                .iter()
                .take(segments.len() - start_idx - 1) // Exclude the last segment
                .cloned()
                .collect();

            if candidate_segments.is_empty() {
                // Just the base type - try exact match first
                if let Some(result) = self.try_resolve_base_type(base_type) {
                    return Some(result);
                }
                // For multi-segment paths that reduce to just the base type,
                // only apply suffix matching if it could be a reasonable std library shortening
                if self.could_be_stdlib_shortening(&segments, base_type) {
                    return self.find_mapping_ending_with(base_type);
                }
            } else {
                // Build candidate path ending with base type
                let mut full_candidate = candidate_segments;
                full_candidate.push(base_type.to_string());
                let candidate_path = full_candidate.join("::");

                if let Some(result) = self.try_resolve_base_type(&candidate_path) {
                    return Some(result);
                }
            }
        }

        None
    }

    /// Check if a path could reasonably be a shortening of a standard library path.
    /// This is used to determine when suffix matching should be applied.
    fn could_be_stdlib_shortening(&self, segments: &[String], base_type: &str) -> bool {
        // Only allow suffix matching for common std library types
        let common_types = ["Option", "Vec", "HashMap", "HashSet", "Result", "String"];
        if !common_types.contains(&base_type) {
            return false;
        }

        // Check if any segment could be a standard library module
        let stdlib_modules = [
            "std",
            "core",
            "alloc",
            "option",
            "vec",
            "collections",
            "string",
            "result",
        ];

        for segment in segments {
            if stdlib_modules.contains(&segment.as_str()) {
                return true;
            }
        }

        false
    }

    /// Find a mapping that ends with the given base type.
    /// For example, if base_type is "Option", this will find "std::option::Option" -> "Option"
    /// Prefers shorter paths and standard library paths over longer/custom paths.
    fn find_mapping_ending_with(&self, base_type: &str) -> Option<&str> {
        // First try exact match
        if let Some(result) = self.try_resolve_base_type(base_type) {
            return Some(result);
        }

        // For any base type, try to find mappings that end with this type
        let suffix = format!("::{}", base_type);
        let mut candidates: Vec<(&str, &str)> = Vec::new();

        for key in self.mappings.keys() {
            if key.ends_with(&suffix)
                && let Some(result) = self.try_resolve_base_type(key)
            {
                candidates.push((key, result));
            }
        }

        if candidates.is_empty() {
            // No candidates found, check primitive mappings if enabled
            if self.use_primitives {
                #[cfg(feature = "static-resolver")]
                {
                    // Check if any primitive mapping ends with this base type
                    if get_primitive_mapping_static(base_type).is_some() {
                        return get_primitive_mapping_static(base_type);
                    }
                    // For primitive mappings, check common patterns
                    return self.check_primitive_patterns(base_type, get_primitive_mapping_static);
                }
                #[cfg(not(feature = "static-resolver"))]
                {
                    if get_primitive_mapping(base_type).is_some() {
                        return get_primitive_mapping(base_type);
                    }
                    // For primitive mappings, check common patterns
                    return self.check_primitive_patterns(base_type, get_primitive_mapping);
                }
            }
            return None;
        }

        // If we have candidates, choose the best one using prioritization
        // Priority order:
        // 1. Standard library paths (std::, core::, alloc::)
        // 2. Shorter paths (fewer segments)
        // 3. Alphabetical order for tie-breaking

        let mut stdlib_candidates: Vec<(&str, &str)> = Vec::new();
        let mut other_candidates: Vec<(&str, &str)> = Vec::new();

        for (key, result) in candidates {
            if key.starts_with("std::") || key.starts_with("core::") || key.starts_with("alloc::") {
                stdlib_candidates.push((key, result));
            } else {
                other_candidates.push((key, result));
            }
        }

        // Prefer standard library candidates
        let preferred_candidates = if !stdlib_candidates.is_empty() {
            stdlib_candidates
        } else {
            other_candidates
        };

        // Sort by path length (shorter is better), then alphabetically
        let mut sorted_candidates = preferred_candidates;
        sorted_candidates.sort_by(|a, b| {
            let len_cmp = a.0.matches("::").count().cmp(&b.0.matches("::").count());
            if len_cmp == std::cmp::Ordering::Equal {
                a.0.cmp(b.0)
            } else {
                len_cmp
            }
        });

        // Return the best candidate
        sorted_candidates.first().map(|(_, result)| *result)
    }

    /// Check common primitive type patterns for a base type
    fn check_primitive_patterns<F>(&self, base_type: &str, primitive_fn: F) -> Option<&str>
    where
        F: Fn(&str) -> Option<&'static str>,
    {
        // Check common prefixes for primitive types
        for prefix in &["std", "core"] {
            let candidate = format!("{}::{}", prefix, base_type);
            if let Some(result) = primitive_fn(&candidate) {
                return Some(result);
            }
            let candidate = format!("{}::primitive::{}", prefix, base_type);
            if let Some(result) = primitive_fn(&candidate) {
                return Some(result);
            }
            // Also try common module patterns
            let candidate = format!("{}::string::{}", prefix, base_type);
            if let Some(result) = primitive_fn(&candidate) {
                return Some(result);
            }
            let candidate = format!("{}::vec::{}", prefix, base_type);
            if let Some(result) = primitive_fn(&candidate) {
                return Some(result);
            }
            let candidate = format!("{}::collections::{}", prefix, base_type);
            if let Some(result) = primitive_fn(&candidate) {
                return Some(result);
            }
            let candidate = format!("{}::option::{}", prefix, base_type);
            if let Some(result) = primitive_fn(&candidate) {
                return Some(result);
            }
            let candidate = format!("{}::result::{}", prefix, base_type);
            if let Some(result) = primitive_fn(&candidate) {
                return Some(result);
            }
        }
        None
    }

    /// Helper method to try resolving a base type against both custom and primitive mappings.
    fn try_resolve_base_type(&self, base_type: &str) -> Option<&str> {
        // Check custom mappings
        if let Some(canonical) = self.mappings.get(base_type) {
            return Some(canonical);
        }

        // Check primitive mappings if enabled
        if self.use_primitives {
            #[cfg(feature = "static-resolver")]
            let primitive_result = get_primitive_mapping_static(base_type);
            #[cfg(not(feature = "static-resolver"))]
            let primitive_result = get_primitive_mapping(base_type);

            return primitive_result;
        }

        None
    }

    /// Get all registered canonical type names.
    pub fn canonical_types(&self) -> impl Iterator<Item = &str> {
        let custom_types: Vec<&str> = self.mappings.values().collect();

        if self.use_primitives {
            #[cfg(feature = "static-resolver")]
            {
                // Collect all primitive type values manually - same as non-PHF case
                let primitive_types = vec![
                    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
                    "usize", "f32", "f64", "bool", "char", "str", "String", "Vec", "HashMap",
                    "HashSet", "Option", "Result",
                ];
                let all_types: Vec<&str> =
                    custom_types.into_iter().chain(primitive_types).collect();
                Box::new(all_types.into_iter()) as Box<dyn Iterator<Item = &str>>
            }
            #[cfg(not(feature = "static-resolver"))]
            {
                // Collect all primitive type values manually
                let primitive_types = vec![
                    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
                    "usize", "f32", "f64", "bool", "char", "str", "String", "Vec", "HashMap",
                    "HashSet", "Option", "Result",
                ];
                let all_types: Vec<&str> =
                    custom_types.into_iter().chain(primitive_types).collect();
                Box::new(all_types.into_iter()) as Box<dyn Iterator<Item = &str>>
            }
        } else {
            Box::new(custom_types.into_iter()) as Box<dyn Iterator<Item = &str>>
        }
    }

    /// Get all registered path patterns.
    pub fn path_patterns(&self) -> impl Iterator<Item = &str> {
        let custom_patterns: Vec<&str> = self.mappings.keys().collect();

        if self.use_primitives {
            #[cfg(feature = "static-resolver")]
            {
                // Note: This would be quite large, so we'll simplify for consistency
                // In practice, users without PHF likely won't need to iterate over all patterns
                Box::new(custom_patterns.into_iter()) as Box<dyn Iterator<Item = &str>>
            }
            #[cfg(not(feature = "static-resolver"))]
            {
                // Note: This would be quite large, so we'll simplify for the fallback case
                // In practice, users without PHF likely won't need to iterate over all patterns
                Box::new(custom_patterns.into_iter()) as Box<dyn Iterator<Item = &str>>
            }
        } else {
            Box::new(custom_patterns.into_iter()) as Box<dyn Iterator<Item = &str>>
        }
    }

    /// Check if a path has a registered mapping.
    pub fn has_mapping(&self, path: &Path) -> bool {
        let normalized = self.normalize_path(path);
        self.mappings.contains_key(&normalized)
            || (self.use_primitives && {
                #[cfg(feature = "static-resolver")]
                {
                    get_primitive_mapping_static(&normalized).is_some()
                }
                #[cfg(not(feature = "static-resolver"))]
                {
                    get_primitive_mapping(&normalized).is_some()
                }
            })
    }

    /// Get the number of registered mappings.
    pub fn len(&self) -> usize {
        let custom_len = self.mappings.len();
        if self.use_primitives {
            custom_len + {
                #[cfg(feature = "static-resolver")]
                {
                    PRIMITIVE_COUNT
                }
                #[cfg(not(feature = "static-resolver"))]
                {
                    PRIMITIVE_COUNT
                }
            }
        } else {
            custom_len
        }
    }

    /// Check if the resolver is empty.
    pub fn is_empty(&self) -> bool {
        let custom_empty = self.mappings.is_empty();
        custom_empty
            && (!self.use_primitives || {
                #[cfg(feature = "static-resolver")]
                {
                    PRIMITIVE_COUNT == 0
                }
                #[cfg(not(feature = "static-resolver"))]
                {
                    PRIMITIVE_COUNT == 0
                }
            })
    }

    /// Normalize a syn Path to a string for comparison.
    ///
    /// This strips raw prefixes, removes leading colons, and creates
    /// a canonical string representation.
    fn normalize_path(&self, path: &Path) -> String {
        let stripped = path.strip_raw();
        let segments: Vec<String> = stripped
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect();
        segments.join("::")
    }

    /// Normalize a path string by removing leading colons and raw prefixes.
    fn normalize_path_string(&self, path_str: &str) -> String {
        // Remove leading ::
        let path_str = path_str.strip_prefix("::").unwrap_or(path_str);

        // Split by :: and strip raw prefixes from each segment
        let segments: Vec<&str> = path_str
            .split("::")
            .map(|segment| {
                if let Some(stripped) = segment.strip_prefix("r#") {
                    stripped
                } else {
                    segment
                }
            })
            .collect();

        segments.join("::")
    }
}

// Specific implementations for DynamicPathResolver (HashMap-based)
impl DynamicPathResolver {
    /// Create a new dynamic path resolver that uses the built-in primitive mappings.
    pub fn with_primitives() -> Self {
        Self::new(HashMap::new(), true)
    }

    /// Create a new dynamic path resolver from a HashMap with optional primitive mappings.
    pub fn from_map(mappings: HashMap<String, String>, use_primitives: bool) -> Self {
        Self::new(mappings, use_primitives)
    }

    /// Enable or disable the use of built-in primitive mappings.
    pub fn set_use_primitives(&mut self, use_primitives: bool) {
        self.use_primitives = use_primitives;
    }

    /// Add a mapping from a path pattern to a canonical type name.
    ///
    /// The path pattern should be the normalized form (without raw prefixes
    /// or leading colons).
    pub fn add_mapping<S1, S2>(&mut self, path_pattern: S1, canonical_type: S2)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let normalized_pattern = self.normalize_path_string(&path_pattern.into());
        self.mappings
            .insert(normalized_pattern, canonical_type.into());
    }

    /// Clear all mappings.
    pub fn clear(&mut self) {
        self.mappings.clear();
    }
}

impl Default for DynamicPathResolver {
    fn default() -> Self {
        Self::new(HashMap::new(), false)
    }
}

/// Convenience functions for creating common resolver types
impl PathResolver<EmptyStorage> {
    /// Create a static path resolver with only primitive mappings.
    pub const fn primitives_only() -> Self {
        Self::new(EmptyStorage, true)
    }

    /// Create an empty static path resolver.
    pub const fn empty() -> Self {
        Self::new(EmptyStorage, false)
    }
}

/// Example function for creating a static resolver with custom PHF mappings
///
/// # Examples
///
/// ```
/// use desynt::{create_static_resolver, PathResolver};
/// use phf::{phf_map, Map};
///
/// static CUSTOM_MAPPINGS: Map<&'static str, &'static str> = phf_map! {
///     "actix_web::HttpResponse" => "HttpResponse",
///     "serde_json::Value" => "JsonValue",
/// };
///
/// const RESOLVER: PathResolver<&'static Map<&'static str, &'static str>> =
///     create_static_resolver(&CUSTOM_MAPPINGS, true);
/// ```
#[cfg(feature = "static-resolver")]
pub const fn create_static_resolver(
    custom_mappings: &'static Map<&'static str, &'static str>,
    use_primitives: bool,
) -> PathResolver<&'static Map<&'static str, &'static str>> {
    PathResolver::new(custom_mappings, use_primitives)
}
