# CodeFlow Execution 05 — Residual Closure & Final Backend Readiness

**Status:** READY FOR EXECUTION  
**Execution type:** Residual corrective closure / final backend convergence  
**Project:** CodeFlow  
**Execution lineage:** Execution 01 → Execution 02 → Execution 03 → Execution 04 → Execution 05  
**Primary purpose:** Close every remaining mandatory backend gap left open after Execution 04 and reach a directly proven frontend-ready backend.  
**Frontend implementation:** OUT OF SCOPE  
**Execution controller/harness:** PROHIBITED  
**LLM dependency in CodeFlow runtime:** PROHIBITED  
**GPU requirement for core product:** PROHIBITED  
**Normal completion condition:** All Execution 05 Goals pass on the accepted final SHA and `frontend_readiness = READY`  
**Ordinary session interruption:** NOT a completion condition  

---

# 0. Mission

Execution 05 exists because Execution 04 successfully introduced a better closure model but did not complete the technical work.

Execution 04 proved two important things:

1. CodeFlow can use machine-readable Goals, state, proofs, failures, and behavioral verifiers instead of relying only on prose.
2. The execution still stopped once it reached the first genuinely difficult open Goal.

Execution 05 therefore has one mission:

> **Resume from the exact residual state after Execution 04 and close every remaining mandatory backend gap without reopening already-proven parser work unless regression evidence demands it.**

Execution 05 must not become another architecture-planning exercise.

It must not invent a new product direction.

It must not create another partial scaffold and stop.

It must not declare success because CI is green while the product-level backend remains incomplete.

It must drive the repository to:

```text
Framework semantics         PASS
Graph algorithms            PASS
Classical ML                PASS
Graph learning              PASS
Model retention gate        PASS
Workflow/runtime truth      PASS
Frontend contract v1        PASS
codeflowd                    PASS
HTTP API v1                 PASS
SSE                         PASS
CLI                         PASS
Security/resources          PASS
Packaging                   PASS
Product CI                  PASS
Final same-SHA E2E          PASS
────────────────────────────────
frontend_readiness          READY
```

---

# 1. Residual Baseline from Execution 04

At the latest verified audit of Execution 04:

```text
G00  Baseline/state reconciliation     PASS
G01  Parser closure                    PASS
G02  Parse-aware frameworks            ACTIVE / FAIL
G03  Real graph algorithms             OPEN
G04  Truthful classical ML             OPEN
G05  Trainable graph learning          OPEN
G06  Model retention gate              OPEN
G07  Workflow/runtime truth            OPEN
G08  Frontend contract v1              OPEN
G09  codeflowd                         OPEN
G10  HTTP API v1                       OPEN
G11  SSE revision stream               OPEN
G12  Real CLI                          OPEN
G13  Security/resources                OPEN
G14  Packaging                         OPEN
G15  Product CI                        OPEN
G16  Final system acceptance           OPEN
```

Execution 05 must re-check the current repository before acting, because additional commits may have occurred.

The following already-proven capability should be preserved unless a direct regression is found:

```text
Multi-language Tree-sitter parsing       PROVEN
10 production grammars                   PROVEN
Malformed syntax recovery                PROVEN
Edited old-tree incremental parsing      PROVEN
Incremental/full evidence equivalence    PROVEN
Portable path semantics                  PROVEN
Core three-OS Rust CI                    PROVEN
```

Execution 05 does not reimplement these merely to appear productive.

---

# 2. Why Execution 05 Is Different

Execution 04 created a Goal registry and verifier framework, but most remaining verifiers were explicit placeholders that failed intentionally.

Execution 05 must replace those placeholders with real behavioral proofs and then close the underlying product gaps.

The execution model is:

```text
STATE
  ↓
SELECT FIRST NON-PASS GOAL
  ↓
RUN REAL VERIFIER
  ↓
FAILURE EVIDENCE
  ↓
GOAL
  ↓
PLAN
  ↓
IMPLEMENT
  ↓
TARGETED TEST
  ↓
REAL VERIFIER
  ↓
FAIL?
  ├─ YES → ROOT CAUSE → REPLAN → LOOP
  └─ NO  → PROOF → COMMIT/CHECKPOINT → NEXT NON-PASS GOAL
                                         ↓
                                  ALL GOALS PASS
                                         ↓
                                 FINAL SAME-SHA RUN
                                         ↓
                               WINDOWS/LINUX/MACOS CI
                                         ↓
                             frontend_readiness = READY
```

The agent is never allowed to convert:

```text
not implemented
```

into:

```text
complete
```

without behavioral evidence.

---

# 3. Execution 05 Control Planes

Execution 05 preserves the five control planes introduced in Execution 04.

