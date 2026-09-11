# CodeFlow Execution 04 — Deterministic Closure Execution

**Status:** READY FOR EXECUTION  
**Execution type:** Final corrective closure / anti-false-completion / frontend-readiness convergence  
**Project:** CodeFlow  
**Baseline repository:** `mohammed09001/codeflow`  
**Baseline branch:** `main`  
**Observed baseline SHA at authoring:** `5781ddc7e0485776f7a06a54ff63791288dc6a62`  
**Primary purpose:** Close every remaining mandatory backend gap after Execution 03 and make false completion mechanically difficult.  
**Frontend implementation:** OUT OF SCOPE  
**Normal stopping condition:** Final same-SHA verifier passes and `frontend_readiness = READY`  
**Execution controller/harness:** PROHIBITED  
**LLM in CodeFlow product core:** PROHIBITED  
**GPU requirement for core product:** PROHIBITED  

---

# 0. Why Execution 04 Exists

Execution 01 established the backend architecture.

Execution 02 attempted corrective completion.

Execution 03 introduced stronger Goal–Plan–Loop instructions and successfully improved important parser behavior, including broad Tree-sitter language registration and a real old-tree incremental parse path.

However, the latest repository audit still showed a critical pattern:

```text
Document says:
    execute all mandatory closure work
        ↓
Agent executes:
    some real work
        ↓
Tests become green
        ↓
Agent/session stops
        ↓
Repository still contains open mandatory gaps
```

Execution 04 must not solve this by writing an even larger prose instruction and hoping the agent remembers every requirement.

Execution 04 changes the execution model.

The model is:

```text
PROMPT
  tells the agent what the complete objective is
        │
        ▼
CONTEXT
  gives the agent only the current relevant truth
        │
        ▼
STATE
  records exactly what is OPEN / ACTIVE / PASS / STALE / BLOCKED
        │
        ▼
LOOP
  forces continued repair while the current Goal is not proven
        │
        ▼
VERIFIER
  independently decides whether the Goal's required behavior is actually present
        │
        ├── FAIL → remain on Goal → diagnose → re-plan → repair → verify again
        │
        └── PASS → record proof → advance automatically
                              │
                              ▼
                     NEXT OPEN GOAL
                              │
                              ▼
                         OPEN = 0
                              │
                              ▼
                RERUN ALL VERIFIERS ON SAME SHA
                              │
                              ▼
                     FINAL SYSTEM E2E
                              │
                              ▼
                  frontend_readiness = READY
```

The important difference is:

> **The agent is not allowed to decide that a Goal is complete merely because it believes the implementation is good. The Goal is complete only when the corresponding behavioral verifier succeeds.**

---

# 1. Current Repository Truth to Preserve

Execution 04 must start by re-reading the current repository because the baseline SHA may have advanced.

At the baseline used to author this document, the following were already valuable and must not be rewritten without evidence:

- canonical identity foundations;
- evidence provenance;
- UPSM implementation-truth model;
- HAG human-abstraction model;
- persistence/store foundations;
- query and semantic-zoom foundations;
- local-first/no-LLM product invariant;
- portable path semantics;
- Windows/Linux/macOS core Rust CI;
- Tree-sitter production grammars for:
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
- explicit `Unknown` language behavior;
- Tree-sitter `InputEdit`;
- retained previous `Tree`;
- `parse(bytes, Some(&edited_tree))`;
- an incremental-vs-clean parse equivalence test.

Execution 04 is not allowed to throw these foundations away merely to simplify later goals.

---

# 2. Remaining Mandatory Gaps After Execution 03

At the latest audit, these gaps remained mandatory:

```text
GAP-01  Execution state/evidence does not match actual repository progress.
GAP-02  Framework semantics still use source.contains/string markers for deterministic facts.
GAP-03  Leiden production behavior is still connected-components baseline behavior.
GAP-04  Agglomerative implementation is not real agglomerative clustering.
GAP-05  HDBSCAN implementation is not real HDBSCAN.
GAP-06  NaiveBayes is not probabilistic Naive Bayes.
GAP-07  tree_rank is not a tree learner.
GAP-08  Node2Vec path is still a deterministic surrogate.
GAP-09  GCN path is not trainable.
GAP-10  GAT path is not trainable attention.
GAP-11  Heterogeneous GNN path is not a trainable typed graph model.
GAP-12  No genuine graph-learning training/checkpoint/reload pipeline exists.
GAP-13  No evidence-based model-retention gate has been completed.
GAP-14  Workflow/runtime truth needs final bounded behavioral verification.
GAP-15  Frontend-view contracts are not yet frozen as final v1 product contracts.
GAP-16  Production codeflowd does not exist.
GAP-17  Real HTTP API v1 does not exist.
GAP-18  SSE revision stream does not exist.
GAP-19  CLI commands are not wired to real backend behavior.
GAP-20  Security/resource guarantees have not been re-verified against the real service boundary.
GAP-21  Clean-machine packaging for the completed service is not proven.
GAP-22  CI does not yet test the complete product path.
GAP-23  Existing CI still invokes legacy harness diagnostics.
GAP-24  Execution 03 has no complete phase evidence/final report.
GAP-25  Final acceptance still targets old Execution 01-era behavior.
GAP-26  No same-SHA full-system frontend-readiness proof exists.
```

A Goal may close more than one gap where they are inseparable, but no gap may disappear from the state without a proof reference.

---

# 3. The Five Control Planes

Execution 04 is built around five distinct responsibilities.

## 3.1 Prompt Engineering

The prompt carries permanent intent:

