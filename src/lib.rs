//! A library for stripping raw prefixes from syn objects and resolving type paths.
//!
//! This library provides utilities to work with [`syn`](https://docs.rs/syn) objects:
//!
//! - **Raw identifier handling**: Strip `r#` prefixes from `Ident`, `Path`, and `PathSegment` objects
//! - **Path resolution**: Map various type path representations to canonical forms
//! - **Type group support**: Handle Rust primitives, prelude types, and common std types
//! - **Multiple storage backends**: Use HashMap (dynamic) or phf::Map (static)
//!
//! # Features
//!
//! ## Raw Identifier Stripping
//!
//! Use the [`StripRaw`] trait to remove `r#` prefixes from syn objects:
//!
//! ```
//! use syn::Ident;
//! use desynt::StripRaw;
//!
//! let ident: Ident = syn::parse_str("r#type").unwrap();
//! let stripped = ident.strip_raw();
//! assert_eq!(stripped.to_string(), "type");
//! ```
//!
//! ## Path Resolution
//!
//! Use [`PathResolver`] to normalize type paths and resolve them to canonical forms.
//! This is useful in proc macros where types can be referenced in multiple ways:
//!
//! ```
//! use desynt::{TypeGroups, DynamicPathResolver};
//! use std::collections::HashMap;
//! use syn::Path;
//!
//! let mut mappings = HashMap::new();
//! mappings.insert("my_crate::types::UserId".to_string(), "UserId".to_string());
//!
//! let resolver = DynamicPathResolver::from_map(mappings, TypeGroups::ALL);
//!
//! // Resolves custom types
//! let path: Path = syn::parse_str("my_crate::types::UserId").unwrap();
//! assert_eq!(resolver.resolve(&path), Some("UserId"));
//!
//! // Also handles type groups with various path forms
//! let path: Path = syn::parse_str("std::option::Option").unwrap();
//! assert_eq!(resolver.resolve(&path), Some("Option"));
//!
//! let path: Path = syn::parse_str("Option<String>").unwrap();
//! assert_eq!(resolver.resolve(&path), Some("Option"));
//! ```
//!
//! ## Built-in Type Categories
//!
//! The [`TypeGroups`] struct lets you control which standard types are automatically resolved:
//!
//! - **Primitives**: Language primitives (i8, u32, f64, bool, char, str, etc.)
//! - **Prelude**: Types in the Rust prelude (String, Vec, Option, Result, Box)
//! - **Common std**: Frequently used std types (HashMap, HashSet)
//!
//! Use the predefined constants for common configurations:
//! - [`TypeGroups::NONE`] - No type groups
//! - [`TypeGroups::PRIMITIVES`] - Only primitives
//! - [`TypeGroups::PRELUDE`] - Primitives + prelude types
//! - [`TypeGroups::ALL`] - All type groups

use std::collections::HashMap;

mod definitions;

#[cfg(feature = "static-resolver")]
use phf::Map;
use syn::{Ident, Path, PathArguments, PathSegment};

/// Storage backend for path-to-canonical-type mappings.
///
/// This trait abstracts over different storage implementations used by [`PathResolver`],
/// allowing for dynamic (HashMap), static (phf::Map), or empty storage backends.
pub trait MappingStorage {
    /// Returns the canonical type name for the given normalized path.
    fn get(&self, path: &str) -> Option<&str>;

    /// Returns `true` if the storage contains a mapping for the given path.
    fn contains_key(&self, path: &str) -> bool;

    /// Returns the number of mappings in the storage.
    fn len(&self) -> usize;

    /// Returns `true` if the storage contains no mappings.
    fn is_empty(&self) -> bool;

    /// Returns an iterator over all path patterns in the storage.
    fn keys(&self) -> Box<dyn Iterator<Item = &str> + '_>;

    /// Returns an iterator over all canonical type names in the storage.
    fn values(&self) -> Box<dyn Iterator<Item = &str> + '_>;
}

/// Implementation of MappingStorage for HashMap (dynamic mappings).
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

/// Implementation of MappingStorage for PHF Map (static mappings).
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

/// Implementation of MappingStorage for &PHF Map (static mappings).
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

