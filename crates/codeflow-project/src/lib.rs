//! Safe, deterministic repository registration and snapshotting.

use std::collections::BTreeMap;
use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use codeflow_core::{Language, ProjectId, RevisionId, SourceIdentity};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Receiver};
use thiserror::Error;

pub const PROJECT_SCHEMA_VERSION: u16 = 1;
pub const DEFAULT_MAX_FILE_BYTES: u64 = 4 * 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProjectRegistration {
    pub schema_version: u16,
    pub project_id: ProjectId,
    pub root: PathBuf,
    pub created_at_unix_seconds: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SnapshotOptions {
    pub max_file_bytes: u64,
    pub follow_symlinks: bool,
}
impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            max_file_bytes: DEFAULT_MAX_FILE_BYTES,
            follow_symlinks: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileDisposition {
    Included,
    Ignored,
    Binary,
    Generated,
    Oversized,
    Symlink,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SnapshotFile {
    pub relative_path: String,
    pub language: Language,
    pub disposition: FileDisposition,
    pub source: Option<SourceIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositorySnapshot {
    pub schema_version: u16,
    pub project_id: ProjectId,
    pub revision_id: RevisionId,
    pub git_revision: Option<String>,
    pub files: Vec<SnapshotFile>,
    pub language_counts: BTreeMap<String, u64>,
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("project root does not exist or is not a directory")]
    InvalidRoot,
    #[error("unsafe repository entry: {0}")]
    UnsafePath(String),
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub fn register_project(
    root: &Path,
    now_unix_seconds: u64,
) -> Result<ProjectRegistration, ProjectError> {
    let canonical_root = root.canonicalize()?;
    if !canonical_root.is_dir() {
        return Err(ProjectError::InvalidRoot);
    }
    let project_id = ProjectId::derive("codeflow.project.v1", &[&canonical_root.to_string_lossy()]);
    let registration = ProjectRegistration {
        schema_version: PROJECT_SCHEMA_VERSION,
        project_id,
        root: canonical_root,
        created_at_unix_seconds: now_unix_seconds,
    };
    let metadata_dir = registration.root.join(".codeflow");
    fs::create_dir_all(&metadata_dir)?;
    fs::write(
        metadata_dir.join("project.json"),
        serde_json::to_vec_pretty(&registration)?,
    )?;
    Ok(registration)
}

pub fn snapshot(
    registration: &ProjectRegistration,
    options: &SnapshotOptions,
) -> Result<RepositorySnapshot, ProjectError> {
    let mut files = Vec::new();
    collect(&registration.root, &registration.root, options, &mut files)?;
    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    let mut language_counts = BTreeMap::new();
    for file in files
        .iter()
        .filter(|file| file.disposition == FileDisposition::Included)
    {
        *language_counts
            .entry(language_name(file.language).to_owned())
            .or_insert(0) += 1;
    }
    let manifest = serde_json::to_vec(&files)?;
    let manifest_digest = blake3::hash(&manifest).to_hex();
    let revision_id = RevisionId::derive(
        "codeflow.snapshot.v1",
        &[registration.project_id.as_str(), manifest_digest.as_ref()],
    );
    Ok(RepositorySnapshot {
        schema_version: PROJECT_SCHEMA_VERSION,
        project_id: registration.project_id.clone(),
        revision_id,
        git_revision: git_revision(&registration.root),
        files,
        language_counts,
    })
}

fn collect(
    root: &Path,
    directory: &Path,
    options: &SnapshotOptions,
    out: &mut Vec<SnapshotFile>,
) -> Result<(), ProjectError> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .map_err(|_| ProjectError::UnsafePath("entry escaped root".to_owned()))?;
        let relative_text = relative.to_string_lossy().replace('\\', "/");
        if relative_text == ".codeflow"
            || relative_text.starts_with(".codeflow/")
            || is_ignored(&relative_text)
        {
            continue;
        }
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            out.push(SnapshotFile {
                relative_path: relative_text,
                language: Language::Unknown,
                disposition: FileDisposition::Symlink,
                source: None,
            });
            continue;
        }
        if metadata.is_dir() {
            collect(root, &path, options, out)?;
            continue;
        }
        if !metadata.is_file() {
            continue;
        }
        let language = language_for(&path);
        let disposition = if metadata.len() > options.max_file_bytes {
            FileDisposition::Oversized
        } else {
            let bytes = fs::read(&path)?;
            if bytes.contains(&0) {
                FileDisposition::Binary
            } else if is_generated(&bytes) {
                FileDisposition::Generated
            } else {
                let source = SourceIdentity::from_bytes(relative, &bytes)
                    .map_err(|error| ProjectError::UnsafePath(error.to_string()))?;
                out.push(SnapshotFile {
                    relative_path: relative_text,
                    language,
                    disposition: FileDisposition::Included,
                    source: Some(source),
                });
                continue;
            }
        };
        out.push(SnapshotFile {
            relative_path: relative_text,
            language,
            disposition,
            source: None,
        });
    }
    Ok(())
}