- finish all mandatory Goals;
- do not reduce scope silently;
- do not stop at Goal boundaries;
- failures are loop input;
- evidence beats self-report;
- preserve architecture invariants;
- do not introduce a mandatory LLM;
- do not build the frontend yet.

The prompt does **not** contain the entire active implementation context.

## 3.2 Context Engineering

The agent loads only:

```text
Global invariants
+
Current Goal contract
+
Current state
+
Relevant production files
+
Relevant tests
+
Latest verifier failure
```

The agent must not repeatedly inject the entire Execution 04 document into every implementation turn.

## 3.3 Loop Engineering

For the active Goal:

```text
OBSERVE
→ REPRODUCE
→ PLAN
→ IMPLEMENT
→ VERIFY NARROWLY
→ VERIFY GOAL
→ FAIL?
   → ROOT CAUSE
   → RE-PLAN
   → REPAIR
   → LOOP
→ PASS?
   → RECORD PROOF
   → NEXT OPEN GOAL
```

## 3.4 State Engineering

The state is machine-readable.

The state answers:

```text
What is open?
What is active?
What passed?
What became stale?
What is genuinely blocked?
What exact SHA was used as proof?
What verifier proved the result?
```

## 3.5 Verification Engineering

Every Goal has an executable verifier.

The verifier must test behavior, not names.

Critical verifiers require **negative controls** that specifically fail the old known-bad implementation.

Example:

```text
A graph with two dense communities joined by one bridge

Connected components:
    1 cluster

A valid community-detection implementation:
    should be capable of separating the communities

Therefore:
    old connected-component surrogate must fail the verifier
```

---

# 4. No Harness / No Phase Runner

Execution 04 explicitly prohibits an execution controller that:

- advances Goals automatically by interpreting prose;
- exits after each Goal;
- waits for user approval;
- manages agent sessions;
- treats a phase boundary as a process boundary;
- decides completion from file existence.

Do not build:

```text
run_phase_01.py
run_phase_02.py
...
```

Do not build a master execution harness.

Allowed:

```text
Verifier scripts
State files
Test fixtures
Final verifier aggregator
CI jobs
```

The final verifier aggregator is not a harness.

It does one thing:

> rerun required behavioral proofs on the current final SHA and return success/failure.

It does not control the coding agent.

---

# 5. Required Execution 04 Repository Layout

At bootstrap, create:

```text
Execution/
  Execution 04.md

  exec04/
    goals.json
    state.json
    proof/
      G00.json
      G01.json
      ...
      G16.json
    failures/
      Gxx-last.json

scripts/
  exec04/
    common.py
    verify_G00_baseline.py
    verify_G01_parser.py
    verify_G02_frameworks.py
    verify_G03_graph_algorithms.py
    verify_G04_classical_ml.py
    verify_G05_graph_learning.py
    verify_G06_model_retention.py
    verify_G07_workflow_runtime.py
    verify_G08_frontend_contracts.py
    verify_G09_codeflowd.py
    verify_G10_http.py
    verify_G11_sse.py
    verify_G12_cli.py
    verify_G13_security_resources.py
    verify_G14_packaging.py
    verify_G15_product_ci_contract.py
    verify_G16_final_system.py
    verify_all.py
```

Names may adapt to repository conventions, but the separation of concerns must remain.

---

# 6. Goal Registry Contract

`Execution/exec04/goals.json` must contain one record per Goal.

Required shape:

```json
{
  "execution": "04",
  "schema_version": 1,
  "goals": [
    {
      "id": "G00",
      "title": "Baseline and state reconciliation",
      "status": "OPEN",
      "depends_on": [],
      "verifier": "scripts/exec04/verify_G00_baseline.py",
      "proof_sha": null,
      "proof_file": null
    }
  ]
}
```

Allowed states:

```text
OPEN
ACTIVE
PASS
STALE
BLOCKED
```

Meaning:

### OPEN

Not yet proven.

### ACTIVE

Current Goal.

### PASS

Verifier passed and proof is recorded.

### STALE

Previously passed, but later changes may invalidate the proof.

### BLOCKED

Only for a genuine external blocker.

Do not use:

```text
mostly_done
partial
almost_complete
ready_with_limitations
```

Those states are forbidden.

---

# 7. Proof Contract

A PASS Goal must produce:

```text
Execution/exec04/proof/Gxx.json
```

Required fields:

```json
{
  "goal": "Gxx",
  "status": "PASS",
  "proof_sha": "<git sha>",
  "verifier": "scripts/exec04/verify_Gxx_name.py",
  "verifier_exit_code": 0,
  "commands": [],
  "tests": [],
  "metrics": {},
  "negative_controls": [],
  "artifacts": [],
  "limitations": []
}
```

Rules:

1. `proof_sha` must equal the repository SHA tested.
2. Exit code must be `0`.
3. Known-bad negative controls must be recorded for critical Goals.
4. A manually written PASS with no verifier run is invalid.
5. If production code affecting a Goal changes later, the Goal may become `STALE`.
6. Final acceptance ignores historical PASS state and reruns all required verifiers on the final SHA.

---

# 8. Verifier Integrity Rules

A coding agent could theoretically make a weak verifier pass.

Execution 04 therefore protects verifier quality.

## 8.1 Never weaken acceptance silently

If a verifier assertion is removed or relaxed:

- record the change;
- explain why;
- prove the new test is equivalent or stricter;
- update the Goal contract only when the original requirement is genuinely impossible or incorrect.

## 8.2 Negative controls

Critical Goals require fixtures that the old bad implementation fails.

## 8.3 No existence-only checks

Forbidden:

```python
assert Path("crates/codeflowd").exists()
```

as proof that `codeflowd` works.

Required:

