//! Variant-qualified spec paths used for composed opinion provenance.
//!
//! These paths are not the same as concrete prim namespace paths. They can
//! carry variant selections and property suffixes while still resolving to a
//! concrete authored prim site.
//!
//! Spec: AOUSD Core §8 (paths), §10.5 (variant selection), and the sparse array
//! edits proposal's discussion of sparse-composed `SdfPathExpression`
//! provenance.

use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
    interner::{TokenId, TokenInterner},
    path::{Path, PathError, PathId, PathInterner},
};

/// A single spec-path component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpecComponent {
    /// A concrete prim namespace segment.
    Prim(TokenId),
    /// A variant selection inserted after the preceding prim segment.
    VariantSelection {
        /// Variant set name.
        set: TokenId,
        /// Selected variant name.
        variant: TokenId,
    },
}

/// A variant-qualified spec path.
///
/// Unlike [`Path`], this can represent variant selections and property suffixes
/// while still preserving the concrete prim path used to look up authored data.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpecPath {
    prim_path: PathId,
    components: Box<[SpecComponent]>,
    property: Option<TokenId>,
}

impl SpecPath {
    /// Builds a plain prim spec path from a concrete prim path.
    #[must_use]
    pub fn from_prim_path(prim_path: PathId, paths: &PathInterner) -> Self {
        let components = paths
            .resolve(prim_path)
            .segments()
            .iter()
            .copied()
            .map(SpecComponent::Prim)
            .collect();
        Self {
            prim_path,
            components,
            property: None,
        }
    }

    /// Builds a variant-qualified spec path by inserting `selections` after
    /// `variant_host` in `prim_path`.
    ///
    /// `variant_host` must be equal to or a prefix of `prim_path`.
    #[must_use]
    pub fn from_variant_qualified_prim_path(
        prim_path: PathId,
        variant_host: PathId,
        selections: &[(TokenId, TokenId)],
        paths: &PathInterner,
    ) -> Self {
        let prim = paths.resolve(prim_path);
        let host = paths.resolve(variant_host);
        let remainder = prim
            .strip_prefix(host)
            .expect("variant host must be equal to or a prefix of prim path");

        let mut components = Vec::new();
        components.extend(host.segments().iter().copied().map(SpecComponent::Prim));
        components.extend(
            selections
                .iter()
                .copied()
                .map(|(set, variant)| SpecComponent::VariantSelection { set, variant }),
        );
        components.extend(remainder.iter().copied().map(SpecComponent::Prim));

        Self {
            prim_path,
            components: components.into_boxed_slice(),
            property: None,
        }
    }

    /// Parses a spec path like `/A{v=red}B/C.attr`.
    pub fn parse(
        s: &str,
        tokens: &mut TokenInterner,
        paths: &mut PathInterner,
    ) -> Result<Self, SpecPathError> {
        if !s.starts_with('/') {
            return Err(SpecPathError::NotAbsolute);
        }

        let mut components = Vec::new();
        let mut prim_segments = Vec::new();
        let bytes = s.as_bytes();
        let mut idx = 1_usize;
        let len = bytes.len();
        let mut property = None;

        while idx < len {
            match bytes[idx] {
                b'/' => {
                    idx += 1;
                }
                b'{' => {
                    idx += 1;
                    let start = idx;
                    while idx < len && bytes[idx] != b'=' {
                        idx += 1;
                    }
                    if idx == len {
                        return Err(SpecPathError::MalformedVariantSelection);
                    }
                    let set = &s[start..idx];
                    idx += 1;
                    let variant_start = idx;
                    while idx < len && bytes[idx] != b'}' {
                        idx += 1;
                    }
                    if idx == len {
                        return Err(SpecPathError::MalformedVariantSelection);
                    }
                    let variant = &s[variant_start..idx];
                    idx += 1;
                    components.push(SpecComponent::VariantSelection {
                        set: tokens.intern(set),
                        variant: tokens.intern(variant),
                    });
                }
                b'.' => {
                    if idx + 1 >= len {
                        return Err(SpecPathError::EmptyPropertyName);
                    }
                    property = Some(tokens.intern(&s[idx + 1..]));
                    break;
                }
                _ => {
                    let start = idx;
                    while idx < len {
                        match bytes[idx] {
                            b'/' | b'{' | b'.' => break,
                            _ => idx += 1,
                        }
                    }
                    let segment = &s[start..idx];
                    if segment.is_empty() {
                        return Err(SpecPathError::EmptySegment);
                    }
                    let token = tokens.intern(segment);
                    prim_segments.push(token);
                    components.push(SpecComponent::Prim(token));
                }
            }
        }

        let prim_path = if prim_segments.is_empty() {
            paths.intern(Path::root())
        } else {
            paths.intern(Path::root().join(&prim_segments))
        };

        Ok(Self {
            prim_path,
            components: components.into_boxed_slice(),
            property,
        })
    }