/// Empty storage implementation with no custom mappings.
///
/// This storage backend is useful for const resolvers that only use
/// type group mappings without any custom path mappings.
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

/// Strips raw identifier prefixes (`r#`) from syn objects.
///
/// This trait is implemented for [`Ident`], [`Path`], and [`PathSegment`],
/// allowing you to normalize identifiers that use raw identifier syntax.
pub trait StripRaw {
    /// The type returned after stripping raw prefixes.
    type Output;

    /// Returns a copy of this object with all raw identifier prefixes removed.
    ///
    /// For example, `r#type` becomes `type`.
    fn strip_raw(&self) -> Self::Output;
}

/// Checks whether a syn object contains raw identifier prefixes.
///
/// This trait is implemented for [`Ident`], [`Path`], and [`PathSegment`].
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

    /// Check if the string represents a raw identifier (starts with `r#`).
    ///
    /// # Examples
    ///
    /// ```
    /// use desynt::utils::is_raw_ident;
    ///
    /// assert!(is_raw_ident("r#type"));
    /// assert!(!is_raw_ident("type"));
    /// ```
    pub fn is_raw_ident(s: &str) -> bool {
        s.starts_with("r#")
    }

    /// Remove the `r#` prefix from a string if present.
    ///
    /// If the string doesn't start with `r#`, returns the original string unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use desynt::utils::strip_raw_prefix;
    ///
    /// assert_eq!(strip_raw_prefix("r#type"), "type");
    /// assert_eq!(strip_raw_prefix("type"), "type");
    /// ```
    pub fn strip_raw_prefix(s: &str) -> &str {
        if is_raw_ident(s) { &s[2..] } else { s }
    }

    /// Create a new [`Ident`] from a string, automatically handling raw prefixes.
    ///
    /// If the input starts with `r#`, creates a raw identifier.
    /// Otherwise, creates a regular identifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use desynt::utils::ident_from_string;
    ///
    /// let ident = ident_from_string("r#type").unwrap();
    /// assert_eq!(ident.to_string(), "r#type");
    /// ```
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
/// 2. **Generic type resolution** - Extracts base type from generics (e.g., `Option<T>` -> `Option`)
/// 3. **Progressive path resolution** - Tries shorter path variations for standard library types
///
/// ## Progressive Path Resolution
///
/// This feature automatically handles common path variations without requiring explicit mappings
/// for every variant. For example, if you map `std::option::Option` -> `Option`, the resolver
/// will automatically handle:
/// - `Option<T>` -> `Option` (single segment with generics)
/// - `option::Option<T>` -> `Option` (shortened qualified path)
/// - `std::option::Option<T>` -> `Option` (full qualified path with generics)
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
/// use desynt::{TypeGroups, PathResolver};
/// use std::collections::HashMap;
/// use syn::Path;
///
/// let mut mappings = HashMap::new();
/// mappings.insert("std::primitive::f64".to_string(), "f64".to_string());
/// mappings.insert("core::primitive::f64".to_string(), "f64".to_string());
///
/// let resolver = PathResolver::new(mappings, TypeGroups::ALL);
///
/// let path: Path = syn::parse_str("::std::primitive::f64").unwrap();
/// if let Some(canonical) = resolver.resolve(&path) {
///     assert_eq!(canonical, "f64");
/// }
/// ```
///
/// ## Static Usage with PHF Map
/// ```
/// use desynt::{TypeGroups, PathResolver, EmptyStorage};
/// use phf::{phf_map, Map};
///
/// // Define mappings in tests/examples where phf_map is allowed
/// // static CUSTOM_MAPPINGS: Map<&'static str, &'static str> = phf_map! {
/// //     "actix_web::HttpResponse" => "HttpResponse",
/// //     "serde_json::Value" => "JsonValue",
/// // };
///
/// // const RESOLVER: PathResolver<&'static Map<&'static str, &'static str>> =
/// //     PathResolver::new(&CUSTOM_MAPPINGS, TypeGroups::ALL);
///
/// // Or with only primitives
/// const PRIMITIVE_RESOLVER: PathResolver<EmptyStorage> =
///     PathResolver::new(EmptyStorage, TypeGroups::PRIMITIVES);
/// ```