```text
spawn codeflowd
→ wait for health
→ analyze
→ commit revision
→ query
→ shutdown
→ restart
```

## 8.4 No name-only checks

Forbidden:

```python
assert hasattr(model, "gat")
```

Required:

```text
trainable_parameter_count > 0
loss executes
backward executes
optimizer step changes parameters
checkpoint reload works
```

## 8.5 Final rerun

`verify_all.py` must execute behavioral verifiers, not read old PASS JSON and trust them.

---

# 9. Continuous Execution Rule

Once Execution 04 begins:

```text
while OPEN + ACTIVE + STALE goals > 0:
    select first dependency-satisfied open/stale goal
    mark ACTIVE
    load current context
    run verifier to observe failure
    plan repair
    implement
    run targeted tests
    run verifier
    if fail:
        record failure
        root-cause
        re-plan
        continue same Goal
    if pass:
        record proof
        mark PASS
        continue immediately
```

This is conceptual behavior.

Do not implement it as a process-controlling harness.

The coding agent follows the rule directly.

---

# 10. Background Process Rule

Processes such as:

```text
codeflowd
HTTP service
SSE test service
watch process
training process
benchmark
```

must not become stopping conditions.

For each process:

- start it with a durable process handle;
- capture bounded logs;
- health-check readiness;
- continue independent tasks;
- poll only when needed;
- terminate when no longer needed;
- restart on recoverable failure.

Do not wait forever on stdout.

Do not stop Execution 04 because one terminal remains occupied.

---

# 11. Dependency Graph

Goals are:

```text
G00 Baseline + state reconciliation
 │
 ├── G01 Parser closure proof
 │     │
 │     └── G02 Parse-aware framework semantics
 │
 ├── G03 Real graph algorithms
 │
 ├── G04 Truthful classical ML
 │
 └── G05 Trainable graph learning
       │
       └── G06 Model retention gate

G01 + G02 + G03
       ↓
G07 Workflow/runtime truth
       ↓
G08 Frontend contracts freeze
       ↓
G09 codeflowd
       ↓
G10 HTTP API
       ↓
G11 SSE
       ↓
G12 CLI

G09 + G10 + G11 + G12
       ↓
G13 Security/resources
       ↓
G14 Packaging/clean machine
       ↓
G15 Product CI
       ↓
G16 Final same-SHA system acceptance
```

Independent Goals may proceed while a long-running benchmark/training task is active, as long as dependency constraints are respected.

---

# 12. G00 — Baseline and State Reconciliation

## Objective

Make Execution 04 begin from the actual repository rather than stale Execution 03 bookkeeping.

## Current known truth

At the authoring baseline, Execution 03 state did not reflect the parser work that was already present.

Execution 04 must correct that mismatch without rewriting history.

## Required work

1. Read current HEAD.
2. Inspect Execution 03 state/evidence.
3. Inspect current CI.
4. Inspect parser behavior.
5. Inspect all remaining known gaps.
6. Create Execution 04 registry/state.
7. Record already-present functionality as baseline facts, not as automatic Execution 04 PASS.
8. Run current direct test baseline.
9. Verify no mandatory uncommitted work is being mistaken for GitHub truth.

## Required verifier

`verify_G00_baseline.py`

Must prove:

```text
repo root valid
git SHA resolved
goals.json schema valid
all G00-G16 entries exist
every Goal has a verifier path
no Goal starts as PASS without proof
frontend_readiness starts NOT_READY
```

Also record:

```text
current Rust tests
current Python tests
current CI status
```

## Pass condition

Registry is complete and repository truth is reconciled.

Immediately continue to G01.

---

# 13. G01 — Parser Closure Proof

## Objective

Do not reimplement the parser if it is already correct.

Prove the parser improvements delivered after Execution 03.

## Required supported languages

```text
Rust
Python
TypeScript
JavaScript
Java
Kotlin
Go
C
C++
C#
```

## Required behavior

For each language:

- valid fixture parses;
- malformed fixture returns bounded syntax error evidence;
- normalized evidence exists;
- parser-local IDs do not become canonical CodeFlow IDs.

For incremental parsing:

```text
parse initial source
→ retain tree
→ apply explicit edit
→ edit previous tree
→ parse(final, Some(old_tree))
→ normalize evidence
```

Compare with:

```text
clean parse(final)
```

They must be equivalent at the CodeFlow evidence layer.

## Negative control

A test-only provider/path using:

```text
parse(final, None)
```

must not satisfy the incremental-reuse assertion.

The verifier should inspect behavior/instrumentation rather than merely grep source when feasible.

## Required verifier

`verify_G01_parser.py`

Must run parser tests and produce:

```json
{
  "languages_passed": 10,
  "incremental_equivalence": true,
  "old_tree_reuse_proven": true
}
```

## Pass condition

All ten languages and true incremental parse behavior pass.

Immediately continue.

---

# 14. G02 — Parse-Aware Framework Semantics

## Objective

Remove deterministic framework facts based solely on raw string containment.

## Current known bad pattern

Production behavior previously used logic equivalent to:

```rust
source.contains(marker)
```

followed by deterministic confidence.

That is forbidden as a deterministic semantic provider.

## Required architecture

Framework adapters operate on parsed syntax evidence or AST/query structures.

Representative support:

```text
Rust:
  Axum
  Actix

Python:
  FastAPI
  Flask
  Django

TypeScript/JavaScript:
  Express
  Nest
  statically recoverable Next server routes

Java/Kotlin:
  Spring
  Ktor

Go:
  net/http
  common static router forms

C#:
  ASP.NET
```

Every semantic must preserve:

