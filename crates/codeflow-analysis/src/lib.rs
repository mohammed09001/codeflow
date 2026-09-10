//! Resilient parser-provider fabric. Tree-sitter node identities never leave this crate.

use std::collections::HashMap;

use codeflow_core::{AnalyzerCapability, Language, SourceIdentity, SourcePosition, SourceSpan};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tree_sitter::{Node, Parser};

pub const SYNTAX_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SyntaxEvidence {
    pub schema_version: u16,
    pub provider: String,
    pub source: SourceIdentity,
    pub language: Language,
    pub nodes: Vec<SyntaxNodeEvidence>,
    pub errors: Vec<SyntaxErrorEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SyntaxNodeEvidence {
    pub kind: String,
    pub span: SourceSpan,
    pub named: bool,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SyntaxErrorEvidence {
    pub span: SourceSpan,
    pub recovered: bool,
}

pub trait ParserProvider: Send {
    fn name(&self) -> &'static str;
    fn capabilities(&self) -> &'static [AnalyzerCapability];
    fn parse(
        &mut self,
        source: SourceIdentity,
        language: Language,
        bytes: &[u8],
    ) -> Result<SyntaxEvidence, ParseError>;
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("no parser grammar is available for {0:?}")]
    Unsupported(Language),
    #[error("tree-sitter parser initialization failed")]
    Initialization,
    #[error("tree-sitter cancelled parse")]
    Cancelled,
}

pub struct TreeSitterProvider {
    parser: Parser,
    cache: HashMap<codeflow_core::SourceBlobId, SyntaxEvidence>,
}
impl TreeSitterProvider {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            cache: HashMap::new(),
        }
    }
}
impl Default for TreeSitterProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserProvider for TreeSitterProvider {
    fn name(&self) -> &'static str {
        "tree-sitter"
    }
    fn capabilities(&self) -> &'static [AnalyzerCapability] {
        &[AnalyzerCapability::Syntax]
    }
    fn parse(
        &mut self,
        source: SourceIdentity,
        language: Language,
        bytes: &[u8],
    ) -> Result<SyntaxEvidence, ParseError> {
        if let Some(cached) = self.cache.get(&source.blob_id) {
            return Ok(cached.clone());
        }
        let grammar = match language {
            Language::Rust => tree_sitter_rust::LANGUAGE.into(),
            _ => return Err(ParseError::Unsupported(language)),
        };
        self.parser
            .set_language(&grammar)
            .map_err(|_| ParseError::Initialization)?;
        let tree = self
            .parser
            .parse(bytes, None)
            .ok_or(ParseError::Cancelled)?;
        let mut nodes = Vec::new();
        let mut errors = Vec::new();
        collect(tree.root_node(), &source, &mut nodes, &mut errors);
        nodes.sort_by(|a, b| (a.span.start.byte, &a.kind).cmp(&(b.span.start.byte, &b.kind)));
        errors.sort_by_key(|error| error.span.start.byte);
        let evidence = SyntaxEvidence {
            schema_version: SYNTAX_SCHEMA_VERSION,
            provider: self.name().to_owned(),
            source: source.clone(),
            language,
            nodes,
            errors,
        };
        self.cache.insert(source.blob_id.clone(), evidence.clone());
        Ok(evidence)
    }
}

fn collect(
    node: Node<'_>,
    source: &SourceIdentity,
    nodes: &mut Vec<SyntaxNodeEvidence>,
    errors: &mut Vec<SyntaxErrorEvidence>,
) {
    let span = span_for(node, source);
    if node.is_named() {
        nodes.push(SyntaxNodeEvidence {
            kind: node.kind().to_owned(),
            span: span.clone(),
            named: true,
        });
    }
    if node.is_error() || node.is_missing() {
        errors.push(SyntaxErrorEvidence {
            span,
            recovered: true,
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect(child, source, nodes, errors);
    }
}
fn span_for(node: Node<'_>, source: &SourceIdentity) -> SourceSpan {
    let start = node.start_position();
    let end = node.end_position();
    SourceSpan {
        source: source.clone(),
        start: SourcePosition {
            line: start.row as u32 + 1,
            column: start.column as u32,
            byte: node.start_byte() as u64,
        },
        end: SourcePosition {
            line: end.row as u32 + 1,
            column: end.column as u32,
            byte: node.end_byte() as u64,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    fn source(bytes: &[u8]) -> SourceIdentity {
        SourceIdentity::from_bytes(Path::new("src/lib.rs"), bytes).unwrap()
    }
    #[test]
    fn rust_parse_produces_normalized_evidence_without_parser_ids() {
        let bytes = b"pub fn hello() {}";
        let mut provider = TreeSitterProvider::new();
        let evidence = provider
            .parse(source(bytes), Language::Rust, bytes)
            .unwrap();
        assert!(
            evidence
                .nodes
                .iter()
                .any(|node| node.kind == "function_item")
        );
        assert!(evidence.errors.is_empty());
        assert_eq!(evidence.provider, "tree-sitter");
    }
    #[test]
    fn syntax_errors_are_recovered_as_evidence() {
        let bytes = b"fn {";
        let mut provider = TreeSitterProvider::new();
        let evidence = provider
            .parse(source(bytes), Language::Rust, bytes)
            .unwrap();
        assert!(!evidence.errors.is_empty());
    }
    #[test]
    fn unsupported_language_is_explicit() {
        let bytes = b"x";
        let mut provider = TreeSitterProvider::new();
        assert!(matches!(
            provider.parse(source(bytes), Language::Python, bytes),
            Err(ParseError::Unsupported(Language::Python))
        ));
    }
}
