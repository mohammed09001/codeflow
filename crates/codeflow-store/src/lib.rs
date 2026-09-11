//! Transactional local graph metadata store; SQLite WAL is the default backend.
use codeflow_core::{ProjectId, RevisionId};
use parquet::{
    data_type::{ByteArray, ByteArrayType},
    file::writer::SerializedFileWriter,
    schema::parser::parse_message_type,
};
use rusqlite::{Connection, OptionalExtension, params};
use std::path::Path;
use std::{io::Cursor, sync::Arc};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("store failure: {0}")]
    Sql(#[from] rusqlite::Error),
    #[error("revision not found")]
    MissingRevision,
    #[error("Parquet failure: {0}")]
    Parquet(String),
}
pub trait ProjectStore {
    fn activate(&mut self, project: &ProjectId, revision: &RevisionId) -> Result<(), StoreError>;
    fn active(&self, project: &ProjectId) -> Result<Option<RevisionId>, StoreError>;
}
pub struct SqliteStore {
    connection: Connection,
}
impl SqliteStore {
    pub fn encode_parquet_feature(payload: &[u8]) -> Result<Vec<u8>, StoreError> {
        let schema = Arc::new(
            parse_message_type("message codeflow { REQUIRED BINARY payload; }")
                .map_err(|error| StoreError::Parquet(error.to_string()))?,
        );
        let mut writer =
            SerializedFileWriter::new(Cursor::new(Vec::new()), schema, Default::default())
                .map_err(|error| StoreError::Parquet(error.to_string()))?;
        let mut group = writer
            .next_row_group()
            .map_err(|error| StoreError::Parquet(error.to_string()))?;
        if let Some(mut column) = group
            .next_column()
            .map_err(|error| StoreError::Parquet(error.to_string()))?
        {
            column
                .typed::<ByteArrayType>()
                .write_batch(&[ByteArray::from(payload.to_vec())], None, None)
                .map_err(|error| StoreError::Parquet(error.to_string()))?;
            column
                .close()
                .map_err(|error| StoreError::Parquet(error.to_string()))?;
        }
        group
            .close()
            .map_err(|error| StoreError::Parquet(error.to_string()))?;
        writer
            .into_inner()
            .map(|cursor| cursor.into_inner())
            .map_err(|error| StoreError::Parquet(error.to_string()))
    }
    pub fn open(path: &Path) -> Result<Self, StoreError> {
        let connection = Connection::open(path)?;
        connection.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON; CREATE TABLE IF NOT EXISTS revisions(project TEXT NOT NULL, revision TEXT NOT NULL, active INTEGER NOT NULL DEFAULT 0, PRIMARY KEY(project,revision)); CREATE TABLE IF NOT EXISTS graph_edges(project TEXT NOT NULL, revision TEXT NOT NULL, source TEXT NOT NULL, target TEXT NOT NULL, PRIMARY KEY(project,revision,source,target)); CREATE TABLE IF NOT EXISTS feature_snapshots(project TEXT NOT NULL, revision TEXT NOT NULL, format TEXT NOT NULL, payload BLOB NOT NULL, PRIMARY KEY(project,revision,format)); CREATE INDEX IF NOT EXISTS revision_active ON revisions(project,active); CREATE INDEX IF NOT EXISTS graph_forward ON graph_edges(project,revision,source); CREATE INDEX IF NOT EXISTS graph_reverse ON graph_edges(project,revision,target);")?;
        Ok(Self { connection })
    }
    pub fn ingest_revision(
        &mut self,
        project: &ProjectId,
        revision: &RevisionId,
    ) -> Result<(), StoreError> {
        self.connection.execute(
            "INSERT OR IGNORE INTO revisions(project,revision) VALUES(?1,?2)",
            params![project.as_str(), revision.as_str()],
        )?;
        Ok(())
    }
    pub fn ingest_edge(
        &mut self,
        project: &ProjectId,
        revision: &RevisionId,
        source: &str,
        target: &str,
    ) -> Result<(), StoreError> {
        self.connection.execute(
            "INSERT OR IGNORE INTO graph_edges(project,revision,source,target) VALUES(?1,?2,?3,?4)",
            params![project.as_str(), revision.as_str(), source, target],
        )?;
        Ok(())
    }
    pub fn adjacent(
        &self,
        project: &ProjectId,
        revision: &RevisionId,
        source: &str,
    ) -> Result<Vec<String>, StoreError> {
        let mut statement=self.connection.prepare("SELECT target FROM graph_edges WHERE project=?1 AND revision=?2 AND source=?3 ORDER BY target")?;
        let rows = statement.query_map(
            params![project.as_str(), revision.as_str(), source],
            |row| row.get(0),
        )?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }
    pub fn reverse_adjacent(
        &self,
        project: &ProjectId,
        revision: &RevisionId,
        target: &str,
    ) -> Result<Vec<String>, StoreError> {
        let mut statement=self.connection.prepare("SELECT source FROM graph_edges WHERE project=?1 AND revision=?2 AND target=?3 ORDER BY source")?;
        let rows = statement.query_map(
            params![project.as_str(), revision.as_str(), target],
            |row| row.get(0),
        )?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }
    pub fn put_feature_snapshot(
        &mut self,
        project: &ProjectId,
        revision: &RevisionId,
        format: &str,
        payload: &[u8],
    ) -> Result<(), StoreError> {
        self.connection.execute("INSERT OR REPLACE INTO feature_snapshots(project,revision,format,payload) VALUES(?1,?2,?3,?4)",params![project.as_str(),revision.as_str(),format,payload])?;
        Ok(())
    }
    pub fn feature_snapshot(
        &self,
        project: &ProjectId,
        revision: &RevisionId,
        format: &str,
    ) -> Result<Option<Vec<u8>>, StoreError> {
        self.connection.query_row("SELECT payload FROM feature_snapshots WHERE project=?1 AND revision=?2 AND format=?3",params![project.as_str(),revision.as_str(),format],|row|row.get(0)).optional().map_err(StoreError::from)
    }
}
impl ProjectStore for SqliteStore {
    fn activate(&mut self, project: &ProjectId, revision: &RevisionId) -> Result<(), StoreError> {
        let transaction = self.connection.transaction()?;
        let exists: Option<i64> = transaction
            .query_row(
                "SELECT 1 FROM revisions WHERE project=?1 AND revision=?2",
                params![project.as_str(), revision.as_str()],
                |row| row.get(0),
            )
            .optional()?;
        if exists.is_none() {
            return Err(StoreError::MissingRevision);
        }
        transaction.execute(
            "UPDATE revisions SET active=0 WHERE project=?1",
            params![project.as_str()],
        )?;
        transaction.execute(
            "UPDATE revisions SET active=1 WHERE project=?1 AND revision=?2",
            params![project.as_str(), revision.as_str()],
        )?;
        transaction.commit()?;
        Ok(())
    }
    fn active(&self, project: &ProjectId) -> Result<Option<RevisionId>, StoreError> {
        Ok(self
            .connection
            .query_row(
                "SELECT revision FROM revisions WHERE project=?1 AND active=1",
                params![project.as_str()],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map(|value| value.and_then(RevisionId::from_serialized))?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static STORE_SEQUENCE: AtomicU64 = AtomicU64::new(0);
    #[test]
    fn activation_is_atomic_and_missing_revision_does_not_replace_active() {
        let path = std::env::temp_dir().join(format!(
            "codeflow-store-test-{}-{}.sqlite",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            STORE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_file(&path);
        let mut store = SqliteStore::open(&path).unwrap();
        let p = ProjectId::derive("p", &["1"]);
        let a = RevisionId::derive("r", &["a"]);
        let b = RevisionId::derive("r", &["b"]);
        store.ingest_revision(&p, &a).unwrap();
        store.activate(&p, &a).unwrap();
        assert!(store.activate(&p, &b).is_err());
        assert!(store.active(&p).unwrap().is_some());
        store.ingest_edge(&p, &a, "a", "b").unwrap();
        store.ingest_edge(&p, &a, "a", "c").unwrap();
        assert_eq!(store.adjacent(&p, &a, "a").unwrap(), vec!["b", "c"]);
        assert_eq!(store.reverse_adjacent(&p, &a, "b").unwrap(), vec!["a"]);
        let parquet = SqliteStore::encode_parquet_feature(b"fixture").unwrap();
        assert_eq!(&parquet[..4], b"PAR1");
        store
            .put_feature_snapshot(&p, &a, "parquet", &parquet)
            .unwrap();
        assert_eq!(
            store.feature_snapshot(&p, &a, "parquet").unwrap(),
            Some(parquet)
        );
        let _ = std::fs::remove_file(path);
    }
}