```text
framework
kind
HTTP method where known
route where statically known
source span
provider
fact class
confidence
evidence id
```

Lexical markers may exist only as lower-authority inferred hints.

## Mandatory negative controls

These must NOT generate deterministic route facts:

```text
// app.get("/fake")
"@GetMapping('/fake')"
README example string
commented FastAPI decorator
identifier named app_get_example
```

## Required verifier

`verify_G02_frameworks.py`

Must include positive and deceptive-negative fixtures.

## Pass condition

No deterministic production framework fact can be produced solely from unparsed marker text.

Immediately continue.

---

# 15. G03 — Real Graph Algorithms

## Objective

End algorithm-name inflation permanently.

## Required result

Production semantics must truthfully distinguish:

```text
Connected Components
Leiden
Agglomerative Clustering
HDBSCAN
```

If the old Rust functions are useful as structural baselines, rename them truthfully:

```text
connected_components_baseline
degree_noise_baseline
```

Do not leave surrogate behavior behind a real algorithm name.

## Leiden requirement

Use a genuine Leiden implementation.

Acceptable:

- mature Python implementation through `igraph`/`leidenalg`;
- a maintained compatible Rust implementation;
- another reviewed mature implementation.

Must support:

- weighted graph where relevant;
- disconnected graph;
- seed/configuration;
- deterministic reproducibility under fixed configuration.

## Agglomerative requirement

Must implement real hierarchical agglomerative clustering with explicit:

```text
feature representation
distance metric
linkage
cluster selection
```

## HDBSCAN requirement

Must implement real hierarchical density clustering with:

```text
density hierarchy
minimum cluster size
noise/outlier semantics
```

## Critical negative fixtures

### Fixture A

Two dense subgraphs connected by one bridge.

Connected components returns one component.

The real community algorithm must have a configuration that recovers two communities.

### Fixture B

Different density groups + isolated noise.

A degree threshold is not enough.

Real HDBSCAN must expose actual cluster/noise behavior.

### Fixture C

Vectors arranged into known hierarchical groups.

Agglomerative result must differ from connected-components output.

## Required verifier

`verify_G03_graph_algorithms.py`

Required output includes:

```json
{
  "connected_components_distinguished_from_leiden": true,
  "agglomerative_is_hierarchical": true,
  "hdbscan_noise_semantics": true,
  "deterministic_fixed_seed": true
}
```

## Pass condition

Production names match real algorithms and old surrogates cannot pass the verifier.

Immediately continue.

---

# 16. G04 — Truthful Classical ML

## Objective

Make classical ML the trustworthy non-DL baseline.

## Naive Bayes

Use an actual probabilistic Naive Bayes model appropriate to the feature representation.

Possible:

```text
GaussianNB
MultinomialNB
BernoulliNB
```

Do not keep nearest-class-mean behavior under `NaiveBayes`.

## Logistic model

Use a stable real logistic baseline with:

```text
preprocessing
seed
regularization
convergence control
predict_proba
```

## Tree model

Replace linear weighted `tree_rank` behavior with a real tree-based learner, for example:

```text
DecisionTreeClassifier
RandomForestClassifier
HistGradientBoostingClassifier
```

or rename/remove `tree_rank` if it is genuinely just a linear ranking baseline.

## Dataset discipline

Prefer repository-level split:

```text
train repositories
validation repositories
test repositories
```

Avoid entity-level leakage between splits.

## Metrics

Record where applicable:

```text
macro F1
micro F1 / accuracy
per-class precision
per-class recall
PR-AUC
Brier score / calibration
inference latency
```

## Negative controls

The verifier must distinguish probabilistic NB from nearest-centroid behavior on a fixture where the decisions differ.

The verifier must distinguish a real tree model from a pure fixed linear ranker.

## Required verifier

`verify_G04_classical_ml.py`

## Pass condition

Every public model name is mathematically truthful and evaluation is reproducible.

Immediately continue.

---

# 17. G05 — Trainable Graph Learning

## Objective

Replace GNN-shaped arithmetic functions with real trainable graph models.

## Runtime isolation

Graph DL is optional.

Core CodeFlow must work without:

```text
PyTorch
PyTorch Geometric
CUDA
GPU
```

The ML-enabled environment may install them.

## Required real models

At minimum:

```text
Node2Vec or MetaPath2Vec-style trainable/real representation baseline
GAE + GCN encoder
GAE + GAT encoder
heterogeneous typed graph model
```

## Mandatory training semantics

A model qualifies only when all are real:

```text
trainable parameters > 0
forward pass
loss
zero_grad
backward
optimizer.step
validation/evaluation
seed
checkpoint save
checkpoint reload
```

## Graph semantics

Heterogeneous path must preserve meaningful relation types such as:

```text
CALLS
READS
WRITES
CONTROLS
IMPORTS
EMITS
LISTENS_TO
```

Do not collapse all types before the heterogeneous model.

## Negative control

Record parameters before and after optimizer step.

At least one parameter tensor must change.

A fixed arithmetic encoder must fail.

## Tiny learnability fixture

Must show one of:

```text
loss_after < loss_before
```

or:

```text
validation objective materially improves
```

under a deterministic tiny test.

## Required verifier

`verify_G05_graph_learning.py`

Required proof:

```json
{
  "trainable_parameters": true,
  "backward_executed": true,
  "optimizer_changed_parameters": true,
  "checkpoint_reload": true,
  "cpu_path": true,
  "typed_relations_used": true
}
```

## Pass condition

No GCN/GAT/Heterogeneous-GNN production claim points to a non-trainable arithmetic surrogate.

Immediately continue.

---

# 18. G06 — Model Retention Gate

## Objective

