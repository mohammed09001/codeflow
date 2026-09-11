# CodeFlow Execution 02 — Corrective Backend Completion & Frontend Readiness

**Status:** READY FOR EXECUTION  
**Execution type:** Corrective / truth-audit / backend-completion  
**Primary scope:** Backend correctness, production boundaries, cross-platform reliability, and frontend-readiness  
**Frontend implementation:** OUT OF SCOPE  
**Execution mode:** Continuous autonomous convergence  
**Authority:** Observable repository behavior, direct tests, benchmarks, and clean CI are authoritative over prior completion claims.

---

## 0. Mission

Execution 02 exists to close the gap between the intended CodeFlow backend and the implementation that actually exists after Execution 01.

This is **not a rewrite** and must not discard working architecture merely because some phases were incomplete. Preserve proven foundations, repair incorrect or shallow implementations, replace misleading surrogates with real implementations or explicitly scoped experimental alternatives, establish production-grade service boundaries, and finish with a backend that can be frozen behind stable contracts before frontend product work begins.

Execution 02 must specifically correct the following known classes of gaps:

- host-dependent path normalization and non-green cross-platform CI;
- Rust-only Tree-sitter runtime despite a multi-language language model;
- absence of true incremental Tree-sitter reparsing;
- framework semantics that are primarily marker/string based rather than syntax-aware;
- graph algorithms whose names currently overstate their implementations;
- classical-ML baselines whose names/behavior do not fully match the claimed algorithms;
- deterministic graph encoders currently standing in for actual trainable graph-learning models;
- absence of a production `codeflowd` lifecycle;
- absence of a real versioned HTTP service boundary;
- absence of a real revision event stream for frontend consumers;
- CLI commands that expose contracts but are not yet fully wired to the engine lifecycle;
- final acceptance that can report completion without proving all required product behavior;
- CI acceptance that is not green on Windows, Linux, and macOS on the same revision.

The end state is not “more code.” The end state is **truthful, directly verified backend capability**.

---

# 1. Non-Negotiable Product Invariants

These invariants apply to every phase and cannot be weakened to make a test pass.

1. **Implementation truth remains distinct from human abstraction.** UPSM remains the canonical implementation-oriented graph. HAG remains a separate human abstraction layer and must never silently replace deterministic implementation facts.
2. **Evidence provenance is mandatory.** Provider-local identities may be retained as evidence but must not become canonical CodeFlow identities.
3. **Observed and deterministic evidence outrank inference.** ML, graph clustering, lexical semantics, and reconstruction layers may propose or rank interpretations; they must not overwrite deterministic or observed truth.
4. **UNKNOWN is a valid product state.** Do not force a label, relation, feature, workflow, or semantic interpretation where evidence is insufficient.
5. **Stable IDs and versioned schemas are preserved.** Any schema break requires an explicit version transition and compatibility/migration handling.
6. **Offline/local-first operation remains viable.** Core analysis must not require a hosted LLM, remote model API, or mandatory cloud GPU.
7. **Optional analyzers remain optional.** SCIP/Joern or other external analyzers may be capability-gated, but their absence must be explicit and safe rather than falsely treated as complete coverage.
8. **Resource use is bounded.** Parsing, graph analysis, queries, streaming, subprocesses, training, and diagnostics require defined budgets and cancellation behavior.
9. **Source privacy is preserved.** Logs, diagnostics, crash reports, and error surfaces must not casually emit repository source content.
10. **Cross-platform semantics are intentional.** Path safety and project identity must behave consistently across Windows, Linux, and macOS, independent of the host parsing rules used by the standard library.
11. **Frontend-facing data is revision-consistent.** Graph, evidence, query, zoom, and event-stream responses must identify the committed revision they represent.
12. **No product feature may be accepted because a function/class/file has the expected name.** Acceptance is behavioral.
13. **No placeholder completion.** TODOs, FIXME markers, empty adapters, fake training loops, no-op endpoints, or renamed surrogates cannot satisfy a mandatory requirement.
14. **No mandatory LLM dependency.** LLM use may not become a requirement for the product analysis path in this execution.
15. **Frontend implementation is not part of Execution 02.** The backend must become frontend-ready, but the execution must not drift into building the product UI.

---

# 2. Source-of-Truth Hierarchy

When repository documents, comments, evidence, tests, and implementation disagree, use this order:

1. current executable behavior on the current repository revision;
2. the requirements and acceptance criteria in this Execution 02 document;
3. the architectural intent preserved from the existing backend contracts;
4. prior execution state/evidence/final reports;
5. comments, labels, names, and historical claims.

A historical `complete` status cannot override a failing direct test or missing required capability.

Do **not** rewrite old Execution 01 evidence to make history appear clean. Execution 02 records corrections separately.

---

# 3. Execution Operating Model

## 3.1 Continuous execution

Execution 02 is a single continuous corrective program. A phase boundary is a bookkeeping and verification boundary, **not a permission boundary**.

After a phase passes its mandatory acceptance criteria:

1. record its evidence;
2. update the Execution 02 state;
3. refresh the active context;
4. continue immediately into the next phase.

Do not stop merely because:

- a loop iteration completed;
- a phase completed;
- a benchmark completed;
- a file was created;
- a test was fixed;
- a report section was written;
- the next phase touches a different crate or language.

Stop only for a genuine external blocker that cannot be resolved within the repository or available environment, such as an unavailable credential/service with no valid local substitute, a destructive action requiring explicit approval, a licensing/legal blocker, an irreconcilable requirement contradiction, or a tooling/environment limitation that has been demonstrated and cannot reasonably be worked around.