## 3.1 Prompt

The prompt defines permanent rules:

- complete every Goal;
- never silently reduce scope;
- do not stop after a Goal;
- treat failures as loop input;
- preserve deterministic truth;
- no mandatory LLM;
- no frontend implementation;
- no false completion.

## 3.2 Context

For the active Goal, load only:

```text
Global invariants
Execution 05 state
Current Goal contract
Relevant previous proof
Relevant production files
Relevant tests
Current verifier
Latest failure record
Current short plan
```

Do not flood context with all historical Execution documents unless a contradiction requires them.

## 3.3 State

State answers:

```text
Which Goal is active?
Which Goals passed?
Which Goals became stale?
Which Goal is blocked?
What exact SHA proves each result?
What remains before frontend readiness?
```

## 3.4 Loop

Each Goal continuously runs:

```text
OBSERVE
→ REPRODUCE
→ PLAN
→ IMPLEMENT
→ TARGETED VERIFY
→ GOAL VERIFY
→ REPAIR
→ REVERIFY
```

until PASS.

## 3.5 Verifier

A Goal is complete only when its real behavioral verifier passes.

---

# 4. Stronger Resume Rule

Execution 05 explicitly supports session interruption.

A coding-agent session may end because of:

- context limit;
- tool limit;
- terminal loss;
- process interruption;
- platform session end;
- user closing the terminal;
- agent crash.

These are not successful completion.

At the beginning of **every new Execution 05 session**, the agent must:

```text
1. Read Execution/exec05/state.json
2. Read Execution/exec05/goals.json
3. Resolve current repository SHA
4. Find first Goal whose state is:
      ACTIVE
      OPEN
      STALE
5. Load that Goal only
6. Run its verifier
7. Resume from current repository state
```

The agent must not ask the user:

```text
Which Goal should I continue from?
```

unless state files are corrupt and cannot be reconstructed from repository evidence.

---

# 5. No Execution 06 Deferral Rule

Execution 05 is the backend closure execution.

If a mandatory backend requirement in this document is difficult, it remains inside Execution 05.

Do not create:

```text
Execution 06
```

for an unfinished Execution 05 requirement.

Execution 06 is permitted only for a genuinely new product capability after frontend readiness.

---

# 6. Repository State Layout

Create:

```text
Execution/
  CodeFlow_Execution_05.md

  exec05/
    goals.json
    state.json

    proof/
      E05-G00.json
      E05-G01.json
      ...
      E05-G14.json

    failures/
      E05-Gxx-last.json

    plans/
      E05-Gxx-plan.md

scripts/
  exec05/
    common.py
    verify_G00_frameworks.py
    verify_G01_graph_algorithms.py
    verify_G02_classical_ml.py
    verify_G03_graph_learning.py
    verify_G04_model_retention.py
    verify_G05_workflow_runtime.py
    verify_G06_frontend_contracts.py
    verify_G07_codeflowd.py
    verify_G08_http.py
    verify_G09_sse.py
    verify_G10_cli.py
    verify_G11_security_resources.py
    verify_G12_packaging.py
    verify_G13_product_ci.py
    verify_G14_final_system.py
    verify_all.py
```

Execution 05 begins from the first residual problem after Execution 04's proven parser work.

---

# 7. Execution 05 Goal Registry

`Execution/exec05/goals.json` must contain:

```json
{
  "execution": "05",
  "schema_version": 1,
  "goals": [
    {
      "id": "E05-G00",
      "title": "Parse-aware framework semantics",
      "status": "OPEN",
      "depends_on": [],
      "verifier": "scripts/exec05/verify_G00_frameworks.py"
    },
    {
      "id": "E05-G01",
      "title": "Real graph algorithms",
      "status": "OPEN",
      "depends_on": [],
      "verifier": "scripts/exec05/verify_G01_graph_algorithms.py"
    },
    {
      "id": "E05-G02",
      "title": "Truthful classical ML",
      "status": "OPEN",
      "depends_on": [],
      "verifier": "scripts/exec05/verify_G02_classical_ml.py"
    },
    {
      "id": "E05-G03",
      "title": "Trainable graph learning",
      "status": "OPEN",
      "depends_on": [],
      "verifier": "scripts/exec05/verify_G03_graph_learning.py"
    },
    {
      "id": "E05-G04",
      "title": "Model retention gate",
      "status": "OPEN",
      "depends_on": ["E05-G02", "E05-G03"],
      "verifier": "scripts/exec05/verify_G04_model_retention.py"
    },
    {
      "id": "E05-G05",
      "title": "Workflow/runtime truth closure",
      "status": "OPEN",
      "depends_on": ["E05-G00", "E05-G01"],
      "verifier": "scripts/exec05/verify_G05_workflow_runtime.py"
    },
    {
      "id": "E05-G06",
      "title": "Frontend contract v1 freeze",
      "status": "OPEN",
      "depends_on": ["E05-G05"],
      "verifier": "scripts/exec05/verify_G06_frontend_contracts.py"
    },
    {
      "id": "E05-G07",
      "title": "Production codeflowd",
      "status": "OPEN",
      "depends_on": ["E05-G06"],
      "verifier": "scripts/exec05/verify_G07_codeflowd.py"
    },
    {
      "id": "E05-G08",
      "title": "HTTP API v1",
      "status": "OPEN",
      "depends_on": ["E05-G07"],
      "verifier": "scripts/exec05/verify_G08_http.py"
    },
    {
      "id": "E05-G09",
      "title": "SSE revision stream",
      "status": "OPEN",
      "depends_on": ["E05-G08"],
      "verifier": "scripts/exec05/verify_G09_sse.py"
    },
    {
      "id": "E05-G10",
      "title": "Real CLI integration",
      "status": "OPEN",
      "depends_on": ["E05-G09"],
      "verifier": "scripts/exec05/verify_G10_cli.py"
    },
    {
      "id": "E05-G11",
      "title": "Security/privacy/resource closure",
      "status": "OPEN",
      "depends_on": ["E05-G07", "E05-G08", "E05-G09", "E05-G10"],
      "verifier": "scripts/exec05/verify_G11_security_resources.py"
    },
    {
      "id": "E05-G12",
      "title": "Packaging and clean-machine proof",
      "status": "OPEN",
      "depends_on": ["E05-G11"],
      "verifier": "scripts/exec05/verify_G12_packaging.py"
    },
    {
      "id": "E05-G13",
      "title": "Product-level CI",
      "status": "OPEN",
      "depends_on": ["E05-G12"],
      "verifier": "scripts/exec05/verify_G13_product_ci.py"
    },
    {
      "id": "E05-G14",
      "title": "Final same-SHA backend acceptance",
      "status": "OPEN",
      "depends_on": ["E05-G04", "E05-G13"],
      "verifier": "scripts/exec05/verify_G14_final_system.py"
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

No other status is allowed.

---

# 8. State Contract

Create:

```text
Execution/exec05/state.json
```

Initial form:

```json
{
  "execution": "05",
  "current_goal": "E05-G00",
  "frontend_readiness": "NOT_READY",
  "accepted_sha": null,
  "last_verified_sha": null,
  "open_goals": 15,
  "blocked_goals": [],
  "notes": []
}
```

Rules:

- `frontend_readiness` cannot be set to READY until E05-G14 passes.
- `accepted_sha` remains null until final acceptance.
- State records truth; it does not control the agent.
- PASS without verifier proof is invalid.
- A later change may mark earlier Goal STALE.

---

# 9. Proof Contract

Each PASS writes:

```text
Execution/exec05/proof/E05-Gxx.json
```

Required:

```json
{
  "goal": "E05-Gxx",
  "status": "PASS",
  "proof_sha": "<sha>",
  "verifier": "...",
  "verifier_exit_code": 0,
  "commands": [],
  "tests": [],
  "negative_controls": [],
  "metrics": {},
  "artifacts": [],
  "limitations": []
}
```

A limitation cannot waive a mandatory acceptance condition.

---

# 10. Failure Contract

Each failed verifier updates:

```text
Execution/exec05/failures/E05-Gxx-last.json
```

Required:

```json
{
  "goal": "E05-Gxx",
  "sha": "<sha>",
  "attempt": 1,
  "failure_class": "...",
  "expected": "...",
  "observed": "...",
  "root_cause": "...",
  "next_plan": []
}
```

There is no maximum retry count.

Failure means continue.

---

# 11. Verifier Rules

Every verifier must be real before its Goal may become ACTIVE for implementation closure.

Do not copy Execution 04's placeholder pattern:

```python
return fail("verifier not implemented")
```

and then stop.

For each Goal:

1. implement the behavioral verifier first or alongside the first repair;
2. prove the old known-bad implementation fails it;
3. repair production code;
4. rerun until PASS.

No existence-only proof.

No function-name proof.

No report-only proof.

---

# 12. E05-G00 — Parse-Aware Framework Semantics

## Goal

Replace deterministic raw-string framework detection with parse-aware semantics.

## Current known bad behavior

Historically:

```rust
source.contains(marker)
```

could produce:

```text
FactClass::Deterministic
confidence = 1.0
```

This is forbidden.

## Required implementation

Use parsed syntax / AST / Tree-sitter query structure.

Supported representative ecosystems:

```text
Rust
  Axum
  Actix