    /// Returns the concrete prim path used to look up authored `PrimSpec` data.
    #[must_use]
    pub const fn prim_path(&self) -> PathId {
        self.prim_path
    }

    /// Returns the property suffix, if any.
    #[must_use]
    pub const fn property(&self) -> Option<TokenId> {
        self.property
    }

    /// Returns the structured components.
    #[must_use]
    pub fn components(&self) -> &[SpecComponent] {
        &self.components
    }

    /// Returns a copy of this path with a property suffix attached.
    #[must_use]
    pub fn with_property(&self, property: TokenId) -> Self {
        let mut out = self.clone();
        out.property = Some(property);
        out
    }

    /// Formats this path in AOUSD-style spec-path syntax.
    #[must_use]
    pub fn display(&self, tokens: &TokenInterner) -> String {
        let mut out = String::from("/");
        let mut need_slash = false;
        for component in self.components.iter().copied() {
            match component {
                SpecComponent::Prim(segment) => {
                    if need_slash {
                        out.push('/');
                    }
                    out.push_str(tokens.resolve(segment));
                    need_slash = true;
                }
                SpecComponent::VariantSelection { set, variant } => {
                    out.push('{');
                    out.push_str(tokens.resolve(set));
                    out.push('=');
                    out.push_str(tokens.resolve(variant));
                    out.push('}');
                    need_slash = false;
                }
            }
        }
        if let Some(property) = self.property {
            out.push('.');
            out.push_str(tokens.resolve(property));
        }
        out
    }
}

/// Errors that can occur while parsing a [`SpecPath`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecPathError {
    /// Path is not absolute (doesn't start with `/`).
    NotAbsolute,
    /// Path contains an empty prim segment.
    EmptySegment,
    /// A variant selection was malformed.
    MalformedVariantSelection,
    /// Property suffix was present but empty.
    EmptyPropertyName,
    /// Underlying concrete prim path was invalid.
    InvalidPrimPath(PathError),
}

impl From<PathError> for SpecPathError {
    fn from(value: PathError) -> Self {
        Self::InvalidPrimPath(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interner::TokenInterner;

    #[test]
    fn from_variant_qualified_prim_path_formats_expected_string() {
        let mut tokens = TokenInterner::default();
        let mut paths = PathInterner::default();
        let host = paths.intern(Path::parse_absolute("/Sarah", &mut tokens).expect("path"));
        let concrete = paths
            .intern(Path::parse_absolute("/Sarah/FaceRig/EyesRig", &mut tokens).expect("path"));
        let selection = (tokens.intern("modelComplexity"), tokens.intern("full"));

        let spec = SpecPath::from_variant_qualified_prim_path(concrete, host, &[selection], &paths);

        assert_eq!(
            spec.display(&tokens),
            "/Sarah{modelComplexity=full}FaceRig/EyesRig"
        );
        assert_eq!(spec.prim_path(), concrete);
    }

    #[test]
    fn parse_round_trips_variant_property_path() {
        let mut tokens = TokenInterner::default();
        let mut paths = PathInterner::default();

        let spec = SpecPath::parse(
            "/A{nestedVariantSet=nestedVariant}.test",
            &mut tokens,
            &mut paths,
        )
        .expect("spec path");

        assert_eq!(
            spec.display(&tokens),
            "/A{nestedVariantSet=nestedVariant}.test"
        );
        let prim = paths.resolve(spec.prim_path());
        assert_eq!(prim.display(&tokens), "/A");
    }
}