Decide whether graph DL deserves a default role.

## Pre-register before final held-out evaluation

Record:

```text
primary task
primary metric
split
seed strategy
latency budget
memory budget
improvement threshold
```

Default threshold unless a stronger repository-specific rule is justified:

```text
+0.03 absolute macro F1
OR
+0.05 absolute PR-AUC
```

over the strongest non-DL baseline.

## Compare

```text
deterministic graph baseline
classical ML
Node2Vec/representation baseline
GAE+GCN
GAE+GAT
heterogeneous model
```

## Valid outcome A

DL wins materially.

Then it may enter the default semantic proposal path.

## Valid outcome B

DL does not win materially.

Then:

```text
default = simpler baseline
DL = experimental/optional
```

This is a successful scientific result.

## Forbidden

Do not:

- change threshold after results;
- leak test data into tuning;
- make DL default because it is fashionable;
- hide resource cost.

## Required verifier

`verify_G06_model_retention.py`

Must confirm the protocol was frozen before final results and selection matches the protocol.

Immediately continue.

---

# 19. G07 — Workflow and Runtime Truth Closure

## Objective

Prove that workflow reconstruction preserves truth classes and remains bounded.

## Required distinctions

```text
DETERMINISTIC STATIC FACT
POSSIBLE
RECONSTRUCTED
OBSERVED
UNKNOWN
```

Static evidence must never become `OBSERVED`.

Runtime traces must never imply that unobserved branches are impossible.

## Fixtures

```text
linear path
branch success/error
nested calls
recursion
cycle
event emit/listen
queue-like continuation
static-only branch
runtime-only observed path
static/runtime contradiction
stale runtime revision
```

## Bounds

Verifier must assert explicit:

```text
max depth
max path count
cycle budget
timeout/cancellation
```

## Required verifier

`verify_G07_workflow_runtime.py`

## Pass condition

Truth classes and budgets are directly proven.

Immediately continue.

---

# 20. G08 — Freeze Frontend View Contract v1

## Objective

Create the backend contract the frontend can safely build against.

## Required schemas

At minimum:

```text
ProjectView
RevisionView
CapabilityView
GraphNodeView
GraphEdgeView
FeatureView
SubsystemView
WorkflowView
EvidenceView
SemanticZoomView
QueryResult
Cursor
ErrorEnvelope
PartialStatus
UnknownStatus
```

## Requirements

Every revision-sensitive response exposes:

```text
project_id
revision_id
schema/api version
```

Edges/nodes must expose relevant truth/evidence class.

Collections must serialize deterministically.

Cursors must not silently cross revisions.

Semantic zoom and graph responses must have budgets.

## Compatibility rule

After G08 passes:

> v1 is frozen.

Later breaking changes require:

```text
v2
```

not silent mutation.

## Required verifier

`verify_G08_frontend_contracts.py`

Must run:

```text
golden serialization
stable ordering
round-trip/schema validation
cursor revision mismatch negative test
unknown/partial status tests
```

Immediately continue.

---

# 21. G09 — Production `codeflowd`

## Objective

Create the real backend process.

## Preferred location

```text
crates/codeflowd/
```

Preferred implementation language: Rust.

## Required lifecycle

```text
START
→ READY
→ OPEN/CREATE PROJECT
→ ANALYZE
→ COMMIT REVISION
→ QUERY COMMITTED STATE
→ WATCH / REANALYZE
→ CANCEL
→ GRACEFUL SHUTDOWN
→ RESTART
→ RECOVER PERSISTED STATE
```

## Atomicity

During failed/in-progress analysis:

```text
previous committed revision remains readable
```

New revision is visible only after atomic commit.

## Networking

Default:

```text
loopback only
```

## Resource behavior

Bound:

```text
concurrent analyses
queue length
request lifetime
child analyzers
memory-sensitive graph tasks
```

## Required verifier

`verify_G09_codeflowd.py`

Behavioral process test:

```text
spawn real binary
wait for readiness
create temp project
analyze
obtain R1
query R1
cause failed/cancelled analysis
prove R1 remains readable
shutdown
restart
prove persisted state coherent
```

Existence of a binary alone fails.

Immediately continue.

---

# 22. G10 — Real HTTP API v1

## Objective

Expose production backend behavior to frontend/CLI clients.

## Preferred stack

Rust HTTP stack integrated with `codeflowd`, preferably Axum/Tokio unless repository evidence justifies another mature stack.

## Minimum endpoints

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

## Requirements

```text
typed error envelope
request size limit
response budget
timeout
revision consistency
cursor validation
capability errors
deterministic JSON
malformed payload handling
```

## OpenAPI

Generate or maintain a machine-readable OpenAPI v1 contract.

The running implementation and OpenAPI must be checked for drift.

## Required verifier

`verify_G10_http.py`

Must use real TCP/HTTP against a spawned daemon.

Direct Python `dispatch()` is not proof.

Immediately continue.

---

# 23. G11 — SSE Revision Stream

## Objective

Provide revision-driven frontend updates.

## Endpoint

Recommended:

```text
GET /v1/projects/{project_id}/events
```

## Event classes

```text
analysis_started
analysis_progress
revision_committed
analysis_completed
analysis_failed
capabilities_changed
```

## Required semantics

```text
monotonic event id
project id
revision id where relevant
Last-Event-ID resume
heartbeat/keepalive
bounded queue
slow-consumer behavior
disconnect cleanup
no raw source payload
```

Critical rule:

> `revision_committed` may be emitted only after the revision is queryable through the normal API.

## Required verifier

`verify_G11_sse.py`

Scenario:

```text
connect
→ analyze
→ receive R1
→ query R1
→ disconnect
→ create R2
→ reconnect from last event id
→ receive missed R2 event
→ query R2
```

Immediately continue.

---

# 24. G12 — Real CLI

## Objective

Make CLI commands perform real product behavior.

## Required commands

```text
codeflow init
codeflow analyze
codeflow watch
codeflow status
codeflow graph
codeflow evidence
```

## Preferred design

Thin client.

The CLI must not duplicate semantic engines.

Prefer:

```text
CLI → codeflowd API
```

or another single canonical service boundary.

## Behavior

### init

Create/register a real project.

### analyze

Trigger real analysis.

### watch

Observe real revisions.

### status

Return actual project/daemon state.

### graph

Return real view graph data.

### evidence

Resolve actual evidence.

## Required verifier

`verify_G12_cli.py`

Use the real executable/client against a real daemon.

Scenario:

```text
init
→ analyze
→ status
→ graph
→ select evidence id
→ evidence
→ watch
→ modify source
→ observe new revision
```

Immediately continue.

---

# 25. G13 — Security, Privacy, and Resource Closure

## Objective

Revalidate hardening against the service that now actually exists.

## Network

Prove:

```text
default bind = loopback
CORS/origin policy constrained
no unintended public bind
```

## Paths

Test:

```text
../ traversal
absolute Unix paths
Windows drive roots
UNC/device forms
symlink escape
unsafe SCIP/provider paths
```

## Subprocess

Prove:

```text
no unsafe shell interpolation
timeout
kill/reap
output cap
concurrency cap
```

## HTTP

Test:

```text
oversized request
malformed JSON
invalid cursor
stale revision
timeout
cancellation
queue saturation
```

## Privacy

Assert normal logs/SSE/errors do not contain raw repository source.

## ML

Bound:

```text
training graph size where applicable
batch size
inference timeout
checkpoint compatibility
```

## Required verifier

`verify_G13_security_resources.py`

Every fixed vulnerability pattern gets a regression test.

Immediately continue.

---

# 26. G14 — Packaging and Clean-Machine Proof

## Objective

Prove installation works outside the development checkout.

## Product profiles

### Core

Must not require:

```text
GPU
PyTorch
Joern
LLM API
cloud service
```

### ML-enabled

Adds optional ML/DL dependencies.

## Required supported systems

```text
Windows
Linux
macOS
```

## Required workflow

For a clean environment:

```text
install/build release
start codeflowd
run health
initialize fixture
analyze
query graph
run CLI command
shutdown
```

## Required verifier

`verify_G14_packaging.py`

Where full multi-OS local verification is not possible, CI clean runners provide the authoritative evidence.

Immediately continue.

---

# 27. G15 — Product CI Contract

## Objective

Make a green check mean that the real product works.

## Remove legacy execution authority

Current/legacy CI invocation of:

```text
python scripts/harness.py self-audit
```

must not remain a mandatory product acceptance authority.

It may be deleted or demoted to non-authoritative legacy diagnostics.

## Required base matrix

```text
ubuntu-latest
macos-latest
windows-latest
```

## Mandatory commands

At minimum:

```text
cargo fmt --check
cargo build --workspace --release
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features

python -m compileall -q python scripts
python -m unittest discover -s scripts/tests -v
python -m unittest discover -s python -p "test_*.py" -v
python scripts/no_llm_guard.py
```

## Mandatory product integration tests

CI must include direct proof for:

```text
G01 parser
G02 framework semantics
G03 graph algorithms
G04 classical ML
G07 workflow/runtime
G08 schemas
G09 daemon
G10 HTTP
G11 SSE
G12 CLI
G13 security/resource
G14 packaging smoke
```

## Graph DL CI

Use bounded CPU smoke/evaluation.

GPU is optional.

Do not require full expensive research training on all operating systems.

## Required verifier

`verify_G15_product_ci_contract.py`

This verifier checks the CI workflow itself contains the mandatory product-level jobs and that it does not rely on the old harness as proof.

The same final SHA must then receive green GitHub Actions.

Immediately continue.

---

# 28. G16 — Final Same-SHA System Acceptance

## Objective

Make this the only route to frontend readiness.

## Final verifier

Create:

```text
scripts/exec04/verify_all.py
```

It must:

1. resolve current SHA;
2. reject dirty production state for final acceptance;
3. run all mandatory Goal verifiers;
4. run the final system E2E;
5. confirm final SHA has required CI evidence or produce `PENDING_CI`, not PASS;
6. write final acceptance result.

It must **not** trust old proof JSON instead of rerunning behavior.

## Final end-to-end scenario

Use a multi-language fixture repository.

Required:

```text
install/start CodeFlow
        ↓
health ready
        ↓
initialize project
        ↓
analyze multi-language repository
        ↓
commit R1
        ↓
receive R1 over SSE
        ↓
query capabilities
        ↓
query graph
        ↓
query semantic view / zoom
        ↓
resolve evidence to source span
        ↓
modify source
        ↓
incremental parse/reanalysis
        ↓
commit R2
        ↓
receive/resume R2 event
        ↓
query R2
        ↓
prove R1 and R2 are revision-distinct
        ↓
shutdown
        ↓
restart
        ↓
recover latest committed state
        ↓
CLI status/graph/evidence against same backend
```

The test must assert meaningful semantic data.

HTTP 200 alone is insufficient.

## Final algorithm truth audit

Before PASS, assert no mandatory production path still exposes old false semantics:

```text
Leiden != connected components
Agglomerative != alias to Leiden/components
HDBSCAN != degree threshold
NaiveBayes != nearest centroid
tree learner != fixed linear weighted score
Node2Vec != hashed degree vector surrogate
GCN != arithmetic neighbor average
GAT != fixed multiplicative transform
Heterogeneous GNN != fixed node-type offset
```