Python
  FastAPI
  Flask
  Django

TypeScript/JavaScript
  Express
  Nest
  statically recoverable Next server routes

Java/Kotlin
  Spring
  Ktor

Go
  net/http
  common router patterns

C#
  ASP.NET
```

Every framework semantic must provide:

```text
framework
kind
route where known
method where known
source span
provider
fact class
confidence
evidence id
```

## Negative controls

These must fail to generate deterministic route facts:

```text
comment: // app.get("/fake")
string: "@GetMapping('/fake')"
README-style string literal
commented Python decorator
identifier called app_get_example
unrelated method called get
```

## Positive controls

Real parsed routes must be detected.

## Verifier

`verify_G00_frameworks.py`

Must:

- build/run production framework extraction;
- test positive fixtures;
- test deceptive negatives;
- prove deterministic facts require parse-aware evidence.

## Pass

No deterministic framework route depends solely on raw `source.contains`.

---

# 13. E05-G01 — Real Graph Algorithms

## Goal

Replace misleading structural surrogates with real algorithms.

## Required production distinction

```text
Connected Components
Leiden
Agglomerative
HDBSCAN
```

must be distinct implementations.

## Connected Components

The old weakly-connected behavior may remain only under truthful naming:

```text
connected_components_baseline
```

## Leiden

Use genuine Leiden.

Must support:

```text
weighted edges where used
seed/config
disconnected graphs
stable serialization
```

## Agglomerative

Use genuine hierarchical agglomerative clustering.

Declare:

```text
feature representation
distance metric
linkage
cluster selection strategy
```

## HDBSCAN

Use genuine HDBSCAN.

Must expose:

```text
cluster labels
noise/outliers
min cluster size
density hierarchy semantics
```

## Negative fixtures

Fixture A:

```text
dense A -- weak bridge -- dense B
```

Connected components => one component.

Real Leiden must be capable of producing two communities.

Fixture B:

```text
dense region A
dense region B
isolated/noise points
```

Real HDBSCAN must identify noise in a way a degree threshold cannot trivially emulate.

Fixture C:

Vector hierarchy with known cluster structure for Agglomerative.

## Verifier

`verify_G01_graph_algorithms.py`

Must fail old:

```text
leiden_baseline = connected components
agglomerative = leiden baseline
hdbscan = degree threshold
```

## Pass

Names match semantics and negative controls reject old behavior.

---

# 14. E05-G02 — Truthful Classical ML

## Goal

Build mathematically correct classical baselines.

## Naive Bayes

Select the correct probabilistic variant for the feature contract:

```text
GaussianNB
MultinomialNB
BernoulliNB
```

Nearest-centroid behavior is forbidden under the NaiveBayes name.

## Logistic Regression

Use a stable implementation with:

```text
seed
regularization
convergence
predict_proba
preprocessing contract
```

## Tree Learner

Replace fixed weighted linear rank with:

```text
DecisionTree
RandomForest
HistGradientBoosting
```

or rename the old score truthfully and add a real tree learner separately.

## Data splitting

Prefer repository/project-separated train/validation/test.

Avoid entity-level leakage.

## Metrics

Record:

```text
macro F1
micro F1 or accuracy
per-class precision
per-class recall
PR-AUC where relevant
Brier/calibration
inference latency
```

## Negative controls

Create a fixture where nearest-centroid prediction differs from probabilistic NB.

Create a fixture where a nonlinear tree decision outperforms a fixed linear rank.

## Verifier

`verify_G02_classical_ml.py`

## Pass

All public algorithm names are truthful and evaluation is reproducible.

---

# 15. E05-G03 — Real Trainable Graph Learning

## Goal

Create actual trainable graph representation models.

## Optional dependency boundary

Core CodeFlow must run without:

```text
PyTorch
PyG
CUDA
GPU
```

Graph learning is an optional ML-enabled capability.

## Required models

At minimum:

```text
real Node2Vec or MetaPath2Vec-style representation baseline
GAE + GCN
GAE + GAT
heterogeneous typed graph model
```

## Required training semantics

Every trainable model must have:

```text
parameters requiring gradients
forward
loss
zero_grad
backward
optimizer.step
epochs
evaluation
seed
checkpoint save
checkpoint reload
```

## Typed graph requirement

Heterogeneous model must preserve relation types such as:

```text
CALLS
READS
WRITES
CONTROLS
IMPORTS
EMITS
LISTENS_TO
```

Do not collapse them before the hetero layer.

## Tiny learnability test

Must demonstrate:

```text
loss_after < loss_before
```

or a predefined equivalent learning objective improvement.

## Parameter-change negative control

Capture trainable parameters before and after optimizer step.

At least one must change.

The old arithmetic encoders must fail.

## Verifier

`verify_G03_graph_learning.py`

## Pass

Graph-learning claims correspond to actual training.

---

# 16. E05-G04 — Model Retention Gate

## Goal

Decide scientifically whether DL belongs in the default CodeFlow path.

## Pre-register

Before final held-out result:

```text
primary task
primary metric
data split
seed strategy
latency budget
memory budget
material-improvement threshold
```

Default:

```text
+0.03 macro F1
OR
+0.05 PR-AUC
```

over strongest non-DL baseline.

## Compare

```text
deterministic baseline
classical ML
Node2Vec representation
GAE + GCN
GAE + GAT
heterogeneous model
```

## Valid outcomes

### A — DL wins

DL may enter default proposal/ranking path.

### B — DL does not win

DL becomes:

```text
optional
experimental
non-default
```

This is acceptable.

## Verifier

`verify_G04_model_retention.py`

Must confirm:

- protocol existed before final evaluation;
- selection follows the protocol;
- resource cost is included;
- no test leakage.

---

# 17. E05-G05 — Workflow and Runtime Truth Closure

## Goal

Prove workflow reconstruction preserves evidence truth and resource bounds.

## Required states

```text
DETERMINISTIC
POSSIBLE
RECONSTRUCTED
OBSERVED
UNKNOWN
```

## Critical invariant

Static analysis may never generate `OBSERVED`.

Runtime observation may never imply unobserved branch impossibility.

## Fixtures

```text
linear workflow
success/error branch
nested calls
recursion
cycle
event emit/listen
async continuation
static-only branch
runtime-only path
static/runtime contradiction
stale runtime revision
```

## Bounds

Require:

```text
max depth
max paths
cycle limit
timeout/cancellation
```

## Verifier

`verify_G05_workflow_runtime.py`

## Pass

Truth classes and bounded behavior are directly proven.

---

# 18. E05-G06 — Frontend Contract v1 Freeze

## Goal

Freeze the backend contract the frontend will build against.

## Required view types

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

## Every revision-sensitive response includes

```text
project_id
revision_id
schema/api version
```

## Required properties

```text
stable IDs
deterministic ordering
truth/evidence class
pagination
revision-safe cursors
query budgets
semantic zoom budgets
versioned errors
```

## Freeze rule

After PASS:

```text
v1 breaking change forbidden
```

Breaking changes require v2.

## Verifier

`verify_G06_frontend_contracts.py`

Must test:

```text
golden serialization
stable ordering
schema validation
cursor/revision mismatch
unknown/partial cases
```

---

# 19. E05-G07 — Production codeflowd

## Goal

Create the real local backend daemon.

## Preferred implementation

Rust binary:

```text
crates/codeflowd/
```

## Required lifecycle

```text
start
ready
create/open project
analyze
commit revision
query revision
watch/reanalyze
cancel
graceful shutdown
restart
recover persisted state
```

## Atomic revision rule

If analysis fails:

```text
previous committed revision remains readable
```

## Default networking

```text
loopback only
```

## Resource bounds

Bound:

```text
analysis concurrency
queue size
child analyzers
query lifetime
shutdown timeout
```

## Verifier

`verify_G07_codeflowd.py`

Real process test:

```text
spawn
→ health
→ temp project
→ analyze
→ R1
→ query R1
→ force failed/cancelled analysis
→ prove R1 still readable
→ shutdown
→ restart
→ recover state
```

File existence does not pass.

---

# 20. E05-G08 — HTTP API v1

## Goal

Expose production service behavior through real HTTP.

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
typed errors
request size bound
response bound
timeouts
revision consistency
cursor validation
capability errors
deterministic JSON
malformed request handling
```