/// Specify type groups to include automatically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeGroups {
    /// Whether to include Rust language primitives (i8, u32, f64, bool, char, str, etc.).
    pub primitives: bool,
    /// Whether to include Rust prelude types (String, Vec, Option, Result, Box).
    pub prelude: bool,
    /// Whether to include common std library types (HashMap, HashSet, BTreeMap, BTreeSet, LinkedList, Cow, RefCell, Arc, Rc).
    pub common_std: bool,
}

impl TypeGroups {
    /// No type groups.
    pub const NONE: Self = Self {
        primitives: false,
        prelude: false,
        common_std: false,
    };

    /// Only Rust language primitives.
    pub const PRIMITIVES: Self = Self {
        primitives: true,
        prelude: false,
        common_std: false,
    };

    /// Primitives and prelude types.
    pub const PRELUDE: Self = Self {
        primitives: true,
        prelude: true,
        common_std: false,
    };

    /// All type groups (primitives + prelude + common std types).
    pub const ALL: Self = Self {
        primitives: true,
        prelude: true,
        common_std: true,
    };

    /// Check if any type groups are enabled.
    pub const fn is_empty(&self) -> bool {
        !self.primitives && !self.prelude && !self.common_std
    }
}

impl Default for TypeGroups {
    fn default() -> Self {
        Self::NONE
    }
}

/// Type alias for dynamic path resolvers using [`HashMap`] storage.
///
/// This resolver allows adding and removing mappings at runtime.
pub type DynamicPathResolver = PathResolver<HashMap<String, String>>;

/// Type alias for static path resolvers using PHF [`Map`] storage.
///
/// This resolver uses compile-time static mappings for zero-cost lookups.
#[cfg(feature = "static-resolver")]
pub type StaticPathResolver<'a> = PathResolver<&'a Map<&'static str, &'static str>>;

/// Type alias for const path resolvers with [`EmptyStorage`].
///
/// These resolvers have no custom mappings, only type group mappings.
pub type PrimitivePathResolver = PathResolver<EmptyStorage>;

/// Const resolver with no mappings at all.
///
/// This resolver will not resolve any paths.
pub const EMPTY_RESOLVER: PrimitivePathResolver = PathResolver::new(EmptyStorage, TypeGroups::NONE);

/// Const resolver with only primitive type mappings (i8, u32, bool, etc.).
pub const PRIMITIVE_RESOLVER: PrimitivePathResolver =
    PathResolver::new(EmptyStorage, TypeGroups::PRIMITIVES);

/// Const resolver with primitives and prelude types (String, Vec, Option, etc.).
pub const PRELUDE_RESOLVER: PrimitivePathResolver =
    PathResolver::new(EmptyStorage, TypeGroups::PRELUDE);

/// Const resolver with all type groups enabled.
///
/// This includes primitives, prelude types, and common std types.
pub const ALL_RESOLVER: PrimitivePathResolver = PathResolver::new(EmptyStorage, TypeGroups::ALL);

/// Path resolver that maps various path representations to canonical types.
///
/// See the module-level documentation for detailed usage examples.
#[derive(Debug, Clone)]
pub struct PathResolver<M> {
    /// Maps normalized path strings to canonical type names.
    mappings: M,
    /// Which type group mappings to include.
    groups: TypeGroups,
}