## Final placeholder audit

Search mandatory production areas for:

```text
TODO
FIXME
placeholder
stub
surrogate
fake
not implemented
```

Each result is classified.

A mandatory runtime dependency on such a path fails acceptance.

## Final state

Only after:

```text
all Goal verifiers PASS on final SHA
+
final E2E PASS
+
required Windows/Linux/macOS CI PASS on same SHA
```

write:

```json
{
  "execution": "04",
  "accepted_sha": "<final sha>",
  "open_goals": 0,
  "stale_goals": 0,
  "blocked_goals": 0,
  "frontend_readiness": "READY"
}
```

Otherwise:

```json
{
  "frontend_readiness": "NOT_READY"
}
```

No intermediate wording.

---

# 29. State Mutation Rules

## 29.1 PASS is verifier-owned

The agent must not mark a Goal PASS before the verifier succeeds.

## 29.2 Later changes can stale earlier proof

If later work changes files/contracts relevant to a prior Goal:

```text
PASS → STALE
```

is allowed and encouraged.

Do not preserve a green badge for convenience.

## 29.3 Final verifier ignores stale history

The final verifier reruns behavior.

It does not infer readiness from historical state alone.

## 29.4 Blocked means genuinely blocked

Only a true external dependency can produce `BLOCKED`.

Difficult engineering is not blocked.

---

# 30. Failure Record

On every Goal verifier failure, write/update:

```text
Execution/exec04/failures/Gxx-last.json
```

Suggested shape:

```json
{
  "goal": "G03",
  "attempt": 4,
  "sha": "...",
  "verifier_exit_code": 1,
  "failure_class": "behavioral_mismatch",
  "observed": "...",
  "expected": "...",
  "root_cause_hypothesis": "...",
  "next_plan": [
    "..."
  ]
}
```

The purpose is context continuity.

It is not a failure counter used to stop execution.

There is no arbitrary “three failures then stop” rule.

---

# 31. Context Packet for Each Goal

Before implementing a Goal, construct an Active Context Packet containing only:

```text
1. Global invariants
2. Goal contract
3. Current Goal state
4. Dependency Goal proof summaries
5. Relevant production files
6. Relevant tests
7. Latest verifier output
8. Current short plan
```

Do not load:

```text
all previous execution documents
all historical reports
all unrelated source files
entire terminal history
```

unless needed to resolve a specific contradiction.

---

# 32. Goal Planning Format

Every active Goal uses a short plan.

Required format:

```text
Goal:
Current failure:
Root cause hypothesis:
Plan revision: N

Step 1
  Action:
  Boundary:
  Verification:

Step 2
  Action:
  Boundary:
  Verification:
```

Plan length should normally be 3–12 steps.

A plan must be replaced when evidence disproves its assumptions.

---

# 33. Loop Discipline

For each plan step:

```text
implement smallest coherent change
→ run targeted test
→ inspect actual failure/output
```

Before Goal closure:

```text
run relevant regression
→ run Goal verifier
```

On verifier failure:

```text
classify
→ find causal layer
→ update failure record
→ revise plan
→ continue
```

Do not:

- repeatedly apply the same patch;
- add duplicate implementations;
- disable the failing assertion;
- skip the Goal;
- move to later dependent Goals without documenting why they are independent.

---

# 34. Repair-First Engineering

Before creating a new subsystem, ask:

```text
Should the existing code be:
  corrected?
  replaced?
  renamed?
  deleted?
  consolidated?
```

Execution 04 should reduce semantic debt.

Examples:

Old:

```text
leiden_baseline = connected components
```

Good repair options:

```text
rename old function to connected_components_baseline
+
implement real Leiden
```

or:

```text
replace function with real Leiden
```

Bad:

```text
keep fake Leiden
+
add real_leiden_v2_new_final()
```

Do not stack obsolete architecture.

---

# 35. No Silent Capability Fallbacks

If a capability is optional and unavailable:

Return an explicit capability state.

Example:

```json
{
  "graph_learning": {
    "available": false,
    "reason": "optional_ml_dependencies_not_installed"
  }
}
```

Do not silently run an unrelated surrogate while returning the real algorithm's name.

---

# 36. No Test-Cheating Rule

Forbidden actions used solely to obtain PASS:

- lowering numeric thresholds after seeing results;
- deleting negative fixtures;
- changing expected output to match broken behavior without contract justification;
- marking integration tests skipped by default;
- using mocks in a final real-process test;
- replacing behavioral assertions with file existence checks;
- treating CI “success” on a different SHA as final proof.

Any required deviation must be explicitly documented and justified against the product requirement.

---

# 37. No LLM Product Dependency

The final product path remains:

```text
Code
→ parser/static analysis
→ UPSM
→ graph/ML abstraction
→ HAG/workflows
→ views/API
```

Execution 04 must not solve difficult semantic reconstruction by adding:

```text
OpenAI API
Anthropic API
Gemini API
hosted embedding API
mandatory local LLM
```

Coding agents may implement the repository.

That does not make an LLM a CodeFlow runtime dependency.

---

# 38. Definition of Frontend-Ready

Frontend development may begin only when:

```text
G00 PASS
G01 PASS
G02 PASS
G03 PASS
G04 PASS
G05 PASS
G06 PASS
G07 PASS
G08 PASS
G09 PASS
G10 PASS
G11 PASS
G12 PASS
G13 PASS
G14 PASS
G15 PASS
G16 PASS
```

and:

```text
all re-run on accepted SHA where applicable
+
Windows CI green
+
Linux CI green
+
macOS CI green
+
final E2E green
```

The status is binary:

```text
READY
NOT_READY
```

---

# 39. What Execution 04 Must Not Do

Do not:

- create a frontend;
- add unrelated product features;
- redesign UPSM/HAG without evidence;
- reimplement the parser that is already working unless verification finds a defect;
- add more supported languages before current mandatory closure;
- create a new execution harness;
- introduce cloud infrastructure as a workaround;
- add authentication/billing unrelated to local service safety;
- require GPU hardware;
- treat DL as mandatory when it does not outperform simpler methods;
- start Execution 05 while open Goals remain.

---

# 40. Mandatory Final Report

Create:

```text
Execution/Execution 04 Final Report.md
```

It must include:

```text
Accepted SHA
Date/time
Goal table G00-G16
Verifier result for every Goal
CI links/status for Windows/Linux/macOS
Parser language matrix
Incremental parse proof
Framework adapter matrix
Graph algorithm implementations
Classical ML model list
Graph DL model list
Model retention decision
Workflow truth classes
Frontend contract version
codeflowd lifecycle status
HTTP endpoint matrix
SSE reconnect result
CLI command matrix
Security/resource findings
Packaging matrix
Known optional capabilities
Known remaining limitations
Frontend readiness result
```

No claim in the report may exceed verifier evidence.

---

# 41. Final Execution Prompt

Use this as the launcher prompt for Claude Code, Codex, OpenCode, or another coding agent:

```text
Execute `Execution/Execution 04.md` as a deterministic closure execution against the current repository state.

Do not treat this as a planning-only task and do not reduce it to an MVP.

The execution model has five control planes:

PROMPT
- Preserve the global CodeFlow product invariants and complete all mandatory Goals.

CONTEXT
- Load only the active Goal, global invariants, current machine-readable state, dependency proof summaries, relevant production files/tests, and the latest verifier failure.
- Do not repeatedly load all historical execution documents.

STATE
- Create and maintain `Execution/exec04/goals.json` and `Execution/exec04/state.json`.
- Goal states are only OPEN, ACTIVE, PASS, STALE, or BLOCKED.
- PASS is allowed only after the Goal's verifier exits successfully.
- BLOCKED is allowed only for a genuine unavailable external dependency.
- If later work can invalidate earlier proof, mark that Goal STALE.
- Never set frontend_readiness to READY manually.

LOOP
For the active Goal:
1. run its verifier to observe the current failure;
2. inspect the repository and identify the root cause;
3. create a short concrete plan;
4. implement the smallest coherent repair;
5. run targeted tests;
6. run the Goal verifier;
7. if it fails, record the failure, revise the plan, and continue the same Goal;
8. if it passes, record proof and immediately activate the next dependency-satisfied OPEN or STALE Goal.

A failed build, failed test, failed benchmark, daemon crash, training failure, dependency conflict, or background process is not a stopping condition. It is loop input.

VERIFIER
- Every Goal must have the behavioral verifier required by Execution 04.
- Verifiers must use negative controls designed to reject the known old bad implementations.
- Never replace behavioral proof with function names, file names, comments, interface existence, mocks, TODO removal, or written reports.
- Do not weaken a verifier to make a broken implementation pass.
- The final verifier must rerun required behavior on the final SHA rather than trusting old PASS records.

NO HARNESS
- Do not use `scripts/harness.py` or any replacement phase runner as an execution controller.
- Evidence and state files are passive truth records.
- The final verifier is an acceptance checker, not a phase controller.

CURRENT REPOSITORY FACTS TO REVERIFY
- Multi-language Tree-sitter and a true old-tree incremental parse path were already added after Execution 03. Preserve them if they pass G01.
- The latest audit still found string-marker deterministic framework semantics, fake/mislabeled graph algorithms, non-probabilistic NaiveBayes, a non-tree `tree_rank`, deterministic GNN surrogates, no real graph-learning training pipeline, no completed model-retention gate, no codeflowd, no real HTTP API, no SSE stream, a metadata-only CLI, incomplete product CI, and no valid final frontend-readiness acceptance.
- Recheck all facts at current HEAD before acting.

Do not stop at Goal boundaries.
Do not ask me whether to continue.
Do not announce "ready for next Goal" and return control.
Move directly to the next Goal.

Background services/training processes must be health-checked and may continue while independent work proceeds. They are not stopping conditions.

Execution 04 is complete only when:
- all Goals G00-G16 pass;
- all required behavioral verifiers pass on the accepted final SHA;
- the final multi-language end-to-end scenario passes;
- Windows, Linux, and macOS mandatory CI are green on that same SHA;
- `Execution/Execution 04 Final Report.md` is truthful and complete;
- `frontend_readiness` is set to `READY` by final verified state.

If any mandatory Goal remains OPEN, ACTIVE, STALE, or BLOCKED:
Execution 04 is NOT complete.

Continue autonomously until the final acceptance gate is genuinely satisfied.
```

---

# 42. Final Transition Rule

There is no Execution 05 for ordinary missing backend work.

If Execution 04 finds a defect that belongs to one of its Goals:

```text
repair it inside Execution 04
```

Do not defer it into another execution simply because the repair is difficult.

Execution 05 may exist only for a genuinely new product capability that is not required for the current backend's frontend readiness.

The intended transition is:

```text
Execution 04
    ↓
all backend closure Goals proven
    ↓
accepted same-SHA backend
    ↓
frontend_readiness = READY
    ↓
freeze backend v1 contracts
    ↓
Frontend Execution 01
```

This is the backend closure boundary for the current CodeFlow architecture.