## OpenAPI

Maintain machine-readable OpenAPI v1.

Check drift between implementation and contract.

## Verifier

`verify_G08_http.py`

Use actual TCP HTTP against real `codeflowd`.

Python direct dispatch is not enough.

---

# 21. E05-G09 — SSE Revision Stream

## Goal

Provide a reliable revision stream.

## Endpoint

Recommended:

```text
GET /v1/projects/{project_id}/events
```

## Events

```text
analysis_started
analysis_progress
revision_committed
analysis_completed
analysis_failed
capabilities_changed
```

## Requirements

```text
monotonic event id
project id
revision id
Last-Event-ID
heartbeat
bounded queue
slow-consumer policy
disconnect cleanup
no source leakage
```

## Critical ordering

`revision_committed` may be emitted only after the revision is queryable.

## Verifier

`verify_G09_sse.py`

Scenario:

```text
connect
→ analyze
→ receive R1
→ query R1
→ disconnect
→ create R2
→ reconnect with Last-Event-ID
→ receive missed R2
→ query R2
```

---

# 22. E05-G10 — Real CLI Integration

## Goal

Make CLI commands perform actual backend operations.

## Required commands

```text
codeflow init
codeflow analyze
codeflow watch
codeflow status
codeflow graph
codeflow evidence
```