A normal compile/test failure is **not** an external blocker. It is loop input.

## 3.2 Direct verification only

This execution must use direct repository commands, direct unit/integration/end-to-end tests, benchmarks, and CI jobs as the acceptance authority.

Do not introduce an orchestration mechanism that pauses execution after each loop or phase. Do not make phase progression dependent on a separate execution runner. Evidence/state files are passive records only; they must not control progression.

## 3.3 Repair before expansion

Prefer the smallest coherent correction that restores the intended contract.

- Modify, replace, or remove misleading implementation before adding parallel duplicate systems.
- Do not keep a fake/surrogate implementation in production simply because a real implementation is added beside it.
- Preserve useful tests and extend them rather than producing redundant test suites.
- If an existing public symbol is misleading, either implement the claimed semantics or rename/deprecate it with compatibility handling.
- Avoid dependency growth unless the dependency materially reduces correctness risk or implementation complexity and passes license/maintenance review.

---

# 4. Context Standard — Required Before Every Phase

The execution agent must maintain a compact but high-fidelity **Active Context Packet**. It is not a separate tool and must not gate execution. It is a reasoning discipline used before making changes.

At the start of Execution 02 and before every phase, refresh the packet from repository evidence rather than memory alone.

The packet must contain:

### A. Repository identity

- repository root;
- current branch;
- current commit SHA;
- dirty/clean status;
- relevant toolchain versions;
- relevant CI state for the current SHA.

### B. Current architectural truth

For the phase being executed, identify:

- authoritative crates/modules;
- public interfaces and schema versions;
- storage boundaries;
- call sites;
- tests that already constrain behavior;
- data/evidence ownership;
- known capability gates;
- affected frontend-facing contracts, if any.

### C. Baseline defect

State the defect in observable terms:

- expected behavior;
- actual behavior;
- reproducible command/test/fixture demonstrating the gap;
- whether the issue is correctness, semantics, naming, performance, compatibility, security, or missing production wiring.

### D. Change boundary

List:

- files/modules expected to change;
- interfaces that must remain compatible;
- schemas that may change only with explicit versioning;
- components that are intentionally out of scope;
- regression risks.

### E. Acceptance map

Before editing, map every mandatory phase requirement to one or more direct verification mechanisms:

- unit test;
- integration test;
- end-to-end test;
- benchmark;
- golden fixture;
- CI matrix result;
- explicit inspection of generated contract/artifact where behavioral execution is impossible.

No requirement may be closed with “implemented” as its only evidence.

### F. Context refresh triggers

Refresh the Active Context Packet whenever:

- a public interface changes;
- a schema changes;
- a new dependency is introduced;
- a benchmark contradicts an assumption;
- three consecutive repair attempts address symptoms without eliminating the failing behavior;
- a test failure reveals a wider architectural dependency;
- a phase crosses Rust/Python/service/CI boundaries;
- the working tree differs materially from the initial phase plan.

This prevents context drift during a long execution without forcing the agent to pause.

---

# 5. Prompt Standard — Master Execution Prompt

Use the following behavior as the execution contract for the coding agent performing Execution 02:

> You own the complete backend outcome of CodeFlow Execution 02. Work from repository evidence, not completion claims. Preserve correct architecture, repair incorrect or shallow behavior, and continuously execute phases in order until the final acceptance gate passes or a genuine external blocker is proven. A phase boundary is not a request-for-approval boundary. Never stop merely to announce that a phase or loop is complete. Do not treat function names, files, interfaces, comments, evidence JSON, or previous reports as proof that behavior exists. Reproduce each defect, implement the smallest correct repair, run direct targeted verification, run relevant regressions, and compare observed results with the explicit acceptance criteria. If verification fails, diagnose the causal defect and continue the repair loop; do not ask for approval for ordinary engineering decisions. Preserve deterministic/observed evidence authority, UNKNOWN states, stable IDs, versioned schemas, local-first operation, privacy, resource bounds, and cross-platform correctness. ML and reconstructed semantics are proposal layers and must never overwrite implementation truth. Keep the repository coherent; do not accumulate duplicate dead systems or placeholders. Do not introduce an execution mechanism that stops between loops or phases. Use direct commands and CI as authority. Record truthful evidence after success, then continue immediately. At the end, run the full final acceptance from a clean state and mark frontend readiness only if every mandatory backend contract is proven on the same accepted revision.

This prompt is self-contained. The execution agent must not depend on inaccessible private planning documents to infer mandatory requirements.

---

# 6. Autonomous Convergence Loop

Every phase uses this loop:

```text
READ CURRENT REPOSITORY STATE
        ↓
REFRESH ACTIVE CONTEXT PACKET
        ↓
REPRODUCE THE BASELINE DEFECT / MISSING CAPABILITY
        ↓
DEFINE THE SMALLEST CORRECT DELTA
        ↓
IMPLEMENT
        ↓
FORMAT / STATIC CHECK / COMPILE
        ↓
RUN TARGETED TESTS
        ↓
RUN RELEVANT REGRESSION SUITE
        ↓
RUN BENCHMARK / CONTRACT / E2E CHECK WHEN REQUIRED
        ↓
COMPARE OBSERVED RESULT WITH PHASE ACCEPTANCE
        ↓
    PASS? ── YES → RECORD EVIDENCE → CONTINUE TO NEXT PHASE
      │
      NO
      ↓
CLASSIFY FAILURE
(correctness / integration / performance / contract / environment)
      ↓
TRACE ROOT CAUSE
      ↓
REPAIR OR REPLAN
      ↓
REPEAT WITHOUT WAITING FOR USER APPROVAL
```

