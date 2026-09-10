//! Safe, deterministic repository registration and snapshotting.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use codeflow_core::{Language, ProjectId, RevisionId, SourceIdentity};
use serde::{Deserialize, Serialize};
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    fn fixture() -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "codeflow-project-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
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
}