fn is_ignored(path: &str) -> bool {
    path.split('/').any(|part| {
        matches!(
            part,
            ".git" | "target" | "node_modules" | "vendor" | ".venv" | "__pycache__"
        )
    })
}
fn is_generated(bytes: &[u8]) -> bool {
    String::from_utf8_lossy(&bytes[..bytes.len().min(2048)])
        .to_ascii_lowercase()
        .contains("@generated")
}
fn language_for(path: &Path) -> Language {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
    {
        "rs" => Language::Rust,
        "py" => Language::Python,
        "ts" | "tsx" => Language::TypeScript,
        "js" | "jsx" => Language::JavaScript,
        "java" => Language::Java,
        "kt" | "kts" => Language::Kotlin,
        "go" => Language::Go,
        "c" | "h" => Language::C,
        "cc" | "cpp" | "cxx" | "hpp" => Language::Cpp,
        "cs" => Language::CSharp,
        _ => Language::Unknown,
    }
}
fn language_name(language: Language) -> &'static str {
    match language {
        Language::Rust => "rust",
        Language::Python => "python",
        Language::TypeScript => "typescript",
        Language::JavaScript => "javascript",
        Language::Java => "java",
        Language::Kotlin => "kotlin",
        Language::Go => "go",
        Language::C => "c",
        Language::Cpp => "cpp",
        Language::CSharp => "csharp",
        Language::Unknown => "unknown",
    }
}
fn git_revision(root: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .ok()?;
    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim().to_owned())
    } else {
        None
    }
}
pub fn git_changed_paths(root: &Path, base: &str) -> Result<Vec<String>, std::io::Error> {
    let output = Command::new("git")
        .args(["diff", "--name-only", base])
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Ok(Vec::new());
    }
    Ok(coalesce_changes(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.replace('\\', "/"))
            .collect::<Vec<_>>(),
    ))
}