There is no arbitrary maximum iteration count. Repetition ends when the mandatory acceptance criteria are satisfied or a genuine external blocker is proven.

When repeated attempts fail, do not repeat the same patch pattern. Re-open assumptions, call sites, data ownership, and architecture boundaries and choose a different implementation path.

---

# 7. Execution Records

Create and maintain:

```text
Execution/Execution 02.state.json
Execution/evidence-02/phase-00.json
Execution/evidence-02/phase-01.json
...
Execution/evidence-02/phase-17.json
Execution/Execution 02 Final Report.md
```

These are **passive audit records**, not execution controllers.

Each phase evidence record must include at minimum:

```json
{
  "execution": "02",
  "phase": "NN",
  "status": "complete | blocked",
  "baseline_defect": [],
  "files_changed": [],
  "commands_run": [
    {"command": "...", "exit_code": 0}
  ],
  "tests": [],
  "benchmark_results": [],
  "contract_or_schema_changes": [],
  "known_limitations": [],
  "unresolved_blockers": [],
  "next_phase": "NN"
}
```

Rules:

- `complete` means the mandatory acceptance is behaviorally proven.
- A capability may not be marked complete merely because an adapter exists.
- A production algorithm may not be marked complete while a surrogate remains behind its production name.
- Red CI means final acceptance cannot be complete.
- A known limitation cannot be used to waive a mandatory requirement unless this document explicitly allows the capability to remain optional.
- Preserve raw benchmark numbers and machine/environment metadata where performance is part of acceptance.

---

# 8. Phase 00 — Truth Baseline, Contract Inventory, and Defect Registry

## Objective

Establish a reproducible baseline and map every known Execution 01 gap to an explicit repair and verification path before changing production behavior.

## Work

1. Record branch, SHA, toolchains, operating system, dependency lock state, and current CI state.
2. Run the current direct baseline commands without changing production code.
3. Inventory public Rust/Python/CLI/backend contracts and all schema/version constants.
4. Inventory the UPSM/HAG/evidence/store boundaries and identify which are already trustworthy and must be preserved.
5. Build a defect registry containing at least:
   - cross-platform path validation;
   - Linux/macOS CI failures;
   - parser language coverage;
   - incremental parsing absence;
   - framework semantic extraction quality;
   - graph clustering surrogates;
   - classical-ML naming/semantic gaps;
   - trainable graph-learning absence;
   - model-comparison gate absence;
   - daemon absence;
   - HTTP API absence;
   - event-stream absence;
   - incomplete CLI wiring;
   - incomplete final acceptance coverage.
6. Map each defect to exactly one primary repair phase and list dependent phases.
7. Capture existing externally visible contracts that the repair must preserve or explicitly version.
8. Do not rewrite historical Execution 01 evidence.

## Mandatory acceptance

- Every known audited gap is assigned to a phase.
- Existing schemas/public boundaries are inventoried.
- Baseline failures are reproducible.
- The working tree is coherent before Phase 01 begins.
- No architecture rewrite is proposed without evidence that the current foundation prevents the required behavior.

---

# 9. Phase 01 — Cross-Platform Path Semantics and Green Core CI

## Objective

Make project/source path safety deterministic across Windows, Linux, and macOS and eliminate the known host-dependent test failure.

## Work

1. Replace host-dependent assumptions in relative-path validation with explicit portable semantics.
2. Consistently reject:
   - Unix absolute paths;
   - Windows drive-rooted paths such as `C:\\secret` and `C:/secret` on every host;
   - UNC/network roots where not explicitly supported;
   - `..` traversal;
   - empty paths;
   - malformed or ambiguous root/prefix forms that could escape the project boundary.
3. Preserve valid normalized repository-relative paths and normalize separator representation deterministically.
4. Add table-driven cross-platform fixtures executed on all CI runners.
5. Ensure symlink/canonicalization policy is explicit at the project-boundary layer rather than accidentally delegated to string normalization.
6. Repair CI/toolchain configuration warnings that materially affect verification, including incorrect action inputs/components if present.
7. Run the Rust workspace checks on Windows, Ubuntu, and macOS.

## Mandatory acceptance

On the same commit:

```text
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

must pass on Windows, Linux, and macOS.

Dedicated tests must prove identical treatment of Windows-style absolute paths even when executed on Unix hosts.

Do not proceed with a known red core CI matrix.

---

# 10. Phase 02 — Multi-Language Tree-sitter Registry and True Incremental Parsing

## Objective

Turn the parser layer from Rust-only coverage into the multi-language deterministic syntax substrate implied by CodeFlow’s language model, and implement actual incremental reparsing.

## Required language coverage

Provide grammar adapters for all supported concrete `Language` variants currently exposed by the product contract unless a variant is explicitly removed through a versioned contract change:

- Rust
- Python
- TypeScript
- JavaScript
- Java
- Kotlin
- Go
- C
- C++
- C#

`Unknown` remains explicit and unsupported rather than guessed.

## Work

1. Introduce a grammar registry/provider architecture rather than expanding a single match into an unmaintainable monolith.
2. Pin grammar dependencies/versions.
3. Preserve the rule that Tree-sitter node IDs never become canonical CodeFlow identities.
4. Normalize syntax evidence into repository-stable source identities/spans.
5. Implement incremental reparsing with a retained previous tree and explicit edit application (`InputEdit` or the equivalent required by the selected Tree-sitter bindings), then parse using the edited previous tree.
6. Define cache invalidation by source blob/revision identity.
7. Preserve parse errors as bounded evidence rather than silently discarding the file.
8. Add cancellation/resource guards for pathological files.
9. Create representative fixtures for every supported language including syntax-error recovery.
10. Test full parse vs incremental parse equivalence after insert/delete/replace edits.
11. Ensure comments/string literals do not become semantic facts merely because they contain syntax-like tokens.

## Mandatory acceptance

- Every supported language has at least one valid parse fixture and one error/recovery fixture where meaningful.
- Unsupported `Unknown` behavior is explicit and typed.
- Incremental reparse is proven to reuse/edit a prior tree rather than always parsing from `None`/scratch.
- Full parse and incremental reparse produce equivalent normalized evidence for the same final source.
- Parser/provider-local IDs do not leak into canonical CodeFlow IDs.
- Workspace tests remain green across the CI matrix.

---

# 11. Phase 03 — SCIP / Joern / External Analysis Adapter Correctness

## Objective

Keep external analyzers optional while making their adapters trustworthy, bounded, provenance-correct, and integration-tested.

## Work

1. Verify official SCIP protobuf decoding against representative fixtures.
2. Preserve explicit index status semantics such as usable/partial/stale; partial coverage must never be promoted to complete semantic truth.
3. Verify SCIP symbols remain provider evidence and do not become canonical IDs.
4. Test safe merge into UPSM with duplicate, conflicting, and partial provider evidence.
5. For Joern/CPG or equivalent external processes:
   - probe capability explicitly;
   - use direct process invocation rather than unsafe shell composition;
   - bound runtime;
   - bound output size;
   - kill and reap timed-out children;
   - sanitize error surfaces;
   - reject unsafe paths;
   - keep provider-local identities local.
6. Add fixture-driven tests that do not require the external binary to be installed for ordinary CI.
7. If the provider is installed in an optional environment, run a real integration smoke test.
8. Do not turn optional provider absence into a pipeline failure or false semantic negative.

## Mandatory acceptance

- Fixture decoding/import/merge is deterministic.
- stale/partial semantics are directly tested.
- timeout/process cleanup is directly tested where practical.
- ordinary core operation works with all optional external analyzers absent.
- provider presence enhances evidence without changing canonical identity rules.

---

# 12. Phase 04 — Parse-Aware Framework Semantics

## Objective

Replace production reliance on simple source-string markers with syntax-aware framework semantic extraction while preserving conservative fallback behavior.

## Work

Build parse/query-aware adapters for representative supported ecosystems. Coverage should include static constructs for the ecosystems the language layer claims to understand, including where practical:

- Rust: Axum / Actix-style routing;
- Python: FastAPI / Flask / Django route structures;
- TypeScript/JavaScript: Express / Nest / Next-style server routing where statically recoverable;
- Java/Kotlin: Spring / Ktor-style routes/controllers;
- Go: `net/http` and common router patterns;
- C#: ASP.NET endpoint/controller patterns.

For every extracted semantic:

- attach provider;
- source span/evidence;
- fact class;
- confidence;
- route/method/name metadata when recoverable.

Lexical/string-marker fallback may remain only as a clearly lower-authority inferred signal. It must not claim deterministic framework truth.

## Mandatory acceptance

Tests must include:

- true positives;
- near-miss negatives;
- comments containing route markers;
- string literals containing route markers;
- aliased/imported framework symbols where recoverable;
- malformed/incomplete files;
- unknown/unrecognized framework behavior.

A comment such as `"app.get("` must not create a deterministic HTTP route fact.

---

# 13. Phase 05 — Real Graph Algorithms and Honest Semantics

## Objective

Eliminate production algorithm-name inflation. Implement the actual algorithms required by the backend or rename/remove non-equivalent surrogates.

## Required algorithms

### A. Leiden community detection

Implement a genuine Leiden community-detection path supporting the graph characteristics CodeFlow needs, including deterministic seed/configuration, weighted edges where applicable, disconnected graphs, and stable serialization of results.

### B. Agglomerative clustering

Implement actual hierarchical/agglomerative clustering with explicit distance/similarity definition and linkage strategy. Do not alias it to another graph partitioner.

### C. HDBSCAN

Implement real density-based hierarchical clustering/noise semantics appropriate to the chosen feature representation. A degree threshold applied to connected components is not sufficient.

## Dependency policy

A mature library may be used if:

- its license is compatible;
- maintenance/security posture is acceptable;
- deterministic controls are understood;
- the wrapper preserves CodeFlow evidence/identity semantics;
- benchmarks show resource use is acceptable.

If implementing internally, add algorithm-specific golden tests sufficient to distinguish the implementation from connected-components shortcuts.

## Mandatory acceptance

Use synthetic and representative graph fixtures with known/expected structural behavior:

- two dense communities connected by a weak bridge;
- disconnected components;
- isolated nodes;
- weighted edges;
- ambiguous boundary nodes;
- explicit density noise for HDBSCAN;
- deterministic repeat runs with fixed seed/configuration.

Record partition-quality metrics where applicable and a large-graph runtime/memory benchmark.

All outputs remain `Inferred`/proposal semantics and must never overwrite UPSM deterministic truth.

---

# 14. Phase 06 — Classical ML Correctness and Evaluation Discipline

## Objective

Ensure every production classical-ML component behaves as its name and contract claim, and establish a leakage-resistant evaluation baseline for later graph learning comparison.

## Work

1. Implement a genuine probabilistic Naive Bayes variant appropriate to the feature representation, or rename/remove the existing non-Naive-Bayes behavior.
2. Preserve and harden logistic regression training with stable preprocessing, convergence/iteration configuration, and deterministic seeds.
3. Replace any function presented as a tree model that is only a weighted linear ranker with an actual tree-based baseline or truthful naming.
4. Define feature schemas and missing/UNKNOWN behavior explicitly.
5. Split training/validation/test data by repository/project boundaries where possible to avoid entity-level leakage.
6. Add calibration evaluation and preserve uncalibrated vs calibrated score provenance.
7. Record model/data/schema version, seed, hyperparameters, and metrics.
8. Do not force UNKNOWN candidates to a class merely to improve aggregate metrics.

## Required metrics

Use task-appropriate metrics and record at least:

- macro F1;
- micro F1 or accuracy when meaningful;
- per-class precision/recall;
- PR-AUC for imbalanced binary/ranking tasks where applicable;
- Brier/calibration measurements for confidence-bearing predictions;
- inference latency;
- peak or bounded memory estimate.

## Mandatory acceptance

- algorithm-specific tests distinguish each model from a naming-only stub;
- deterministic seed behavior is tested;
- held-out repository/project evaluation is produced where training data permits;
- no train/test leakage through shared entity IDs or duplicated fixtures;
- model output remains inferential evidence.

---

# 15. Phase 07 — Trainable Graph Representation and GNN Layer

## Objective

Replace deterministic “GNN-shaped” encoders with actual trainable graph representation models while keeping deep learning optional for product correctness.

## Required research/implementation paths

Implement and directly train/evaluate:

1. node2vec and/or MetaPath2Vec-style representation baseline appropriate to the graph;
2. Graph Autoencoder with a GCN encoder;
3. Graph Autoencoder with a GAT encoder;
4. a heterogeneous graph model that uses node/edge types rather than ignoring them;
5. typed link reconstruction objective;
6. masked-feature/self-supervised objective where useful;
7. negative sampling;
8. neighbor-sampled or mini-batched training path for graphs that do not fit naive full-graph execution.

## Training requirements

A valid trainable path must have:

- learnable parameters;
- loss computation;
- backward pass/autodiff;
- optimizer step;
- epochs/early-stop policy;
- deterministic/random seed control;
- checkpoint/model metadata;
- CPU execution;
- optional GPU acceleration that is capability-detected rather than required.

Do not fake trainability with fixed arithmetic transforms.

## Architecture constraints

- Deep learning never becomes the authority for deterministic program facts.
- Core CodeFlow analysis must work when DL dependencies are unavailable.
- Heavy ML dependencies should be isolated behind optional packaging/features if feasible.
- Model artifacts must identify graph/schema/data compatibility.
- Runtime product inference must be bounded and cancellable.

## Mandatory acceptance

- a tiny fixture can demonstrably reduce training loss or improve the defined representation objective;
- saved model/checkpoint reload produces compatible inference;
- CPU training/inference smoke tests pass;
- optional GPU path is tested when hardware exists but absence does not fail core CI;
- heterogeneous type information measurably affects the heterogeneous model path;
- no model output mutates UPSM truth without an evidence-preserving proposal step.

---

# 16. Phase 08 — Model Comparison and Retention Gate

## Objective

Decide whether graph deep learning deserves a default product role using evidence rather than architecture fashion.

## Work

Before looking at final held-out results, record the comparison protocol and primary metric.

Compare at minimum:

- deterministic/structural baseline;
- classical ML baseline;
- graph-representation baseline;
- trained GCN autoencoder path;
- trained GAT path;
- heterogeneous graph model.

Use the same held-out repository/project protocol where possible.

Perform ablations for important signal groups where data permits.

## Default material-improvement rule

Unless repository evidence establishes a better pre-registered rule, retain DL in the default product path only if it produces a material improvement over the strongest non-DL baseline, such as one of:

- at least **+0.03 absolute macro-F1**, or
- at least **+0.05 absolute PR-AUC** on the pre-declared primary imbalanced task,

while remaining within the declared latency/memory/resource budget.

The exact threshold and primary metric must be recorded **before** inspecting the final held-out result.

## Important outcome rule

If DL does not materially outperform simpler approaches, that is a successful scientific result. Keep the deterministic/classical path as default and retain DL as optional/experimental or remove it from the production path. Do not manipulate the benchmark or force DL into the architecture.

## Mandatory acceptance

- comparison protocol recorded before final evaluation;
- reproducible held-out results recorded;
- latency/memory tradeoffs recorded;
- default product selection justified by measurements;
- no weaker model is made default solely because it is more sophisticated.

---

# 17. Phase 09 — Workflow and Runtime Reconstruction Truth Hardening

## Objective

Make workflow/path/runtime abstractions precise about what is observed, reconstructed, and unknown.

## Work

1. Bound static path reconstruction; never claim complete runtime coverage from static enumeration.
2. Keep runtime traces as observed-only evidence.
3. Ensure trace abstraction summarizes observed sessions and does not synthesize unseen calls/events as observed.
4. Reconcile static and runtime contradictions through evidence/provenance rather than destructive replacement.
5. Bound path count/depth/cycle expansion.
6. Preserve revision and source identity through workflow projections.
7. Add tests for recursion, cycles, conditional branches, missing runtime coverage, stale trace revisions, and conflicting provider evidence.

## Mandatory acceptance

- static-only paths are explicitly reconstructed/inferred;
- runtime-backed paths are explicitly observed where supported;
- unobserved behavior is never mislabeled as observed;
- bounded enumeration passes pathological graph fixtures;
- repeated runs are deterministic under the same evidence/configuration.

---

# 18. Phase 10 — Query, Semantic Zoom, and Frontend View Contract Freeze

## Objective

Stabilize the backend data model that a future frontend will consume before any production UI is built.

## Work

Define/version the frontend-facing view contracts for:

- project identity;
- committed revision identity;
- graph node/edge views;
- HAG/UPSM distinction;
- fact class/confidence;
- evidence references;
- query results;
- semantic zoom/materialization;
- pagination/cursors;
- capability availability;
- partial/unknown status;
- bounded errors.

Requirements:

1. deterministic ordering for serialized collections;
2. stable IDs within the intended identity domain;
3. explicit revision on every revision-dependent response;
4. cursor pagination that cannot silently cross incompatible revisions;
5. query budgets and cancellation;
6. bounded semantic-zoom materialization;
7. golden serialization fixtures;
8. backwards-compatibility tests for the v1 contract once frozen.

## Mandatory acceptance

The same committed project revision queried twice with unchanged inputs produces semantically identical, deterministically ordered frontend-view payloads.

No frontend-facing response may require consumers to infer whether a relation is deterministic, inferred, reconstructed, observed, or unknown from undocumented conventions.

---

# 19. Phase 11 — Production `codeflowd` Lifecycle

## Objective

Create the real local backend service process that owns CodeFlow project analysis and revision state.

## Architecture

Prefer a Rust daemon/service binary integrated with the canonical Rust engine unless repository evidence strongly justifies another boundary. Avoid a Python-only shim that merely echoes metadata while the real engine remains inaccessible.

## Required lifecycle

`codeflowd` must support:

- startup and graceful shutdown;
- local project initialization/open;
- project state ownership;
- analysis request scheduling;
- watch/update scheduling;
- committed revision publication;
- query/view access to committed state;
- capability discovery;
- cancellation;
- bounded concurrency;
- crash-safe or recoverable store handling;
- explicit errors when optional capabilities are unavailable.

Bind to loopback by default unless explicitly configured otherwise.

A failed/in-progress analysis must not corrupt the previously committed readable revision.

## Mandatory acceptance

An end-to-end test must:

1. start the daemon as a real process;
2. open/init a temporary fixture project;
3. request analysis;
4. wait for a committed revision;
5. query graph/evidence data from that revision;
6. stop the daemon cleanly;
7. restart and confirm persisted project/revision state remains coherent where persistence is intended.

Core daemon operation must not require DL dependencies or optional external analyzers.

---

# 20. Phase 12 — Versioned HTTP API and Internal Service Contract

## Objective

Expose a real, production-usable v1 service contract for the future frontend.

## HTTP API

Implement a real HTTP JSON service on top of `codeflowd`. Exact paths may adapt to existing project conventions, but the v1 contract must cover the equivalent of:

```text
GET  /v1/health
GET  /v1/capabilities
POST /v1/projects
GET  /v1/projects/{project_id}
POST /v1/projects/{project_id}/analyze
GET  /v1/projects/{project_id}/status
GET  /v1/projects/{project_id}/revisions
GET  /v1/projects/{project_id}/graph
POST /v1/projects/{project_id}/query
GET  /v1/projects/{project_id}/evidence/{evidence_id}
```

Contract requirements:

- typed errors;
- explicit API version;
- explicit project/revision identity;
- cursor pagination;
- request/response size limits;
- query/time budgets;
- cancellation/timeout behavior;
- deterministic serialization;
- capability errors distinct from internal failures;
- safe malformed-input handling.

Generate or maintain a machine-readable OpenAPI contract and verify it against the running service.

## Internal RPC

If an internal gRPC boundary remains part of the retained architecture, implement it as a real tested service contract with one canonical schema and no duplicate semantic model. If repository evidence shows it adds no current boundary value, document the decision and keep one production source of truth rather than maintaining ceremonial RPC code.

## Mandatory acceptance

End-to-end API tests must run against a real spawned daemon, not direct function calls only.

Unsupported API versions, invalid cursors, stale revisions, missing capabilities, oversized requests, and malformed payloads must produce deterministic typed failures.

---

# 21. Phase 13 — Revision Event Stream for Frontend Consumers

## Objective

Provide a robust unidirectional event stream so a future frontend can react to analysis/revision changes without polling the entire backend.

## Preferred transport

Use SSE unless a proven bidirectional requirement justifies WebSocket. Do not introduce WebSocket merely for novelty.

Suggested endpoint:

```text
GET /v1/projects/{project_id}/events
```

Suggested event classes:

```text
analysis_started
analysis_progress        (optional, bounded/coalesced)
revision_committed
analysis_completed
analysis_failed
capabilities_changed
```

## Requirements

- monotonic event IDs/cursors per relevant stream domain;
- revision ID in revision-affecting events;
- reconnect/resume using `Last-Event-ID` or equivalent;
- heartbeat/keepalive policy;
- bounded queues;
- explicit slow-consumer/backpressure behavior;
- no source-code payload leakage;
- clean disconnect handling;
- no event claiming a revision before it is atomically readable through the query API.

## Mandatory acceptance

A direct integration test must:

1. connect to the stream;
2. trigger analysis;
3. observe lifecycle events;
4. observe a committed revision;
5. disconnect;
6. trigger another revision;
7. reconnect from a cursor;
8. receive the missing committed event exactly according to the documented semantics;
9. query that revision successfully through the HTTP API.

---

# 22. Phase 14 — Real CLI Wiring and End-to-End Local Workflow

## Objective

Turn the existing CLI surface into a functional local interface to the real backend/engine lifecycle.

## Required commands

Preserve or version the existing intent of:

```text
codeflow init
codeflow analyze
codeflow watch
codeflow status
codeflow graph
codeflow evidence
```

## Requirements

- commands perform actual work rather than printing only command metadata;
- machine-readable JSON output remains available and versioned;
- human-readable output does not change backend semantics;
- predictable exit codes;
- cancellation/interrupt behavior;
- daemon auto-connect/start policy is explicit;
- `watch` reflects committed revisions and does not expose half-written state;
- `graph` and `evidence` consume the same canonical backend contracts used by future frontend clients.

## Mandatory acceptance

A clean temporary project must pass an end-to-end CLI workflow:

```text
init → analyze → status → graph/query → evidence → watch/change/revision
```

with assertions on actual returned backend data, not only process exit success.

---

# 23. Phase 15 — Security, Privacy, Resource, and Observability Hardening

## Objective

Harden the now-real service boundary before declaring it frontend-ready.

## Work

1. Keep default network exposure on loopback.
2. Define local client authorization/session policy where needed; do not assume every local webpage/process should automatically control the daemon.
3. Default-deny or tightly constrain CORS/origin behavior.
4. Re-test path traversal, Windows drive forms, UNC paths, symlinks/project escape behavior, and unsafe provider document paths.
5. Ensure subprocess execution avoids shell injection.
6. Bound subprocess time, memory where practical, output, and process count.
7. Bound parser/query/graph/stream/training resource use.
8. Ensure diagnostics contain metadata/provenance without repository source text unless an explicit local diagnostic mode requests it.
9. Sanitize internal errors crossing the API boundary.
10. Add malformed/fuzz/property tests where they provide meaningful coverage for parsers/contracts.
11. Verify graceful behavior under queue saturation, oversized repositories, cancelled requests, and unavailable optional providers.
12. Ensure observability identifies revision/request/capability state without logging raw source.

## Mandatory acceptance

Security/privacy/resource tests must run directly in CI where deterministic. No high-severity known path escape, shell injection, unauthenticated non-loopback exposure, or source-leaking diagnostic defect may remain open at final acceptance.

---

# 24. Phase 16 — Packaging, Clean-Machine Verification, and Release Matrix

## Objective

Prove the backend can be built, installed, started, and tested outside the developer’s current machine.

## Work

1. Keep dependency versions locked/reproducible.
2. Separate optional heavy ML/GPU dependencies from the minimal product path where technically practical.
3. Produce release builds/artifacts for supported desktop/server targets.
4. Add clean-machine installation/startup smoke tests.
5. Verify migration/startup with a fresh project store and with the latest compatible prior schema fixture.
6. Run full CI on:
   - Windows;
   - Ubuntu/Linux;
   - macOS.
7. Run service/API/SSE/CLI integration tests in CI where supported.
8. Document platform-specific optional capability differences explicitly.
9. Update architecture/operator/API documentation to match implementation truth.

## Mandatory acceptance

The same accepted source revision must have a green required CI matrix. A platform may not be called supported if its mandatory core tests are red.

The base CodeFlow service must start without requiring a GPU, external model service, Joern, or another optional analyzer.

---

# 25. Phase 17 — Final Acceptance and Frontend Readiness Gate

## Objective

Perform a direct, evidence-based acceptance of the complete corrected backend and decide whether the backend contract can be frozen for production frontend development.

## 25.1 Mandatory direct commands

From a clean checkout/state, run the relevant final commands directly. At minimum:

```text
cargo fmt --check
cargo build --workspace --release
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
python -m compileall -q python scripts
python -m unittest discover -s scripts/tests -v
python scripts/no_llm_guard.py
```

Also run the direct Python ML regression suites defined by the final repository layout, plus direct daemon/API/SSE/CLI end-to-end suites and packaging smoke tests.

If the ML/DL layer is isolated behind optional environments/features, test both:

- minimal/core product installation;
- ML-enabled installation.

No final acceptance script may substitute existence checks for these behavioral commands.

## 25.2 CI gate

Windows, Linux, and macOS required jobs must be green on the **same final commit**.

A stale earlier green run cannot validate a later accepted SHA.

## 25.3 Requirement-to-evidence audit

For every phase 00–17:

- verify the evidence record exists;
- verify its commands correspond to the current accepted behavior;
- verify mandatory tests actually ran;
- verify the implementation still contains the accepted behavior after later phases;
- re-run cross-phase end-to-end behavior where integration could invalidate earlier isolated tests.

## 25.4 Placeholder and truth audit

Before declaring success:

- search production code for TODO/FIXME/placeholders relevant to mandatory behavior;
- inspect production algorithm names against their implementations;
- verify no fake encoder/training path is still presented as a trained model;
- verify no HTTP/RPC/event endpoint is documentation-only;
- verify supported parser languages are actually accepted by the runtime;
- verify capability-gated optional providers are reported as optional rather than silently complete;
- verify frontend-facing contracts remain versioned and revision-consistent.

## 25.5 End-to-end frontend-readiness scenario

A single real scenario must prove:

```text
start backend
   ↓