impl<M> PathResolver<M>
where
    M: MappingStorage,
{
    /// Create a new path resolver with the specified storage backend and type groups.
    ///
    /// # Examples
    ///
    /// ```
    /// use desynt::{PathResolver, TypeGroups, EmptyStorage};
    ///
    /// const RESOLVER: PathResolver<EmptyStorage> =
    ///     PathResolver::new(EmptyStorage, TypeGroups::PRIMITIVES);
    /// ```
    pub const fn new(mappings: M, groups: TypeGroups) -> Self {
        Self { mappings, groups }
    }

    /// Return the current type groups configuration.
    pub const fn groups(&self) -> TypeGroups {
        self.groups
    }

    /// Return `true` if any type group mappings are enabled.
    pub const fn uses_groups(&self) -> bool {
        !self.groups.is_empty()
    }

    /// Return `true` if primitive type mappings are enabled.
    pub const fn uses_primitives(&self) -> bool {
        self.groups.primitives
    }

    /// Return `true` if prelude type mappings are enabled.
    pub const fn uses_prelude(&self) -> bool {
        self.groups.prelude
    }

    /// Return `true` if common std type mappings are enabled.
    pub const fn uses_common_std(&self) -> bool {
        self.groups.common_std
    }

    /// Resolve a syn [`Path`] to its canonical type name.
    ///
    /// This method uses multiple resolution strategies:
    /// 1. Exact path matching - Direct lookup of the full path
    /// 2. Generic type resolution - Extracts base type from generics (e.g., `Option<T>` -> `Option`)
    /// 3. Progressive path resolution - Tries shorter path variations for standard library types
    ///
    /// # Returns
    ///
    /// Returns `Some(&str)` with the canonical type name if a mapping exists,
    /// otherwise returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use desynt::{DynamicPathResolver, TypeGroups};
    /// use syn::Path;
    ///
    /// let resolver = DynamicPathResolver::with_all_groups();
    /// let path: Path = syn::parse_str("std::option::Option").unwrap();
    /// assert_eq!(resolver.resolve(&path), Some("Option"));
    /// ```
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
    ///
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
    ///
    /// This is used to determine when suffix matching should be applied.
    fn could_be_stdlib_shortening(&self, segments: &[String], base_type: &str) -> bool {
        // Only allow suffix matching for prelude types and common std library types
        let common_types = [
            "Option",
            "Vec",
            "HashMap",
            "HashSet",
            "Result",
            "String",
            "Box",
            "BTreeMap",
            "BTreeSet",
            "LinkedList",
            "Cow",
            "RefCell",
            "Arc",
            "Rc",
        ];
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
            "borrow",
            "boxed",
            "cell",
            "sync",
            "rc",
            "hash_map",
            "hash_set",
            "btree_map",
            "btree_set",
            "linked_list",
        ];

        for segment in segments {
            if stdlib_modules.contains(&segment.as_str()) {
                return true;
            }
        }

        false
    }

    /// Find a mapping that ends with the given base type.
    ///
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
            if key.ends_with(&suffix) {
                if let Some(result) = self.try_resolve_base_type(key) {
                    candidates.push((key, result));
                }
            }
        }

        if candidates.is_empty() {
            // No candidates found, check built-in mappings if enabled
            if !self.groups.is_empty() {
                #[cfg(feature = "static-resolver")]
                {
                    // Check if any built-in mapping matches this base type
                    if definitions::get_builtin_mapping_static(base_type, self.groups).is_some() {
                        return definitions::get_builtin_mapping_static(base_type, self.groups);
                    }
                    // For built-in mappings, check common patterns
                    return self.check_builtin_patterns(base_type);
                }
                #[cfg(not(feature = "static-resolver"))]
                {
                    if definitions::get_builtin_mapping(base_type, self.groups).is_some() {
                        return definitions::get_builtin_mapping(base_type, self.groups);
                    }
                    // For built-in mappings, check common patterns
                    return self.check_builtin_patterns(base_type);
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

    /// Check common primitive type patterns for a base type.
    fn check_builtin_patterns(&self, base_type: &str) -> Option<&str> {
        // Check common prefixes for type groups
        for prefix in &["std", "core", "alloc"] {
            let patterns = [
                format!("{}::{}", prefix, base_type),
                format!("{}::primitive::{}", prefix, base_type),
                format!("{}::string::{}", prefix, base_type),
                format!("{}::vec::{}", prefix, base_type),
                format!("{}::collections::{}", prefix, base_type),
                format!("{}::collections::hash_map::{}", prefix, base_type),
                format!("{}::collections::hash_set::{}", prefix, base_type),
                format!("{}::collections::btree_map::{}", prefix, base_type),
                format!("{}::collections::btree_set::{}", prefix, base_type),
                format!("{}::collections::linked_list::{}", prefix, base_type),
                format!("{}::option::{}", prefix, base_type),
                format!("{}::result::{}", prefix, base_type),
                format!("{}::boxed::{}", prefix, base_type),
                format!("{}::borrow::{}", prefix, base_type),
                format!("{}::cell::{}", prefix, base_type),
                format!("{}::sync::{}", prefix, base_type),
                format!("{}::rc::{}", prefix, base_type),
            ];

            for candidate in &patterns {
                #[cfg(feature = "static-resolver")]
                {
                    if let Some(result) =
                        definitions::get_builtin_mapping_static(candidate, self.groups)
                    {
                        return Some(result);
                    }
                }
                #[cfg(not(feature = "static-resolver"))]
                {
                    if let Some(result) = definitions::get_builtin_mapping(candidate, self.groups) {
                        return Some(result);
                    }
                }
            }
        }
        None
    }

    /// Try resolving a base type against both custom and built-in mappings.
    fn try_resolve_base_type(&self, base_type: &str) -> Option<&str> {
        // Check custom mappings
        if let Some(canonical) = self.mappings.get(base_type) {
            return Some(canonical);
        }

        // Check built-in mappings if enabled
        if !self.groups.is_empty() {
            #[cfg(feature = "static-resolver")]
            let builtin_result = definitions::get_builtin_mapping_static(base_type, self.groups);
            #[cfg(not(feature = "static-resolver"))]
            let builtin_result = definitions::get_builtin_mapping(base_type, self.groups);

            return builtin_result;
        }

        None
    }

    /// Return an iterator over all canonical type names known to this resolver.
    ///
    /// This includes both custom mappings and type group mappings (if enabled).
    pub fn canonical_types(&self) -> impl Iterator<Item = &str> {
        let custom_types: Vec<&str> = self.mappings.values().collect();

        if !self.groups.is_empty() {
            let mut builtin_types = Vec::new();

            if self.groups.primitives {
                builtin_types.extend_from_slice(&[
                    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
                    "usize", "f32", "f64", "bool", "char", "str",
                ]);
            }

            if self.groups.prelude {
                builtin_types.extend_from_slice(&["String", "Vec", "Option", "Result", "Box"]);
            }

            if self.groups.common_std {
                builtin_types.extend_from_slice(&[
                    "HashMap",
                    "HashSet",
                    "BTreeMap",
                    "BTreeSet",
                    "LinkedList",
                    "Cow",
                    "RefCell",
                    "Arc",
                    "Rc",
                ]);
            }

            let all_types: Vec<&str> = custom_types.into_iter().chain(builtin_types).collect();
            Box::new(all_types.into_iter()) as Box<dyn Iterator<Item = &str>>
        } else {
            Box::new(custom_types.into_iter()) as Box<dyn Iterator<Item = &str>>
        }
    }

    /// Return an iterator over all registered path patterns.
    ///
    /// Note: This only returns custom path patterns, not type group patterns,
    /// as type group patterns are numerous and implementation-dependent.
    pub fn path_patterns(&self) -> impl Iterator<Item = &str> {
        // Note: For simplicity, we only return custom patterns
        // Built-in patterns are numerous and implementation-dependent
        let custom_patterns: Vec<&str> = self.mappings.keys().collect();
        Box::new(custom_patterns.into_iter()) as Box<dyn Iterator<Item = &str>>
    }

    /// Return `true` if this resolver has a mapping for the given path.
    ///
    /// This checks both custom mappings and type group mappings (if enabled).
    pub fn has_mapping(&self, path: &Path) -> bool {
        let normalized = self.normalize_path(path);
        self.mappings.contains_key(&normalized)
            || (!self.groups.is_empty() && {
                #[cfg(feature = "static-resolver")]
                {
                    definitions::get_builtin_mapping_static(&normalized, self.groups).is_some()
                }
                #[cfg(not(feature = "static-resolver"))]
                {
                    definitions::get_builtin_mapping(&normalized, self.groups).is_some()
                }
            })
    }

    /// Return the total number of custom mappings in this resolver.
    ///
    /// Note: This does not include type group mappings as their count is
    /// implementation-dependent and subject to change.
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Return `true` if this resolver has no mappings.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty() && self.groups.is_empty()
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
    /// Create a new dynamic path resolver with all type groups enabled.
    ///
    /// This is equivalent to calling `new(HashMap::new(), TypeGroups::ALL)`.
    pub fn with_all_groups() -> Self {
        Self::new(HashMap::new(), TypeGroups::ALL)
    }

    /// Create a new dynamic path resolver with only primitive type mappings.
    ///
    /// This is equivalent to calling `new(HashMap::new(), TypeGroups::PRIMITIVES)`.
    pub fn with_primitives() -> Self {
        Self::new(HashMap::new(), TypeGroups::PRIMITIVES)
    }

    /// Create a new dynamic path resolver with primitives and prelude types.
    ///
    /// This is equivalent to calling `new(HashMap::new(), TypeGroups::PRELUDE)`.
    pub fn with_prelude() -> Self {
        Self::new(HashMap::new(), TypeGroups::PRELUDE)
    }

    /// Create a new dynamic path resolver from an existing HashMap with specified type groups.
    pub fn from_map(mappings: HashMap<String, String>, groups: TypeGroups) -> Self {
        Self::new(mappings, groups)
    }

    /// Set which type groups to use for this resolver.
    pub fn set_groups(&mut self, groups: TypeGroups) {
        self.groups = groups;
    }

    /// Enable or disable the use of built-in primitive mappings.
    ///
    /// Deprecated: Use `set_groups` with `TypeGroups::PRIMITIVES` instead.
    #[deprecated(since = "0.2.0", note = "Use set_groups with TypeGroups instead")]
    pub fn set_use_primitives(&mut self, use_primitives: bool) {
        self.groups = if use_primitives {
            TypeGroups::ALL
        } else {
            TypeGroups::NONE
        };
    }

    /// Add a custom mapping from a path pattern to a canonical type name.
    ///
    /// The path pattern will be normalized (raw prefixes and leading `::` removed)
    /// before being stored.
    pub fn add_mapping<S1, S2>(&mut self, path_pattern: S1, canonical_type: S2)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let normalized_pattern = self.normalize_path_string(&path_pattern.into());
        self.mappings
            .insert(normalized_pattern, canonical_type.into());
    }

    /// Remove all custom mappings from this resolver.
    ///
    /// Type group mappings (if enabled) are not affected.
    pub fn clear(&mut self) {
        self.mappings.clear();
    }
}

impl Default for DynamicPathResolver {
    fn default() -> Self {
        Self::new(HashMap::new(), TypeGroups::NONE)
    }
}

/// Convenience functions for creating common resolver types.
impl PathResolver<EmptyStorage> {
    /// Create a const path resolver with all type groups enabled.
    ///
    /// This resolver has no custom mappings, only type group mappings.
    pub const fn all_groups() -> Self {
        Self::new(EmptyStorage, TypeGroups::ALL)
    }

    /// Create a const path resolver with primitives and prelude types.
    ///
    /// This resolver has no custom mappings, only primitives and prelude type mappings.
    pub const fn with_prelude() -> Self {
        Self::new(EmptyStorage, TypeGroups::PRELUDE)
    }

    /// Create a const path resolver with only primitive type mappings.
    ///
    /// This resolver has no custom mappings, only primitive type mappings.
    pub const fn primitives_only() -> Self {
        Self::new(EmptyStorage, TypeGroups::PRIMITIVES)
    }

    /// Create an empty const path resolver with no mappings.
    ///
    /// This resolver will not resolve any paths.
    pub const fn empty() -> Self {
        Self::new(EmptyStorage, TypeGroups::NONE)
    }
}

/// Create a static resolver with custom PHF mappings.
///
/// # Examples
///
/// ```
/// use desynt::{TypeGroups, create_static_resolver, PathResolver};
/// use phf::{phf_map, Map};
///
/// const CUSTOM_MAPPINGS: Map<&'static str, &'static str> = phf_map! {
///     "actix_web::HttpResponse" => "HttpResponse",
///     "serde_json::Value" => "JsonValue",
/// };
///
/// const RESOLVER: PathResolver<&'static Map<&'static str, &'static str>> =
///     create_static_resolver(&CUSTOM_MAPPINGS, TypeGroups::ALL);
/// ```
#[cfg(feature = "static-resolver")]
pub const fn create_static_resolver(
    custom_mappings: &'static Map<&'static str, &'static str>,
    groups: TypeGroups,
) -> PathResolver<&'static Map<&'static str, &'static str>> {
    PathResolver::new(custom_mappings, groups)
}
