# Operations and troubleshooting

CodeFlow is offline by default and does not execute repository scripts. Register
projects through the CLI, inspect `.codeflow/project.json`, and use structured
diagnostic bundles for failures. Missing SCIP or Joern providers are capability
fallbacks, not fatal errors. A failed run is quarantined and must not replace an
active revision. If a watcher or analyzer process exits, rerun the bounded CLI
operation; persisted revisions remain transactionally protected.

Optional install guidance is in `scripts/install.ps1`; release targets are in
`packaging/release-artifacts.json`.