## Preferred architecture

```text
CLI → codeflowd API
```

No second semantic engine in CLI.

## Verifier

`verify_G10_cli.py`

Scenario:

```text
init
→ analyze
→ status
→ graph
→ pick evidence id
→ evidence
→ watch
→ edit file
→ observe new revision
```

Metadata-only printing fails.

---

# 23. E05-G11 — Security, Privacy, Resource Closure

## Goal

Verify real service boundaries after daemon/API/SSE/CLI exist.

## Network

Prove:

```text
loopback default
controlled bind override
restricted CORS/origin
```

## Paths

Test:

```text
../
Unix absolute
Windows drive
UNC/device
symlink escape
unsafe provider path
```

## Process safety

Prove:

```text
no unsafe shell interpolation
timeout
kill/reap
stdout/stderr bound
process-count bound
```

## API

Test:

```text
oversized request
malformed JSON
invalid cursor
stale revision
timeout
cancel
queue saturation
```

## Privacy

Normal logs/events/errors must not leak repository source text.

## Verifier

`verify_G11_security_resources.py`

Every discovered high-severity issue requires a regression test.

---

# 24. E05-G12 — Packaging and Clean-Machine Proof

## Goal

Prove CodeFlow works outside the development checkout.

## Core profile

Must not require:

```text
GPU
PyTorch
Joern
LLM
cloud service
```

## ML profile

Adds optional ML/DL dependencies.

## Supported systems

```text
Windows
Linux
macOS
```

## Required clean workflow

```text
build/install release
start daemon
health check
init project
analyze
query
CLI smoke
shutdown
```

## Verifier

`verify_G12_packaging.py`

CI clean runners may provide multi-OS proof.

---

# 25. E05-G13 — Product-Level CI

## Goal

Make green CI represent the product, not only compilation.

## Remove legacy harness authority

Current legacy:

```text
python scripts/harness.py self-audit
```

must no longer be a mandatory acceptance authority.

It may remain as non-authoritative legacy diagnostics or be removed.

## OS matrix

```text
ubuntu-latest
macos-latest
windows-latest
```

## Core mandatory commands

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

## Product tests

CI must exercise:

```text
framework semantics
graph algorithms
classical ML
workflow/runtime
frontend contracts
daemon
HTTP
SSE
CLI
security/resource
packaging smoke
```

Graph DL may use bounded CPU smoke in CI.

## Verifier

`verify_G13_product_ci.py`

Must inspect CI contract and then require green workflow on final candidate SHA.

---

# 26. E05-G14 — Final Same-SHA Backend Acceptance

## Goal

Make this the only route to READY.

## Final aggregator

Create:

```text
scripts/exec05/verify_all.py
```

It must:

1. resolve SHA;
2. reject dirty production state;
3. rerun all Execution 05 behavioral verifiers;
4. re-run parser regression verifier from Execution 04;
5. run final E2E;
6. check final same-SHA CI;
7. write final state.

It must not trust historical PASS JSON alone.

## Full end-to-end scenario

Use a multi-language fixture repository.

Required:

```text
start CodeFlow
→ health
→ initialize project
→ analyze
→ commit R1
→ receive R1 over SSE
→ query capabilities
→ query graph
→ semantic zoom
→ resolve evidence to source
→ edit source
→ incremental reanalysis
→ commit R2
→ receive/resume R2
→ query R2
→ prove R1/R2 isolation
→ CLI status/graph/evidence
→ shutdown
→ restart
→ recover latest state
```

## Mandatory semantic assertions

Do not accept HTTP 200 alone.

Assert:

```text
non-empty meaningful graph
stable IDs
revision IDs
evidence links
truth classes
feature/workflow views
```

## Final anti-surrogate audit

Fail if mandatory production code still exposes these old semantics:

```text
Leiden = connected components
Agglomerative = Leiden/components alias
HDBSCAN = degree threshold
NaiveBayes = nearest centroid
Tree learner = fixed linear rank
Node2Vec = hashed degree surrogate
GCN = arithmetic neighbor averaging
GAT = fixed multiplicative transform
Hetero GNN = fixed node-type offset
```

## Final parser regression

Re-run the proven Execution 04 parser tests:

```text
10 languages
malformed recovery
edited old-tree incremental parse
incremental/full equivalence
```

## Final CI rule