initialize/open project
   ↓
analyze multi-language fixture project
   ↓
commit revision
   ↓
receive revision event
   ↓
query project status/capabilities
   ↓
query graph view
   ↓
perform bounded query / semantic zoom
   ↓
fetch supporting evidence
   ↓
modify source
   ↓
incrementally analyze/watch
   ↓
commit next revision
   ↓
resume/observe event stream
   ↓
query new revision consistently
   ↓
clean shutdown / restart
```

The test must assert substantive data and revision relationships, not only HTTP status codes.

## 25.6 Frontend readiness decision

Set in the final report:

```text
frontend_readiness: READY
```

only when all of the following are true:

- core CI is green on Windows/Linux/macOS for the accepted SHA;
- multi-language parsing is real for the declared supported set;
- incremental parsing is real and verified;
- graph algorithms use truthful implementations/names;
- classical ML baselines are truthful and evaluated;
- trainable graph models have been genuinely trained/evaluated or have been intentionally demoted from the default product after the model-retention gate;
- `codeflowd` lifecycle is real;
- v1 HTTP API is real and contract-tested;
- frontend revision stream is real and reconnect-tested;
- CLI is wired to real behavior;
- graph/query/evidence/view schemas are stable and versioned;
- final end-to-end scenario passes;
- there are no mandatory known blockers hidden as “limitations.”

Otherwise set:

```text
frontend_readiness: NOT_READY
```

and enumerate exact blockers. Do not soften the result.

---

# 26. Cross-Phase Regression Policy

A phase passing once does not protect it from later regression.

Whenever a later phase changes any of the following, re-run the owning earlier phase’s relevant tests:

- canonical IDs;
- path normalization;
- source spans;
- parser evidence;
- UPSM/HAG schemas;
- revision semantics;
- persistence schema;
- graph algorithm input representation;
- feature schema;
- model serialization;
- query/view serialization;
- API schemas;
- event schema;
- CLI output schema.

The final accepted state is the only state that matters for readiness.

---

# 27. Performance Discipline

Correctness comes first, but avoid repairing correctness by making ordinary use impractical.

For phases that can materially affect performance, preserve before/after benchmark evidence with repository size/fixture characteristics and environment metadata.

At minimum measure where applicable:

- cold full analysis latency;
- incremental analysis latency;
- parser throughput;
- graph materialization latency;
- bounded query p50/p95;
- daemon startup time;
- event propagation time from committed revision to client observation;
- peak memory on representative repositories;
- ML inference latency;
- optional training cost separately from product inference.

No universal numeric target is invented here without a representative dataset. Instead, establish a baseline in Phase 00/affected phase, define the acceptable regression budget before the final benchmark, and justify any significant regression with a concrete correctness/capability gain.

---

# 28. Dependency and Licensing Discipline

Before adding a new runtime or ML/graph dependency:

1. confirm why existing dependencies cannot satisfy the requirement cleanly;
2. inspect license compatibility;
3. inspect maintenance/release posture;
4. pin or lock versions through the repository’s normal dependency mechanism;
5. isolate optional heavyweight dependencies when possible;
6. add a direct smoke test proving the wrapper/adapter behavior CodeFlow depends on;
7. do not expose third-party internal IDs as CodeFlow canonical identities.

Do not reimplement a complex algorithm badly merely to avoid every dependency, and do not add a large dependency for a trivial helper.

---

# 29. Schema and Compatibility Discipline

When changing a schema or serialized contract:

- state whether the change is additive, compatible, migratable, or breaking;
- update the explicit schema/API version when required;
- add migration/compatibility tests;
- keep old data behavior deterministic;
- update golden fixtures intentionally;
- never silently reinterpret an existing field with a new meaning;
- keep frontend v1 contracts frozen after Phase 10 except for additive backwards-compatible changes required to finish service wiring.

If a post-Phase-10 change would break v1, either redesign it to be compatible or explicitly version the contract rather than mutating v1 invisibly.

---

# 30. Failure Classification and Escalation Rules

When verification fails, classify it before editing again:

### Correctness failure

Observed output is wrong. Trace input → normalization → analysis → evidence → projection → response.

### Integration failure

Components work alone but not together. Inspect ownership, lifecycle, revision, serialization, and cancellation boundaries.

### Performance failure

Correct output exceeds a declared budget. Profile before optimizing; do not guess.

### Contract failure

Implementation and public/schema/API behavior disagree. Decide whether implementation or versioned contract is authoritative under this document, then repair coherently.

### Environment failure

Only classify as environment when the failure depends on an unavailable external capability or platform fact. A cross-platform bug in CodeFlow is not an environment excuse.

If three consecutive repair attempts do not remove the same failure, refresh the full context packet and re-evaluate the architectural assumption causing it.

---

# 31. Explicit Non-Goals

Execution 02 does **not**:

- build the production frontend;
- redesign the product brand/UI;
- introduce a mandatory hosted backend merely to make local service wiring easier;
- make LLMs authoritative program-analysis components;
- make GPU availability a requirement for core CodeFlow;
- replace UPSM/HAG with a single blended graph;
- rewrite correct storage/identity/evidence foundations without evidence of necessity;
- add cloud accounts/auth/billing unless a direct backend requirement already exists outside this corrective scope;
- chase additional analyzers/languages/frameworks beyond the declared supported contract while mandatory gaps remain open.

---

# 32. Definition of Done

Execution 02 is complete only when the repository provides a truthful, production-usable backend foundation with:

- portable path safety;
- green required Windows/Linux/macOS CI;
- real multi-language parsing;
- real incremental parsing;
- evidence-correct optional external analyzers;
- parse-aware framework semantics;
- truthful graph algorithms;
- truthful classical ML;
- genuinely trainable graph-learning experiments with an evidence-based retention decision;
- bounded workflow/runtime reconstruction;
- frozen versioned frontend-view contracts;
- a real `codeflowd` lifecycle;
- a real v1 HTTP API;
- a reliable revision event stream;
- a real CLI workflow;
- security/privacy/resource hardening;
- clean packaging/startup verification;
- a direct full-system acceptance scenario;
- a final report whose claims match the exact accepted commit.

The desired final state is:

```text
Execution 01 architecture preserved where correct
                 +
Execution 02 corrective gaps closed
                 ↓
Backend contracts frozen and directly proven
                 ↓
frontend_readiness: READY
                 ↓
Production frontend execution may begin
```

Anything less must be reported as `NOT_READY` with the remaining blockers stated explicitly.