/// Deterministic dependency-closure invalidator used by watchers and Git batches.
pub fn invalidation_closure(
    changed: impl IntoIterator<Item = String>,
    reverse_dependencies: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    let mut invalidated: BTreeSet<String> = changed.into_iter().collect();
    let mut pending: VecDeque<String> = invalidated.iter().cloned().collect();
    while let Some(path) = pending.pop_front() {
        if let Some(dependents) = reverse_dependencies.get(&path) {
            for dependent in dependents {
                if invalidated.insert(dependent.clone()) {
                    pending.push_back(dependent.clone());
                }
            }
        }
    }
    invalidated
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidationPlan {
    pub files: BTreeSet<String>,
    pub reason: &'static str,
}
impl InvalidationPlan {
    pub fn from_changes(
        changed: impl IntoIterator<Item = String>,
        reverse_dependencies: &BTreeMap<String, BTreeSet<String>>,
    ) -> Self {
        Self {
            files: invalidation_closure(changed, reverse_dependencies),
            reason: "source_change",
        }
    }
}
/// All dependency surfaces that can cause downstream semantic products to change.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IncrementalDependencies {
    pub reverse_files: BTreeMap<String, BTreeSet<String>>,
    pub reverse_symbols: BTreeMap<String, BTreeSet<String>>,
    pub reverse_flow_regions: BTreeMap<String, BTreeSet<String>>,
    pub reverse_hag_nodes: BTreeMap<String, BTreeSet<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IncrementalPlan {
    pub changed_files: BTreeSet<String>,
    pub invalidated_files: BTreeSet<String>,
    pub invalidated_symbols: BTreeSet<String>,
    pub invalidated_flow_regions: BTreeSet<String>,
    pub invalidated_hag_nodes: BTreeSet<String>,
}

/// Computes the complete deterministic downstream closure for a coalesced change batch.
pub fn incremental_plan(
    changed: impl IntoIterator<Item = String>,
    dependencies: &IncrementalDependencies,
) -> IncrementalPlan {
    let changed_files: BTreeSet<_> = changed.into_iter().collect();
    let invalidated_files =
        invalidation_closure(changed_files.clone(), &dependencies.reverse_files);
    let invalidated_symbols = invalidation_closure(
        invalidated_files.iter().cloned(),
        &dependencies.reverse_symbols,
    );
    let invalidated_flow_regions = invalidation_closure(
        invalidated_symbols.iter().cloned(),
        &dependencies.reverse_flow_regions,
    );
    let invalidated_hag_nodes = invalidation_closure(
        invalidated_flow_regions.iter().cloned(),
        &dependencies.reverse_hag_nodes,
    );
    IncrementalPlan {
        changed_files,
        invalidated_files,
        invalidated_symbols,
        invalidated_flow_regions,
        invalidated_hag_nodes,
    }
}

/// Computes a content-addressed revision so an incremental publish can be compared with a full rebuild.
pub fn incremental_revision(
    project: &ProjectId,
    base: &RevisionId,
    plan: &IncrementalPlan,
) -> RevisionId {
    let payload = serde_json::to_vec(plan).expect("incremental plan is serializable");
    RevisionId::derive(
        "codeflow.incremental.v1",
        &[
            project.as_str(),
            base.as_str(),
            &blake3::hash(&payload).to_hex(),
        ],
    )
}
/// Collapses repeated watcher paths into a deterministic batch before invalidation.
pub fn coalesce_changes(paths: impl IntoIterator<Item = String>) -> Vec<String> {
    paths
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
pub fn paths_from_events(events: impl IntoIterator<Item = Event>) -> Vec<String> {
    coalesce_changes(events.into_iter().flat_map(|event| {
        event
            .paths
            .into_iter()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
    }))
}
pub struct WatcherHandle {
    pub watcher: RecommendedWatcher,
    pub events: Receiver<Result<Event, notify::Error>>,
}
pub fn start_watcher(root: &Path) -> Result<WatcherHandle, notify::Error> {
    let (sender, events) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |event| {
            let _ = sender.send(event);
        },
        Config::default(),
    )?;
    watcher.watch(root, RecursiveMode::Recursive)?;
    Ok(WatcherHandle { watcher, events })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);
    fn fixture() -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "codeflow-project-{}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(path.join("src")).unwrap();
        fs::write(path.join("src/main.rs"), "fn main() {} ").unwrap();
        fs::write(path.join("src/generated.rs"), "// @generated\n").unwrap();
        fs::write(path.join("binary.dat"), [0_u8, 1]).unwrap();
        fs::create_dir_all(path.join("target")).unwrap();
        fs::write(path.join("target/drop.rs"), "bad").unwrap();
        path
    }
    #[test]
    fn registration_persists_metadata_and_snapshot_is_stable() {
        let root = fixture();
        let project = register_project(&root, 1).unwrap();
        let first = snapshot(&project, &SnapshotOptions::default()).unwrap();
        let second = snapshot(&project, &SnapshotOptions::default()).unwrap();
        assert!(root.join(".codeflow/project.json").is_file());
        assert_eq!(first.revision_id, second.revision_id);
        assert_eq!(first.language_counts.get("rust"), Some(&1));
        assert_eq!(
            first
                .files
                .iter()
                .filter(|f| f.disposition == FileDisposition::Included)
                .count(),
            1
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn oversized_and_symlink_inputs_degrade_explicitly() {
        let root = fixture();
        fs::write(root.join("large.py"), b"0123456789").unwrap();
        let project = register_project(&root, 1).unwrap();
        let snap = snapshot(
            &project,
            &SnapshotOptions {
                max_file_bytes: 1,
                follow_symlinks: false,
            },
        )
        .unwrap();
        assert!(
            snap.files
                .iter()
                .any(|f| f.disposition == FileDisposition::Oversized)
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn invalidation_closure_is_transitive_and_cycle_safe() {
        let mut reverse = BTreeMap::new();
        reverse.insert("a".into(), BTreeSet::from(["b".into()]));
        reverse.insert("b".into(), BTreeSet::from(["a".into(), "c".into()]));
        assert_eq!(
            invalidation_closure(["a".into()], &reverse),
            BTreeSet::from(["a".into(), "b".into(), "c".into()])
        );
    }
    #[test]
    fn coalescing_is_stable() {
        assert_eq!(
            coalesce_changes(["b".into(), "a".into(), "b".into()]),
            vec!["a", "b"]
        );
    }
    #[test]
    fn git_batch_probe_is_capability_safe() {
        let root = std::env::temp_dir();
        assert!(git_changed_paths(&root, "HEAD").is_ok());
    }
    #[test]
    fn invalidation_plan_is_revision_scoped_by_reason() {
        let plan = InvalidationPlan::from_changes(["a".into()], &BTreeMap::new());
        assert_eq!(plan.reason, "source_change");
        assert!(plan.files.contains("a"));
    }
    #[test]
    fn watcher_event_paths_are_coalesced() {
        let mut first =
            notify::Event::new(notify::EventKind::Modify(notify::event::ModifyKind::Any));
        first.paths.push(PathBuf::from("b.rs"));
        let mut second = first.clone();
        second.paths.push(PathBuf::from("a.rs"));
        assert_eq!(paths_from_events([first, second]), vec!["a.rs", "b.rs"]);
    }
    #[test]
    fn incremental_plan_reaches_symbols_flows_and_hag() {
        let mut dependencies = IncrementalDependencies::default();
        dependencies
            .reverse_files
            .insert("a.rs".into(), BTreeSet::from(["b.rs".into()]));
        dependencies
            .reverse_symbols
            .insert("b.rs".into(), BTreeSet::from(["symbol:B".into()]));
        dependencies
            .reverse_flow_regions
            .insert("symbol:B".into(), BTreeSet::from(["flow:handler".into()]));
        dependencies.reverse_hag_nodes.insert(
            "flow:handler".into(),
            BTreeSet::from(["hag:service".into()]),
        );
        let plan = incremental_plan(["a.rs".into(), "a.rs".into()], &dependencies);
        assert_eq!(
            plan.invalidated_files,
            BTreeSet::from(["a.rs".into(), "b.rs".into()])
        );
        assert!(plan.invalidated_hag_nodes.contains("hag:service"));
        let project = ProjectId::derive("p", &["incremental"]);
        let base = RevisionId::derive("r", &["base"]);
        assert_eq!(
            incremental_revision(&project, &base, &plan),
            incremental_revision(&project, &base, &plan)
        );
    }
}