The exact accepted SHA must have green mandatory:

```text
Windows
Linux
macOS
```

## Final state

Only if everything passes:

```json
{
  "execution": "05",
  "open_goals": 0,
  "stale_goals": 0,
  "blocked_goals": 0,
  "accepted_sha": "<sha>",
  "frontend_readiness": "READY"
}
```

Anything else:

```json
{
  "frontend_readiness": "NOT_READY"
}
```

---

# 27. Mandatory Commit/Checkpoint Policy

Execution 05 must avoid the previous problem where proof referred to a working tree rather than the committed repository.

After a Goal passes:

1. ensure production/test changes are coherent;
2. commit or create an explicit checkpoint commit before treating the proof as durable;
3. rerun the Goal verifier against the committed SHA when feasible;
4. store that SHA in proof JSON.

Final acceptance always ignores old SHA proof and reruns everything on the final candidate SHA.

---

# 28. Stale-Proof Policy

If later work modifies:

```text
parser
evidence schema
UPSM
HAG
workflow model
view contracts
daemon lifecycle
API schemas
revision semantics
ML feature schema
graph representation
```

then earlier dependent Goals may become:

```text
STALE
```

A stale Goal must be rerun before final acceptance.

Do not preserve PASS cosmetically.

---

# 29. Background Process Policy

Long-running processes are normal.

Examples:

```text
codeflowd
SSE stream
watch
benchmark
ML training
CI polling
```

Rules:

- keep process handle;
- capture bounded logs;
- health-check;
- continue independent work;
- use bounded polling;
- restart recoverable failures;
- clean up at the appropriate lifecycle boundary.

A busy terminal is not a reason to stop Execution 05.

---

# 30. Plan Format

Every Goal keeps:

```text
Execution/exec05/plans/E05-Gxx-plan.md
```

Required:

```text
# Goal
# Current Failure
# Root Cause Hypothesis
# Plan Revision

## Step 1
Action:
Boundary:
Verification:

## Step 2
Action:
Boundary:
Verification:
```

Plan revisions replace disproven assumptions.

Do not follow a stale plan just because it exists.

---

# 31. Context Recovery After Session Loss

If a session restarts:

```text
read state
→ find ACTIVE/Open first dependency-satisfied goal
→ read latest failure
→ read current plan
→ inspect current git diff/SHA
→ rerun verifier
→ continue
```

Never rely on conversational memory to know where execution stopped.

---

# 32. No False Blockers

These are not blockers:

```text
test failure
compile failure
dependency conflict
algorithm bug
poor benchmark
training instability
daemon crash
SSE reconnect bug
CI failure
cross-platform bug
large refactor
```

They require repair.

True blocker means an objectively unavailable external requirement.

---

# 33. No Silent Scope Reduction

If a requirement appears expensive or difficult:

Do not:

```text
rename it
mock it
skip it
weaken its verifier
mark it optional
```

unless the product contract itself permits optionality.

Any scope change must be explicit, justified, versioned, and reflected in capabilities.

---

# 34. No LLM Runtime Dependency

Do not introduce:

```text
OpenAI API
Anthropic API
Gemini API
hosted embedding API
mandatory local LLM
```

to close semantic gaps.

The runtime remains:

```text
Code
→ deterministic analysis
→ UPSM
→ graph/ML inference
→ HAG/workflow
→ views/service
```

---

# 35. Definition of Done

Execution 05 is complete only when:

```text
E05-G00 PASS
E05-G01 PASS
E05-G02 PASS
E05-G03 PASS
E05-G04 PASS
E05-G05 PASS
E05-G06 PASS
E05-G07 PASS
E05-G08 PASS
E05-G09 PASS
E05-G10 PASS
E05-G11 PASS
E05-G12 PASS
E05-G13 PASS
E05-G14 PASS
```

and:

```text
Execution 04 parser regression PASS
same-SHA Windows CI PASS
same-SHA Linux CI PASS
same-SHA macOS CI PASS
final E2E PASS
Execution 05 Final Report present
frontend_readiness = READY
```

---

# 36. Mandatory Final Report

Create:

```text
Execution/Execution 05 Final Report.md
```

Must include:

```text
Accepted SHA
Execution start/end timestamps
Goal table
Verifier result per Goal
Parser regression matrix
Framework support matrix
Graph algorithm implementation summary
Classical ML models
Graph learning models
Model retention decision
Workflow/runtime truth behavior
Frontend contract version
codeflowd lifecycle
HTTP endpoint matrix
SSE reconnect result
CLI command matrix
Security/resource findings
Packaging matrix
Windows/Linux/macOS CI status
Known optional capabilities
Known limitations
Frontend readiness
```

