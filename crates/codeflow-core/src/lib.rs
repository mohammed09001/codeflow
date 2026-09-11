//! Stable, provider-independent primitives shared by all CodeFlow backends.
//! These types deliberately contain no provider-local identifier or source text.

use std::fmt;
use std::path::Path;

use blake3::Hasher;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CORE_SCHEMA_VERSION: u16 = 1;

macro_rules! canonical_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);
        impl $name {
            pub fn derive(domain: &str, parts: &[&str]) -> Self {
                let mut hasher = Hasher::new();
                hasher.update(domain.as_bytes());
                hasher.update(&[0]);
                for part in parts {
                    hasher.update(part.as_bytes());
                    hasher.update(&[0]);
                }
                Self(hasher.finalize().to_hex().to_string())
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
            pub fn from_serialized(value: String) -> Option<Self> {
                (value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
                    .then_some(Self(value))
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

canonical_id!(ProjectId);
canonical_id!(RevisionId);
canonical_id!(EntityId);
canonical_id!(EdgeId);
canonical_id!(EvidenceId);
canonical_id!(SourceBlobId);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SchemaEnvelope<T> {
    pub schema_version: u16,
    pub payload: T,
}
impl<T> SchemaEnvelope<T> {
    pub fn current(payload: T) -> Self {
        Self {
            schema_version: CORE_SCHEMA_VERSION,
            payload,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceIdentity {
    pub normalized_path: String,
    pub blob_id: SourceBlobId,
    pub byte_len: u64,
}
impl SourceIdentity {
    pub fn from_bytes(path: &Path, bytes: &[u8]) -> Result<Self, IdentityError> {
        let normalized_path = normalize_relative_path(path)?;
        let content_digest = blake3::hash(bytes).to_hex();
        Ok(Self {
            blob_id: SourceBlobId::derive(
                "codeflow.source.blake3.v1",
                &[&normalized_path, content_digest.as_ref()],
            ),
            normalized_path,
            byte_len: bytes.len() as u64,
        })
    }
}

pub fn normalize_relative_path(path: &Path) -> Result<String, IdentityError> {
    // `Path::components` deliberately follows the host platform. Canonical source
    // identities must not: a Windows-rooted path is unsafe on a Unix host too.
    let portable = path.to_string_lossy().replace('\\', "/");
    if portable.starts_with('/')
        || portable
            .as_bytes()
            .get(1)
            .is_some_and(|separator| *separator == b':')
            && portable
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphabetic)
    {
        return Err(IdentityError::NonRelativePath);
    }

    let mut components = Vec::new();
    for component in portable.split('/') {
        match component {
            "" | "." => {}
            ".." => return Err(IdentityError::NonRelativePath),
            component => components.push(component),
        }
    }
    if components.is_empty() {
        return Err(IdentityError::EmptyPath);
    }
    Ok(components.join("/"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct SourcePosition {
    pub line: u32,
    pub column: u32,
    pub byte: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub source: SourceIdentity,
    pub start: SourcePosition,
    pub end: SourcePosition,
}
impl SourceSpan {
    pub fn validate(&self) -> Result<(), IdentityError> {
        if self.start.byte > self.end.byte
            || (self.start.byte == self.end.byte
                && (self.start.line, self.start.column) > (self.end.line, self.end.column))
        {
            return Err(IdentityError::InvalidSpan);
        }
        if self.end.byte > self.source.byte_len {
            return Err(IdentityError::SpanOutsideSource);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Java,
    Kotlin,
    Go,
    C,
    Cpp,
    CSharp,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalyzerCapability {
    Syntax,
    Definitions,
    References,
    Calls,
    ControlFlow,
    DataFlow,
    TypeInfo,
    FrameworkSemantics,
    RuntimeTrace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactClass {
    Observed,
    Deterministic,
    Inferred,
    Reconstructed,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Confidence {
    pub structural: f32,
    pub semantic: f32,
    pub runtime: f32,
    pub overall: f32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvidenceRecord {
    pub id: EvidenceId,
    pub provider: String,
    pub provider_version: Option<String>,
    pub fact_class: FactClass,
    pub source_span: Option<SourceSpan>,
    pub inputs: Vec<EvidenceId>,
    pub claim_kind: String,
    pub unknown: bool,
}
impl EvidenceRecord {
    pub fn validate(&self) -> Result<(), EvidenceError> {
        if self.provider.is_empty() || self.claim_kind.is_empty() {
            return Err(EvidenceError::MissingProvenance);
        }
        if self.unknown && self.fact_class != FactClass::Unknown {
            return Err(EvidenceError::InvalidUnknown);
        }
        if !self.unknown && self.fact_class != FactClass::Unknown && self.source_span.is_none() {
            return Err(EvidenceError::MissingProvenance);
        }
        Ok(())
    }
}
#[derive(Debug, Error, Eq, PartialEq)]
pub enum EvidenceError {
    #[error("evidence claim is missing provenance")]
    MissingProvenance,
    #[error("unknown claims must have UNKNOWN fact class")]
    InvalidUnknown,
}
impl Confidence {
    pub fn new(structural: f32, semantic: f32, runtime: f32) -> Result<Self, IdentityError> {
        for value in [structural, semantic, runtime] {
            if !(0.0..=1.0).contains(&value) || !value.is_finite() {
                return Err(IdentityError::InvalidConfidence);
            }
        }
        Ok(Self {
            structural,
            semantic,
            runtime,
            overall: (structural + semantic + runtime) / 3.0,
        })
    }
    pub fn unknown() -> Self {
        Self {
            structural: 0.0,
            semantic: 0.0,
            runtime: 0.0,
            overall: 0.0,
        }
    }
    pub fn aggregate(values: &[Self]) -> Self {
        if values.is_empty() {
            return Self::unknown();
        }
        let n = values.len() as f32;
        Self::new(
            values.iter().map(|v| v.structural).sum::<f32>() / n,
            values.iter().map(|v| v.semantic).sum::<f32>() / n,
            values.iter().map(|v| v.runtime).sum::<f32>() / n,
        )
        .expect("means of valid confidence values remain valid")
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum IdentityError {
    #[error("paths must be non-empty and relative")]
    NonRelativePath,
    #[error("paths must not be empty")]
    EmptyPath,
    #[error("source span has an invalid ordering")]
    InvalidSpan,
    #[error("source span lies outside its source blob")]
    SpanOutsideSource,
    #[error("confidence dimensions must be finite values in [0, 1]")]
    InvalidConfidence,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn ids_are_deterministic_and_domain_separated() {
        assert_eq!(
            ProjectId::derive("p", &["a"]),
            ProjectId::derive("p", &["a"])
        );
        assert_ne!(
            ProjectId::derive("p", &["a"]),
            ProjectId::derive("q", &["a"])
        );
    }
    #[test]
    fn source_identity_normalizes_and_changes_with_content() {
        let first = SourceIdentity::from_bytes(Path::new("src/./lib.rs"), b"a").unwrap();
        let second = SourceIdentity::from_bytes(Path::new("src/lib.rs"), b"a").unwrap();
        let changed = SourceIdentity::from_bytes(Path::new("src/lib.rs"), b"b").unwrap();
        assert_eq!(first.normalized_path, second.normalized_path);
        assert_eq!(first.blob_id, second.blob_id);
        assert_ne!(first.blob_id, changed.blob_id);
    }
    #[test]
    fn absolute_and_parent_paths_are_rejected() {
        for path in [
            "",
            ".",
            "../secret",
            "src/../secret",
            "/secret",
            "\\secret",
            "C:/secret",
            "C:\\secret",
            "C:secret",
            "//server/share",
            "\\\\server\\share",
            "\\\\?\\C:\\secret",
        ] {
            assert!(normalize_relative_path(Path::new(path)).is_err(), "{path}");
        }
    }
    #[test]
    fn portable_relative_path_normalization_is_host_independent() {
        for (input, expected) in [
            ("src/lib.rs", "src/lib.rs"),
            ("src\\lib.rs", "src/lib.rs"),
            ("./src//nested/./lib.rs", "src/nested/lib.rs"),
        ] {
            assert_eq!(normalize_relative_path(Path::new(input)).unwrap(), expected);
        }
    }
    #[test]
    fn spans_and_confidence_validate() {
        let source = SourceIdentity::from_bytes(Path::new("a.rs"), b"abc").unwrap();
        let span = SourceSpan {
            source,
            start: SourcePosition {
                line: 1,
                column: 0,
                byte: 0,
            },
            end: SourcePosition {
                line: 1,
                column: 3,
                byte: 3,
            },
        };
        assert!(span.validate().is_ok());
        assert!(Confidence::new(1.1, 0.0, 0.0).is_err());
    }
    #[test]
    fn envelope_round_trips() {
        let json = serde_json::to_string(&SchemaEnvelope::current(ProjectId::derive("p", &["x"])))
            .unwrap();
        let _: SchemaEnvelope<ProjectId> = serde_json::from_str(&json).unwrap();
    }
    #[test]
    fn provenance_requires_source_or_explicit_unknown() {
        let source = SourceIdentity::from_bytes(Path::new("a.rs"), b"x").unwrap();
        let span = SourceSpan {
            source,
            start: SourcePosition {
                line: 1,
                column: 0,
                byte: 0,
            },
            end: SourcePosition {
                line: 1,
                column: 1,
                byte: 1,
            },
        };
        let known = EvidenceRecord {
            id: EvidenceId::derive("e", &["1"]),
            provider: "tree-sitter".into(),
            provider_version: None,
            fact_class: FactClass::Deterministic,
            source_span: Some(span),
            inputs: vec![],
            claim_kind: "syntax".into(),
            unknown: false,
        };
        assert!(known.validate().is_ok());
        let mut unknown = known;
        unknown.source_span = None;
        unknown.unknown = true;
        unknown.fact_class = FactClass::Unknown;
        assert!(unknown.validate().is_ok());
    }
    #[test]
    fn confidence_aggregation_is_deterministic_and_empty_is_unknown() {
        assert_eq!(Confidence::aggregate(&[]), Confidence::unknown());
        let value = Confidence::aggregate(&[
            Confidence::new(1.0, 0.0, 0.0).unwrap(),
            Confidence::new(0.0, 1.0, 1.0).unwrap(),
        ]);
        assert_eq!(value.overall, 0.5);
    }
}
