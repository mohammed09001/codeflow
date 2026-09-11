//! Resilient parser-provider fabric. Tree-sitter node identities never leave this crate.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::process::Command;
use std::time::{Duration, Instant};

use codeflow_core::{AnalyzerCapability, Language, SourceIdentity, SourcePosition, SourceSpan};
use codeflow_core::{EvidenceId, FactClass};
use codeflow_upsm::{EvidenceRef, NodeKind, UpsmGraph};
use protobuf::Message;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tree_sitter::{InputEdit, Node, Parser, Point, Tree};

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
    #[error("no compatible previous tree is available for this incremental edit")]
    PreviousTreeUnavailable,
    #[error("the incremental edit does not describe valid source byte ranges")]
    InvalidEdit,
}

/// Byte offsets use the source before and after an edit respectively.  Points are
/// derived internally so callers cannot accidentally pass character columns to
/// Tree-sitter, which requires byte columns.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextEdit {
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub new_end_byte: usize,
}

struct ParseSession {
    language: Language,
    tree: Tree,
}

pub struct TreeSitterProvider {
    parser: Parser,
    cache: HashMap<codeflow_core::SourceBlobId, SyntaxEvidence>,
    sessions: HashMap<codeflow_core::SourceBlobId, ParseSession>,
    #[cfg(test)]
    parse_reused_edited_tree: Vec<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScipSymbolEvidence {
    /// Provider-local SCIP symbol. It is evidence only and never a CodeFlow ID.
    pub provider_symbol: String,
    pub display_name: String,
    pub path: String,
    pub definition: bool,
    pub reference: bool,
}

#[derive(Debug, Error)]
pub enum ScipError {
    #[error("invalid SCIP protobuf: {0}")]
    Decode(String),
    #[error("SCIP document path is unsafe")]
    UnsafePath,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScipIndexStatus {
    Usable,
    Partial,
    Stale,
}

/// Classifies a decoded SCIP index against the current snapshot paths without
/// pretending that missing provider coverage is a semantic negative fact.
pub fn assess_scip_index(indexed_paths: &[String], snapshot_paths: &[String]) -> ScipIndexStatus {
    if indexed_paths.is_empty() {
        return ScipIndexStatus::Partial;
    }
    let indexed: std::collections::BTreeSet<_> = indexed_paths.iter().collect();
    let snapshot: std::collections::BTreeSet<_> = snapshot_paths.iter().collect();
    if indexed.iter().any(|path| !snapshot.contains(path)) {
        ScipIndexStatus::Stale
    } else if snapshot.iter().any(|path| !indexed.contains(path)) {
        ScipIndexStatus::Partial
    } else {
        ScipIndexStatus::Usable
    }
}

/// Capability probe; absence is a normal condition and does not fail analysis.
pub fn probe_scip_indexer(executable: &str) -> bool {
    Command::new(executable)
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}
pub fn identifier_tokens(identifier: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in identifier.split(|c: char| !c.is_alphanumeric()) {
        if !part.is_empty() {
            let mut current = String::new();
            let chars: Vec<char> = part.chars().collect();
            for (index, ch) in chars.iter().copied().enumerate() {
                let acronym_boundary = ch.is_uppercase()
                    && index > 0
                    && current.len() > 1
                    && current.chars().all(|value| value.is_uppercase())
                    && chars.get(index + 1).is_some_and(|next| next.is_lowercase());
                if index > 0
                    && ch.is_uppercase()
                    && (!current.is_empty()
                        && (!current.chars().all(|value| value.is_uppercase()) || acronym_boundary))
                {
                    out.push(current.to_lowercase());
                    current.clear();
                }
                current.push(ch);
            }
            if !current.is_empty() {
                out.push(current.to_lowercase());
            }
        }
    }
    out.into_iter()
        .filter(|token| {
            !matches!(
                token.as_str(),
                "get" | "set" | "the" | "and" | "impl" | "fn" | "class"
            )
        })
        .collect()
}
pub fn lexical_signal(identifier: &str, vocabulary: &[&str]) -> f32 {
    let tokens = identifier_tokens(identifier);
    if tokens.is_empty() {
        return 0.0;
    }
    tokens
        .iter()
        .filter(|token| vocabulary.iter().any(|word| token == word))
        .count() as f32
        / tokens.len() as f32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LexicalVector {
    pub entity: String,
    pub weights: BTreeMap<String, f32>,
    pub fact_class: FactClass,
}

pub fn tf_idf_vectors(documents: &BTreeMap<String, String>) -> Vec<LexicalVector> {
    let token_docs: BTreeMap<_, _> = documents
        .iter()
        .map(|(id, text)| (id.clone(), identifier_tokens(text)))
        .collect();
    let total = token_docs.len() as f32;
    let mut document_frequency = BTreeMap::<String, usize>::new();
    for tokens in token_docs.values() {
        for token in tokens.iter().collect::<BTreeSet<_>>() {
            *document_frequency.entry(token.clone()).or_default() += 1;
        }
    }
    token_docs
        .into_iter()
        .map(|(entity, tokens)| {
            let mut counts = BTreeMap::<String, usize>::new();
            for token in tokens {
                *counts.entry(token).or_default() += 1;
            }
            let length = counts.values().sum::<usize>().max(1) as f32;
            let weights = counts
                .into_iter()
                .map(|(token, count)| {
                    let df = document_frequency[&token] as f32;
                    (
                        token,
                        (count as f32 / length) * ((total + 1.0) / (df + 1.0)).ln() + 1.0,
                    )
                })
                .collect();
            LexicalVector {
                entity,
                weights,
                fact_class: FactClass::Deterministic,
            }
        })
        .collect()
}

pub fn bm25_score(
    query: &str,
    document: &str,
    corpus_size: usize,
    document_frequency: usize,
) -> f32 {
    let query_tokens = identifier_tokens(query);
    let doc_tokens = identifier_tokens(document);
    let mut score = 0.0;
    for token in query_tokens {
        let tf = doc_tokens
            .iter()
            .filter(|candidate| *candidate == &token)
            .count() as f32;
        if tf > 0.0 {
            score += ((corpus_size as f32 + 1.0) / (document_frequency as f32 + 1.0)).ln()
                * (tf / (tf + 1.2));
        }
    }
    score
}

pub fn route_signal(route: &str, vocabulary: &[&str]) -> f32 {
    lexical_signal(route, vocabulary)
}

pub fn neighborhood_enriched_signal(
    identifier: &str,
    neighbors: &[&str],
    vocabulary: &[&str],
) -> f32 {
    let mut total = lexical_signal(identifier, vocabulary);
    if !neighbors.is_empty() {
        total = (total
            + neighbors
                .iter()
                .map(|item| lexical_signal(item, vocabulary))
                .sum::<f32>()
                / neighbors.len() as f32)
            / 2.0;
    }
    total
}

pub fn label_candidates(labels: &[&str], vocabulary: &[&str]) -> Vec<(String, f32)> {
    let mut result: Vec<_> = labels
        .iter()
        .map(|label| ((*label).to_owned(), lexical_signal(label, vocabulary)))
        .collect();
    result.sort_by(|left, right| {
        right
            .1
            .total_cmp(&left.1)
            .then_with(|| left.0.cmp(&right.0))
    });
    result
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderAvailability {
    pub available: bool,
    pub version: Option<String>,
}

#[derive(Debug, Error)]
pub enum JoernError {
    #[error("Joern is unavailable")]
    Unavailable,
    #[error("Joern process timed out")]
    TimedOut,
    #[error("Joern process failed")]
    Failed,
    #[error("Joern process I/O failure: {0}")]
    Io(String),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpgEvidence {
    pub node_kind: String,
    pub provider_node_id: String,
    pub edge_kind: Option<String>,
    pub provider_target_id: Option<String>,
}
#[derive(Debug, Error)]
pub enum CpgImportError {
    #[error("invalid Joern export: {0}")]
    Invalid(String),
}
/// Decodes a deliberately small provider-local JSON projection; provider IDs never
/// cross this adapter as canonical CodeFlow identities.
pub fn import_joern_cpg_json(bytes: &[u8]) -> Result<Vec<CpgEvidence>, CpgImportError> {
    let mut records: Vec<CpgEvidence> = serde_json::from_slice(bytes)
        .map_err(|error| CpgImportError::Invalid(error.to_string()))?;
    records.sort_by(|a, b| {
        (&a.provider_node_id, &a.edge_kind, &a.provider_target_id).cmp(&(
            &b.provider_node_id,
            &b.edge_kind,
            &b.provider_target_id,
        ))
    });
    Ok(records)
}

pub fn probe_joern(executable: &str) -> ProviderAvailability {
    match Command::new(executable).arg("--version").output() {
        Ok(output) if output.status.success() => ProviderAvailability {
            available: true,
            version: String::from_utf8(output.stdout)
                .ok()
                .map(|value| value.trim().to_owned()),
        },
        _ => ProviderAvailability {
            available: false,
            version: None,
        },
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameworkSemantic {
    pub framework: String,
    pub kind: String,
    pub route: Option<String>,
    pub confidence: f32,
    pub provider: String,
    pub fact_class: FactClass,
    pub evidence_id: EvidenceId,
}
pub trait FrameworkAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, language: Language, source: &str) -> Vec<FrameworkSemantic>;
}
pub struct DeterministicWebAdapter;
impl FrameworkAdapter for DeterministicWebAdapter {
    fn name(&self) -> &'static str {
        "deterministic-web"
    }
    fn detect(&self, language: Language, source: &str) -> Vec<FrameworkSemantic> {
        let (framework, markers): (Option<&str>, &[&str]) = match language {
            Language::Rust => (Some("rust-web"), &["#[get(", "#[post(", "Router::new"]),
            Language::Python => (
                Some("python-web"),
                &["@app.route", "@router.get", "@app.get"],
            ),
            Language::TypeScript | Language::JavaScript => {
                (Some("js-web"), &["app.get(", "router.get(", "app.post("])
            }
            Language::Java => (
                Some("spring"),
                &["@GetMapping", "@PostMapping", "@RequestMapping"],
            ),
            Language::Go => (Some("go-http"), &["http.HandleFunc", "router.GET("]),
            Language::CSharp => (Some("aspnet"), &["MapGet(", "[HttpGet"]),
            _ => (None, &[]),
        };
        framework
            .into_iter()
            .flat_map(|name| {
                markers
                    .iter()
                    .filter(move |marker| source.contains(**marker))
                    .map(move |marker| FrameworkSemantic {
                        framework: name.into(),
                        kind: "http_route".into(),
                        route: Some((*marker).into()),
                        confidence: 1.0,
                        provider: "deterministic-web".into(),
                        fact_class: FactClass::Deterministic,
                        evidence_id: EvidenceId::derive("codeflow.framework.v1", &[name, *marker]),
                    })
            })
            .collect()
    }
}

/// Runs a bounded external CPG export without exposing source content in errors.
pub fn run_joern_export(
    executable: &str,
    args: &[String],
    timeout: Duration,
) -> Result<Vec<u8>, JoernError> {
    if !probe_joern(executable).available {
        return Err(JoernError::Unavailable);
    }
    let mut child = Command::new(executable)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|error| JoernError::Io(error.kind().to_string()))?;
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| JoernError::Io(error.kind().to_string()))?
        {
            if !status.success() {
                return Err(JoernError::Failed);
            }
            return child
                .wait_with_output()
                .map(|output| output.stdout)
                .map_err(|error| JoernError::Io(error.kind().to_string()));
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(JoernError::TimedOut);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Decodes the official SCIP protobuf while retaining provider symbols only as evidence.
pub fn import_scip(bytes: &[u8]) -> Result<Vec<ScipSymbolEvidence>, ScipError> {
    let index = scip::types::Index::parse_from_bytes(bytes)
        .map_err(|error| ScipError::Decode(error.to_string()))?;
    let mut evidence = Vec::new();
    for document in index.documents {
        if document.relative_path.is_empty()
            || document.relative_path.split('/').any(|part| part == "..")
        {
            return Err(ScipError::UnsafePath);
        }
        for occurrence in document.occurrences {
            if occurrence.symbol.is_empty() {
                continue;
            }
            let definition = occurrence.symbol_roles & 1 != 0;
            evidence.push(ScipSymbolEvidence {
                provider_symbol: occurrence.symbol.clone(),
                display_name: occurrence.symbol,
                path: document.relative_path.clone(),
                definition,
                reference: !definition,
            });
        }
    }
    evidence.sort_by(|left, right| {
        (&left.path, &left.provider_symbol, left.definition).cmp(&(
            &right.path,
            &right.provider_symbol,
            right.definition,
        ))
    });
    evidence.dedup();
    Ok(evidence)
}

/// Merges decoded SCIP occurrences through the canonical UPSM boundary. SCIP's
/// symbol string is retained only in EvidenceRef.provider_key.
pub fn merge_scip_into_upsm(graph: &mut UpsmGraph, records: Vec<ScipSymbolEvidence>) {
    for record in records {
        let mut properties = std::collections::BTreeMap::new();
        properties.insert("path".to_owned(), record.path.clone());
        properties.insert(
            "symbol_role".to_owned(),
            if record.definition {
                "definition"
            } else {
                "reference"
            }
            .to_owned(),
        );
        let name = record
            .display_name
            .rsplit(['/', '#', '.'])
            .next()
            .filter(|name| !name.is_empty())
            .unwrap_or("unknown");
        let qualified_name = format!("{}::{name}", record.path);
        let evidence = EvidenceRef {
            id: EvidenceId::derive(
                "codeflow.scip.evidence.v1",
                &[&record.path, &record.provider_symbol],
            ),
            provider: "scip".to_owned(),
            provider_key: record.provider_symbol,
            fact_class: FactClass::Deterministic,
            span: None,
        };
        graph.merge_node(
            NodeKind::Unknown,
            &qualified_name,
            None,
            properties,
            evidence,
        );
    }
}
impl TreeSitterProvider {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            cache: HashMap::new(),
            sessions: HashMap::new(),
            #[cfg(test)]
            parse_reused_edited_tree: Vec::new(),
        }
    }

    /// Reparses `bytes` by editing the tree retained for `previous_source`.
    /// The returned evidence is normalized exactly as a clean parse would be;
    /// Tree-sitter node identities never leave this provider.
    pub fn parse_incremental(
        &mut self,
        previous_source: &SourceIdentity,
        source: SourceIdentity,
        language: Language,
        previous_bytes: &[u8],
        bytes: &[u8],
        edit: TextEdit,
    ) -> Result<SyntaxEvidence, ParseError> {
        if edit.start_byte > edit.old_end_byte
            || edit.old_end_byte > previous_bytes.len()
            || edit.start_byte > edit.new_end_byte
            || edit.new_end_byte > bytes.len()
        {
            return Err(ParseError::InvalidEdit);
        }
        if let Some(cached) = self.cache.get(&source.blob_id) {
            return Ok(cached.clone());
        }
        let mut edited_tree = self
            .sessions
            .get(&previous_source.blob_id)
            .filter(|session| session.language == language)
            .map(|session| session.tree.clone())
            .ok_or(ParseError::PreviousTreeUnavailable)?;
        self.configure(language)?;
        edited_tree.edit(&InputEdit {
            start_byte: edit.start_byte,
            old_end_byte: edit.old_end_byte,
            new_end_byte: edit.new_end_byte,
            start_position: point_at(previous_bytes, edit.start_byte)
                .ok_or(ParseError::InvalidEdit)?,
            old_end_position: point_at(previous_bytes, edit.old_end_byte)
                .ok_or(ParseError::InvalidEdit)?,
            new_end_position: point_at(bytes, edit.new_end_byte).ok_or(ParseError::InvalidEdit)?,
        });
        let tree = self
            .parser
            .parse(bytes, Some(&edited_tree))
            .ok_or(ParseError::Cancelled)?;
        #[cfg(test)]
        self.parse_reused_edited_tree.push(true);
        Ok(self.record_parse(source, language, tree))
    }

    fn configure(&mut self, language: Language) -> Result<(), ParseError> {
        let grammar = grammar_for(language).ok_or(ParseError::Unsupported(language))?;
        self.parser
            .set_language(&grammar)
            .map_err(|_| ParseError::Initialization)
    }

    fn record_parse(
        &mut self,
        source: SourceIdentity,
        language: Language,
        tree: Tree,
    ) -> SyntaxEvidence {
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
        self.sessions
            .insert(source.blob_id, ParseSession { language, tree });
        evidence
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
        self.configure(language)?;
        let tree = self
            .parser
            .parse(bytes, None)
            .ok_or(ParseError::Cancelled)?;
        #[cfg(test)]
        self.parse_reused_edited_tree.push(false);
        Ok(self.record_parse(source, language, tree))
    }
}

fn grammar_for(language: Language) -> Option<tree_sitter::Language> {
    Some(match language {
        Language::Rust => tree_sitter_rust::LANGUAGE.into(),
        Language::Python => tree_sitter_python::LANGUAGE.into(),
        Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        Language::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
        Language::Java => tree_sitter_java::LANGUAGE.into(),
        Language::Kotlin => tree_sitter_kotlin_ng::LANGUAGE.into(),
        Language::Go => tree_sitter_go::LANGUAGE.into(),
        Language::C => tree_sitter_c::LANGUAGE.into(),
        Language::Cpp => tree_sitter_cpp::LANGUAGE.into(),
        Language::CSharp => tree_sitter_c_sharp::LANGUAGE.into(),
        Language::Unknown => return None,
    })
}

fn point_at(bytes: &[u8], offset: usize) -> Option<Point> {
    (offset <= bytes.len()).then(|| {
        let prefix = &bytes[..offset];
        let row = prefix.iter().filter(|byte| **byte == b'\n').count();
        let column = prefix
            .iter()
            .rposition(|byte| *byte == b'\n')
            .map_or(prefix.len(), |last_newline| prefix.len() - last_newline - 1);
        Point { row, column }
    })
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
    fn unknown_language_is_explicit() {
        let bytes = b"x";
        let mut provider = TreeSitterProvider::new();
        assert!(matches!(
            provider.parse(source(bytes), Language::Unknown, bytes),
            Err(ParseError::Unsupported(Language::Unknown))
        ));
    }

    #[test]
    fn every_declared_language_has_a_production_grammar_and_recovers() {
        let fixtures = [
            (
                Language::Rust,
                b"fn main() {}".as_slice(),
                b"fn {".as_slice(),
            ),
            (
                Language::Python,
                b"def main():\n    pass\n".as_slice(),
                b"def (".as_slice(),
            ),
            (
                Language::TypeScript,
                b"const x: number = 1;".as_slice(),
                b"const = ;".as_slice(),
            ),
            (
                Language::JavaScript,
                b"const x = 1;".as_slice(),
                b"const = ;".as_slice(),
            ),
            (
                Language::Java,
                b"class Main {}".as_slice(),
                b"class {".as_slice(),
            ),
            (
                Language::Kotlin,
                b"fun main() {}".as_slice(),
                b"fun {".as_slice(),
            ),
            (
                Language::Go,
                b"package main\nfunc main() {}".as_slice(),
                b"func {".as_slice(),
            ),
            (
                Language::C,
                b"int main(void) { return 0; }".as_slice(),
                b"int {".as_slice(),
            ),
            (
                Language::Cpp,
                b"int main() { return 0; }".as_slice(),
                b"int {".as_slice(),
            ),
            (
                Language::CSharp,
                b"class Main {}".as_slice(),
                b"class {".as_slice(),
            ),
        ];
        let mut provider = TreeSitterProvider::new();
        for (language, valid, malformed) in fixtures {
            let valid_evidence = provider
                .parse(source(valid), language, valid)
                .unwrap_or_else(|error| panic!("{language:?} valid fixture: {error}"));
            assert!(!valid_evidence.nodes.is_empty(), "{language:?}");
            let malformed_evidence = provider
                .parse(source(malformed), language, malformed)
                .unwrap_or_else(|error| panic!("{language:?} malformed fixture: {error}"));
            assert!(!malformed_evidence.nodes.is_empty(), "{language:?}");
            assert!(!malformed_evidence.errors.is_empty(), "{language:?}");
        }
    }

    #[test]
    fn incremental_parse_reuses_edited_tree_and_matches_clean_parse() {
        let before = b"def hello():\n    return 1\n";
        let after = b"def hello():\n    return 42\n";
        let before_source = source(before);
        let after_source = source(after);
        let mut incremental = TreeSitterProvider::new();
        incremental
            .parse(before_source.clone(), Language::Python, before)
            .unwrap();
        let incremental_evidence = incremental
            .parse_incremental(
                &before_source,
                after_source.clone(),
                Language::Python,
                before,
                after,
                TextEdit {
                    start_byte: before.len() - 2,
                    old_end_byte: before.len() - 1,
                    new_end_byte: after.len() - 1,
                },
            )
            .unwrap();
        let mut clean = TreeSitterProvider::new();
        let clean_evidence = clean.parse(after_source, Language::Python, after).unwrap();
        assert_eq!(incremental_evidence, clean_evidence);
        assert_eq!(incremental.parse_reused_edited_tree, vec![false, true]);
        // Negative control: equivalent evidence from a clean parse is not proof
        // that an edited tree was reused.
        assert_eq!(clean.parse_reused_edited_tree, vec![false]);
    }

    #[test]
    fn scip_protobuf_import_is_deterministic_and_rejects_bad_paths() {
        let mut index = scip::types::Index::new();
        let mut document = scip::types::Document::new();
        document.relative_path = "src/lib.rs".to_owned();
        let mut occurrence = scip::types::Occurrence::new();
        occurrence.symbol = "rust . hello().".to_owned();
        occurrence.symbol_roles = 1;
        document.occurrences.push(occurrence);
        index.documents.push(document);
        let bytes = index.write_to_bytes().unwrap();
        let imported = import_scip(&bytes).unwrap();
        assert_eq!(imported.len(), 1);
        assert!(imported[0].definition);
        let mut unsafe_index = scip::types::Index::new();
        let mut unsafe_document = scip::types::Document::new();
        unsafe_document.relative_path = "../outside.rs".to_owned();
        unsafe_index.documents.push(unsafe_document);
        assert!(matches!(
            import_scip(&unsafe_index.write_to_bytes().unwrap()),
            Err(ScipError::UnsafePath)
        ));
    }
    #[test]
    fn scip_conversion_keeps_provider_symbols_out_of_canonical_ids() {
        let mut graph = UpsmGraph::new();
        merge_scip_into_upsm(
            &mut graph,
            vec![ScipSymbolEvidence {
                provider_symbol: "scip-local".into(),
                display_name: "hello".into(),
                path: "src/lib.rs".into(),
                definition: true,
                reference: false,
            }],
        );
        let node = graph.nodes.values().next().unwrap();
        assert!(!node.id.as_str().contains("scip-local"));
        assert_eq!(node.evidence[0].provider_key, "scip-local");
    }

    #[test]
    fn missing_scip_indexer_is_a_capability_absence() {
        assert!(!probe_scip_indexer(
            "codeflow-test-indexer-that-does-not-exist"
        ));
    }

    #[test]
    fn scip_staleness_never_promotes_partial_coverage_to_complete() {
        let snapshot = vec!["a.rs".to_owned(), "b.rs".to_owned()];
        assert_eq!(
            assess_scip_index(&["a.rs".to_owned()], &snapshot),
            ScipIndexStatus::Partial
        );
        assert_eq!(
            assess_scip_index(&["gone.rs".to_owned()], &snapshot),
            ScipIndexStatus::Stale
        );
    }

    #[test]
    fn missing_joern_is_an_explicit_capability_fallback() {
        assert!(!probe_joern("codeflow-test-joern-that-does-not-exist").available);
        assert!(matches!(
            run_joern_export(
                "codeflow-test-joern-that-does-not-exist",
                &[],
                Duration::from_millis(1)
            ),
            Err(JoernError::Unavailable)
        ));
    }
    #[test]
    fn joern_cpg_import_preserves_provider_identity_as_evidence_only() {
        let records=import_joern_cpg_json(br#"[{"node_kind":"CALL","provider_node_id":"9","edge_kind":"CALL","provider_target_id":"10"}]"#).unwrap();
        assert_eq!(records[0].provider_node_id, "9");
        assert_eq!(records[0].edge_kind.as_deref(), Some("CALL"));
    }
    #[test]
    fn framework_adapter_is_deterministic_and_unknown_safe() {
        let adapter = DeterministicWebAdapter;
        let rust = adapter.detect(Language::Rust, "#[get(\"/health\")] fn health() {} ");
        assert_eq!(rust.len(), 1);
        assert_eq!(rust[0].fact_class, FactClass::Deterministic);
        assert_eq!(rust[0].provider, "deterministic-web");
        assert!(!rust[0].evidence_id.as_str().is_empty());
        assert_eq!(
            adapter.detect(Language::Python, "@app.route('/x')").len(),
            1
        );
        assert_eq!(adapter.detect(Language::Java, "@GetMapping('/x')").len(), 1);
        assert_eq!(
            adapter
                .detect(Language::Go, "http.HandleFunc('/x', h)")
                .len(),
            1
        );
        assert_eq!(
            adapter
                .detect(Language::CSharp, "app.MapGet('/x', h)")
                .len(),
            1
        );
        assert!(adapter.detect(Language::Unknown, "anything").is_empty());
    }
    #[test]
    fn identifier_signals_are_style_and_noise_robust() {
        assert_eq!(
            identifier_tokens("HTTPResponse_getValue"),
            vec!["http", "response", "value"]
        );
        assert_eq!(lexical_signal("checkoutOrder", &["checkout", "order"]), 1.0);
        let docs = BTreeMap::from([
            (String::from("a"), String::from("checkoutOrder")),
            (String::from("b"), String::from("billing")),
        ]);
        let vectors = tf_idf_vectors(&docs);
        assert_eq!(vectors.len(), 2);
        assert!(bm25_score("checkout", "checkoutOrder", 2, 1) > 0.0);
        assert!(neighborhood_enriched_signal("checkout", &["order"], &["checkout", "order"]) > 0.5);
        assert_eq!(
            label_candidates(&["billing", "checkout"], &["checkout"])[0].0,
            "checkout"
        );
    }
}