Claims cannot exceed evidence.

---

# 37. Final Execution Prompt

Use this exact launcher for Codex, Claude Code, OpenCode, or another capable coding agent:

```text
Execute `Execution/CodeFlow_Execution_05.md` against the current CodeFlow repository state.

This is a residual closure execution, not a planning task and not an MVP.

Do not redo already-proven parser work unless its regression verifier fails.

At the beginning of this session:
1. read `Execution/exec05/state.json` if it exists;
2. read `Execution/exec05/goals.json`;
3. resolve current git SHA and working tree;
4. find the first Goal that is ACTIVE, OPEN, or STALE and whose dependencies are satisfied;
5. load only that Goal, its verifier, latest failure record, plan, relevant production files, and relevant tests;
6. run its verifier before making changes.

For each Goal:

GOAL
- Use the Goal contract in Execution 05 as the observable target.

PLAN
- Maintain a short repository-grounded plan in `Execution/exec05/plans/`.
- Replace the plan when evidence disproves it.

LOOP
- Reproduce the failure.
- Implement the smallest coherent repair.
- Run targeted verification.
- Run the Goal verifier.
- If it fails, record the failure, root-cause it, revise the plan, and continue the same Goal.
- A build failure, test failure, algorithm failure, poor benchmark, daemon crash, dependency conflict, CI failure, training instability, or background process is loop input, not a stopping condition.

VERIFY
- Every Goal requires a real behavioral verifier.
- Do not use placeholder verifiers.
- Do not use existence-only, name-only, report-only, or mock-only proof for final behavior.
- Preserve negative controls that reject the known old bad implementations.
- Never weaken a verifier merely to make production code pass.

STATE
- Goal states are only OPEN, ACTIVE, PASS, STALE, BLOCKED.
- PASS is allowed only after the Goal verifier passes.
- BLOCKED is only for a genuinely unavailable external requirement.
- If later changes may invalidate an earlier Goal, mark it STALE.
- Do not set `frontend_readiness` to READY manually.

CHECKPOINT
- After a Goal passes, create a coherent commit/checkpoint when feasible.
- Prefer proof against a committed SHA rather than an uncommitted working tree.
- Immediately continue to the next dependency-satisfied Goal.

RESUME
- If the session is interrupted, the next run must resume from Execution 05 state automatically.
- Do not ask which Goal to continue from when state can answer it.

NO HARNESS
- Do not create or use an execution harness/phase runner as a controller.
- State and proof files are passive records.
- `verify_all.py` is a final acceptance checker only.

NO EXECUTION 06 DEFERRAL
- Do not defer any mandatory Execution 05 backend requirement to Execution 06.
- Repair it inside Execution 05.

NO FRONTEND
- Do not implement production frontend UI in this execution.

NO LLM RUNTIME
- Do not introduce mandatory LLM or hosted embedding dependencies into CodeFlow.

Current residual areas to close include:
- parse-aware framework semantics;
- genuine Leiden, Agglomerative, and HDBSCAN;
- truthful probabilistic classical ML and real tree learner;
- real trainable graph learning;
- scientific model-retention gate;
- bounded workflow/runtime truth;
- frozen frontend v1 contracts;
- production codeflowd;
- real HTTP v1;
- SSE revision streaming;
- real CLI integration;
- service security/privacy/resource hardening;
- clean packaging;
- product-level cross-platform CI;
- final same-SHA end-to-end acceptance.

Do not stop after a Goal passes.
Do not ask for permission to continue.
Do not return "ready for next Goal".
Advance immediately.

Normal completion is allowed only when:
- E05-G00 through E05-G14 all PASS;
- all required behavioral verifiers rerun successfully on the final candidate;
- Execution 04 parser regression still passes;
- the final end-to-end scenario passes;
- Windows, Linux, and macOS mandatory CI are green on the same accepted SHA;
- `Execution/Execution 05 Final Report.md` is complete and truthful;
- final verified state is `frontend_readiness = READY`.

If any mandatory Goal remains OPEN, ACTIVE, STALE, or BLOCKED, Execution 05 is not complete.

Continue autonomously until final backend acceptance is genuinely proven.
```

---

# 38. Final Transition

When Execution 05 passes:

```text
CodeFlow backend closure complete
        ↓
v1 service/view contracts frozen
        ↓
frontend_readiness = READY
        ↓
Backend feature expansion pauses
        ↓
Frontend Execution 01 begins
```

Frontend development must consume backend semantics.

It must not duplicate:

```text
UPSM interpretation
HAG reconstruction
feature inference
workflow reconstruction
confidence logic
revision logic
evidence resolution
```

Execution 05 is the final backend closure execution for the current CodeFlow architecture.
