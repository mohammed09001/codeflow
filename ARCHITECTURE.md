# CodeFlow backend architecture

Rust crates provide canonical IDs, source snapshots, syntax/provider evidence,
UPSM normalization, HAG abstractions, SQLite/Parquet persistence, and bounded
incremental invalidation. The Python package is an optional offline worker for
classical ML, graph representations, workflows, runtime traces, queries, and
resource/security contracts. Deterministic evidence remains authoritative;
learned and runtime outputs are explicitly marked as proposals or observed facts.

## Verification

Run `python scripts/final_acceptance.py`, `cargo test --workspace --all-features`,
and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
