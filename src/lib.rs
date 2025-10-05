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
use syn::{Ident, Path, PathSegment};

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
        if ident_str.starts_with("r#") {
            // Create a new Ident without the raw prefix
            Ident::new(&ident_str[2..], self.span())
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
/// # Examples
///
/// ```
/// use desynt::PathResolver;
/// use syn::Path;
///
/// let mut resolver = PathResolver::new();
///
/// // Define canonical mappings
/// resolver.add_mapping("std::primitive::f64", "f64");
/// resolver.add_mapping("core::primitive::f64", "f64");
/// resolver.add_mapping("std::string::String", "String");
///
/// // Resolve paths to canonical forms
/// let path: Path = syn::parse_str("::std::primitive::f64").unwrap();
/// if let Some(canonical) = resolver.resolve(&path) {
///     assert_eq!(canonical, "f64");
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PathResolver {
    /// Maps normalized path strings to canonical type names
    mappings: HashMap<String, String>,
}

impl PathResolver {
    /// Create a new empty path resolver.
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    /// Create a path resolver with common primitive type mappings.
    pub fn with_primitives() -> Self {
        let mut resolver = Self::new();

        // Add common primitive type mappings
        let primitives = [
            "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
            "f32", "f64", "bool", "char", "str",
        ];

        for primitive in &primitives {
            resolver.add_mapping(&format!("std::primitive::{}", primitive), *primitive);
            resolver.add_mapping(&format!("core::primitive::{}", primitive), *primitive);
            resolver.add_mapping(&format!("std::{}", primitive), *primitive);
            resolver.add_mapping(&format!("core::{}", primitive), *primitive);
        }

        // Add common std types
        resolver.add_mapping("std::string::String", "String");
        resolver.add_mapping("std::vec::Vec", "Vec");
        resolver.add_mapping("std::collections::HashMap", "HashMap");
        resolver.add_mapping("std::collections::HashSet", "HashSet");
        resolver.add_mapping("std::option::Option", "Option");
        resolver.add_mapping("std::result::Result", "Result");

        resolver
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

    /// Resolve a syn Path to its canonical type name, if a mapping exists.
    pub fn resolve(&self, path: &Path) -> Option<&str> {
        let normalized = self.normalize_path(path);
        self.mappings.get(&normalized).map(|s| s.as_str())
    }

    /// Get all registered canonical type names.
    pub fn canonical_types(&self) -> impl Iterator<Item = &str> {
        self.mappings.values().map(|s| s.as_str())
    }

    /// Get all registered path patterns.
    pub fn path_patterns(&self) -> impl Iterator<Item = &str> {
        self.mappings.keys().map(|s| s.as_str())
    }

    /// Check if a path has a registered mapping.
    pub fn has_mapping(&self, path: &Path) -> bool {
        let normalized = self.normalize_path(path);
        self.mappings.contains_key(&normalized)
    }

    /// Get the number of registered mappings.
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Check if the resolver is empty.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    /// Clear all mappings.
    pub fn clear(&mut self) {
        self.mappings.clear();
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
                if segment.starts_with("r#") {
                    &segment[2..]
                } else {
                    segment
                }
            })
            .collect();

        segments.join("::")
    }
}

impl Default for PathResolver {
    fn default() -> Self {
        Self::new()
    }
}
