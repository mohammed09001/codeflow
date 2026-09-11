# CodeFlow Execution 03 — Forced Closure, Goal–Plan–Loop Backend Completion

**Status:** READY FOR EXECUTION  
**Execution type:** Corrective closure / convergence / backend finalization  
**Project:** CodeFlow  
**Primary objective:** Close every known backend gap before frontend production work begins  
**Frontend implementation:** OUT OF SCOPE  
**Execution method:** Goal → Plan → Loop → Proof → Advance  
**Harness:** PROHIBITED AS AN EXECUTION CONTROLLER  
**LLM dependency in product core:** PROHIBITED  
**Normal stop condition:** Final Acceptance Gate only  

---

# 0. Executive Contract

Execution 03 exists because Execution 02 was not completed in the repository even though execution was reported as complete.

At the time this document was prepared, the repository had a useful foundation and a green three-platform core CI run, but the corrective work required by Execution 02 had not progressed beyond the first corrective phase in the persisted execution state.

This execution therefore does not add a new product direction.

It has one job:

> **Force CodeFlow to converge from the current repository state to a backend that is truthfully complete, directly verified, frontend-ready, and impossible to mark complete while known mandatory gaps remain.**

The agent executing this document must not reinterpret this as:

- a planning exercise;
- a prototype;
- an MVP;
- a partial implementation;
- a report-writing task;
- a refactor-only task;
- a documentation task;
- a “best effort” pass;
- a request to stop after each phase;
- a request to wait for human approval between phases.

The product target remains:

```text
Codebase
   ↓
Deterministic Program Analysis
   ↓
UPSM
   ↓
Graph Algorithms + Classical ML + Optional Trainable Graph Learning
   ↓
Evidence Fusion
   ↓
HAG + Workflow Reconstruction
   ↓
Versioned Query / View Contracts
   ↓
codeflowd
   ↓
HTTP + SSE + CLI
   ↓
Future Production Frontend
```

Execution 03 is complete only when the backend side of this chain is real, tested, versioned, and directly proven.

---

# 1. Known Repository Truth at Execution 03 Creation

The executor must re-check all of this against the current HEAD before modifying code. These are baseline observations, not permanent assumptions.

The repository already contains valuable foundations:

- stable canonical identity concepts;
- evidence provenance;
- UPSM;
- HAG;
- project/store boundaries;
- SQLite-backed persistence;
- query/view foundations;
- a no-LLM product invariant;
- cross-platform path corrections;
- a Windows/Linux/macOS CI workflow that has passed for the current core checks.

The following mandatory gaps were still present during the latest audit:

1. Tree-sitter runtime parses Rust only.
2. Incremental parsing is not real; parsing still uses a fresh parse path rather than editing and reusing the previous tree.
3. Framework recognition relies on source-string marker detection.
4. Framework markers can be promoted to deterministic facts without parse-aware validation.
5. `leiden_baseline` is weakly connected components rather than Leiden.
6. `agglomerative_baseline` delegates to the same connected-components result rather than implementing agglomerative clustering.
7. `hdbscan_baseline` is a degree/noise transform over that partition rather than HDBSCAN.
8. the class named `NaiveBayes` is not a probabilistic Naive Bayes implementation;
9. the function named `tree_rank` is a linear weighted score rather than a tree-based learner.
10. `node2vec_baseline`, `gcn_encode`, `gat_encode`, and `hetero_encode` are deterministic arithmetic surrogates rather than trainable graph models.
11. no real optimizer/backpropagation/checkpoint training loop exists for graph learning.
12. no evidence-based DL retention decision has been performed.
13. no production `codeflowd` daemon exists.
14. no real v1 HTTP server exists.
15. no production SSE revision stream exists.
16. CLI commands other than initialization are not wired to real backend operations.
17. the old final acceptance script still reflects Execution 01-era evidence rather than the required Execution 03 behavioral acceptance.
18. Execution 02 evidence was incomplete and frontend readiness remained `NOT_READY`.

Execution 03 must close these gaps or prove that a particular item is intentionally removed from the supported product contract through an explicit, versioned, justified decision.

Silently reducing scope is forbidden.

---

# 2. Mandatory Goal–Plan–Loop Mode

Every phase in this document is executed using **Goal–Plan–Loop Mode**.

This is not optional process language. It is the operating contract.

## 2.1 GOAL

Before changing code, state one observable goal.

A valid goal has:

- a concrete product capability;
- a reproducible current failure or missing behavior;
- a measurable terminal condition;
- a direct verification path.

Bad goal:

```text
Improve the parser.
```

Valid goal:

```text
All declared supported languages parse through the production Tree-sitter provider,
and incremental edits reuse an edited previous tree while producing normalized
evidence equivalent to a full parse of the same final source.
```

## 2.2 PLAN

After defining the Goal, inspect the current repository and produce a short active plan.

The plan must:

- derive from current code rather than memory;
- identify files and contracts likely to change;
- identify direct tests to add before or with implementation;
- identify regressions that must remain green;
- identify dependencies only when necessary;
- identify the smallest coherent sequence that can satisfy the Goal.

The plan is provisional.

If evidence disproves it, replace it.

Do not continue executing a known-wrong plan merely because it was written first.

## 2.3 LOOP

Execute continuously:

```text
OBSERVE CURRENT STATE
        ↓
REPRODUCE GAP
        ↓
IMPLEMENT NEXT PLAN STEP
        ↓
RUN NARROW VERIFICATION
        ↓
RUN RELEVANT REGRESSION
        ↓
COMPARE WITH GOAL
        ↓
GOAL CLOSED?
   ├─ NO → ROOT-CAUSE ANALYSIS → RE-PLAN → LOOP
   └─ YES → PROVE → RECORD → ADVANCE IMMEDIATELY
```

The loop has no artificial iteration maximum.

A failed test is loop input.

A build error is loop input.

A benchmark regression is loop input.

A daemon crash is loop input.

A model that fails to beat a baseline is loop input.

A dependency conflict is loop input.

None of those are normal stopping conditions.

## 2.4 PROOF

A Goal is closed only by direct evidence.

Allowed proof:

- executable unit test;
- integration test;
- end-to-end test;
- benchmark;
- clean build;
- real spawned process test;
- API contract test against a running service;
- CI result on the exact final SHA;
- model training/evaluation logs with reproducible configuration.

Not proof:

- function names;
- class names;
- comments;
- TODO completion claims;
- existence of files;
- existence of interfaces;
- static JSON evidence without underlying command evidence;
- a report saying “implemented”;
- a mock that does not exercise production behavior.

## 2.5 ADVANCE

When a Goal closes:

1. record evidence;
2. update passive execution state;
3. refresh repository context;
4. begin the next Goal immediately.

Do not ask:

```text
Should I continue?
```

Do not output:

```text
Phase complete. Ready for the next phase.
```

Do not stop because a phase boundary has been reached.

---

# 3. No-Harness Rule

Execution 03 must not introduce or depend on an execution harness that controls phase progression.

Specifically:

- do not use `scripts/harness.py` to decide whether execution continues;
- do not create a replacement harness;
- do not build a phase runner that exits after each phase;
- do not require a human confirmation file;
- do not require a “continue” command;
- do not turn evidence files into control-flow gates;
- do not make a background process owner terminate work after a phase;
- do not use a workflow that naturally returns control after each Goal.

Existing legacy harness code may be:

- removed if obsolete;
- retained only as legacy diagnostic code;
- converted into non-authoritative tests;

but it must not control Execution 03.

The authoritative execution mechanism is the Goal–Plan–Loop behavior in this document plus direct shell/tool execution.

---

# 4. Continuous Execution and Background Process Policy

Long-running processes must not stop the execution.

Examples:

- `codeflowd`;
- watch mode;
- HTTP server;
- SSE integration server;
- ML training;
- benchmark process;
- optional analyzer process;
- test fixture server.

When a required background process is started:

1. capture PID/process handle;
2. redirect or retain bounded logs;
3. health-check readiness;
4. continue independent work;
5. revisit when dependent work needs its result;
6. terminate only when no longer required;
7. restart on recoverable crash;
8. never equate “terminal still busy” with “execution must stop.”

Use bounded polling.

Never use an unbounded synchronous wait when independent tasks can proceed.

---

# 5. Stop Conditions

There are only two valid global stop conditions.

## 5.1 Successful stop

Execution 03 Final Acceptance Gate passes and:

```text
frontend_readiness: READY
```

is proven for the final accepted commit.

## 5.2 Genuine hard blocker

A blocker is genuine only when continuing requires something objectively unavailable, such as:

- unavailable credential for a mandatory external service with no permitted substitute;
- unavailable proprietary dependency required by the product contract;
- a legal/license conflict that cannot be resolved technically;
- unavailable hardware where the product contract explicitly requires that hardware;
- a destructive irreversible decision requiring the project owner.

The following are not hard blockers:

- test failures;
- build failures;
- dependency version conflicts;
- missing packages that can be installed;
- model training instability;
- poor model quality;
- performance regressions;
- daemon crashes;
- CI failures;
- difficult refactors;
- large code changes;
- unfamiliar technology.

Those conditions require another loop iteration.

If one branch is genuinely blocked, record it and continue every independent Goal before stopping globally.

---

# 6. Product Invariants

Execution 03 may repair implementation but must preserve these invariants.

1. UPSM is implementation truth.
2. HAG is human abstraction and remains separate from UPSM.
3. deterministic/observed evidence outranks inference.
4. inferred semantics never silently overwrite deterministic facts.
5. UNKNOWN is a valid state.
6. canonical IDs remain provider-neutral.
7. provider-local IDs remain evidence/provenance only.
8. every important relation remains traceable to source/evidence.
9. no mandatory LLM dependency enters CodeFlow.
10. core operation remains local/offline-capable.
11. GPU availability is optional.
12. ML/DL is proposal/ranking evidence, not implementation truth.
13. frontend-visible responses are revision-consistent.
14. resource use is bounded.
15. logs/errors do not casually leak source code.
16. Windows/Linux/macOS remain supported core targets.
17. production contracts are versioned.
18. unsupported capability is explicit rather than faked.

---

# 7. Passive Execution Records

Create:

```text
Execution/Execution 03.state.json
Execution/evidence-03/phase-00.json
Execution/evidence-03/phase-01.json
...
Execution/evidence-03/phase-17.json
Execution/Execution 03 Final Report.md
```

These records are passive.

They never control whether the agent continues.

Recommended state shape:

```json
{
  "execution": "03",
  "current_phase": "00",
  "accepted_sha": null,
  "phases": {},
  "hard_blockers": [],
  "frontend_readiness": "NOT_READY"
}
```

Every phase record must contain:

```json
{
  "phase": "NN",
  "goal": "...",
  "baseline_proof": [],
  "plan": [],
  "files_changed": [],
  "commands_run": [],
  "tests": [],
  "benchmarks": [],
  "failures_encountered": [],
  "repairs": [],
  "result": "complete | blocked",
  "remaining_gap": [],
  "next_phase": "NN"
}
```

Never write `complete` before the Goal's acceptance conditions pass.

---

# 8. Phase 00 — Truth Reset and Execution 03 Baseline

## GOAL

Establish current repository truth and remove any false assumption that Execution 02 is already complete.

## PLAN

1. inspect current HEAD, branch, working tree, tags, and CI;
2. read Execution 02 state/evidence;
3. inspect actual production implementations for every known gap;
4. run current direct baseline tests;
5. build a requirements-to-proof matrix;
6. initialize Execution 03 passive state;
7. remove harness-based progression from the active workflow.

## LOOP

Repeat until every mandatory gap has:

- an observed baseline;
- an owning phase;
- a direct acceptance mechanism.

Do not modify unrelated product architecture in this phase.

## Required baseline commands

Run the applicable current commands directly:

```text
git status --short
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD

cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features

python -m compileall -q python scripts
python -m unittest discover -s scripts/tests -v
python -m unittest discover -s python -p "test_*.py" -v
python scripts/no_llm_guard.py
```

If any command fails, record it. Do not hide the failure.

## Required baseline assertions

Confirm directly whether:

- parser is still Rust-only;
- parsing still uses `parse(..., None)`;
- framework detection still uses string containment;
- graph clustering functions are still surrogates;
- classical ML names still overstate behavior;
- GNN functions are still non-trainable;
- `codeflowd` exists or not;
- HTTP server exists or not;
- SSE exists or not;
- CLI commands are wired or not;
- final acceptance references Execution 03 or not.

## ACCEPTANCE GATE

Phase 00 closes only when every known gap is assigned to exactly one later Goal and the current state is truthfully recorded.

Then immediately start Phase 01.

---

# 9. Phase 01 — Freeze Cross-Platform Core Correctness

## GOAL

Preserve the corrected portable path semantics and prove the foundational core remains green while later work begins.

## PLAN

1. run portable path tests;
2. add missing UNC/device/traversal/symlink boundary fixtures if still missing;
3. verify CI workflow matrix;
4. ensure all three operating systems test the same core command set;
5. remove any stale “blocked” state that has already been disproven by green CI.

## LOOP

If any OS behaves differently:

```text
reproduce
→ isolate host-dependent assumption
→ replace with portable semantics
→ test locally where possible
→ push candidate
→ verify CI
→ repair until all three are green
```

## ACCEPTANCE GATE

Same commit must pass:

```text
Windows
Ubuntu/Linux
macOS
```

for:

```text
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Execution records must reflect the actual current CI truth.

Then immediately start Phase 02.

---

# 10. Phase 02 — Real Multi-Language Tree-sitter and Incremental Parsing

## GOAL

Turn Tree-sitter into a production multi-language incremental parsing subsystem.

## Required supported language set

Unless a versioned product-contract change explicitly removes one:

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

`Unknown` remains explicit.

## PLAN

### Step 1 — Grammar registry

Replace a Rust-only match with a registry abstraction.

The registry must map:

```text
Language
→ grammar provider
→ grammar/version metadata
→ parser configuration
```

Pin compatible grammar versions.

### Step 2 — Parse session state

A parse session must be able to retain:

- source identity;
- blob/revision identity;
- previous `Tree`;
- language;
- previous byte/point state needed for edits.

### Step 3 — Real incremental edit

For an edit:

1. calculate byte and point ranges;
2. apply Tree-sitter `InputEdit`/tree edit to the previous tree;
3. call parser parse with the edited old tree;
4. produce normalized CodeFlow evidence from the resulting new tree.

A cache hit is not incremental parsing.

`parse(bytes, None)` is not incremental parsing.

### Step 4 — Equivalence

For each language, test:

```text
initial source
→ full parse
→ edit source
→ incremental reparse
```

against:

```text
final edited source
→ clean full parse
```

Normalized CodeFlow evidence must be equivalent.

### Step 5 — Syntax error recovery

Every grammar needs malformed/incomplete fixtures.

CodeFlow must return useful bounded evidence rather than crash.

### Step 6 — Resource bounds

Add parse cancellation or bounded execution behavior for pathological files.

## LOOP

For each language:

```text
register grammar
→ compile
→ parse valid fixture
→ parse malformed fixture
→ incremental edit test
→ normalization equivalence
→ regression suite
→ next language
```

Do not wait for all languages before testing the first.

## ACCEPTANCE GATE

All declared languages:

- parse through production provider;
- recover from representative syntax errors;
- produce normalized evidence;
- never expose Tree-sitter node IDs as canonical CodeFlow IDs.

Incremental parsing must reuse an edited old tree.

No production parser path may silently fall back to Rust-only behavior.

Then immediately start Phase 03.

---

# 11. Phase 03 — Parse-Aware Framework Semantics

## GOAL

Eliminate deterministic semantic facts derived solely from string containment.

## PLAN

Build syntax-aware adapters using parsed structure and Tree-sitter queries or equivalent AST-aware logic.

Target representative ecosystems:

```text
Rust:
  Axum
  Actix

Python:
  FastAPI
  Flask
  Django

TypeScript / JavaScript:
  Express
  Nest
  Next server routes where statically recoverable

Java / Kotlin:
  Spring
  Ktor

Go:
  net/http
  common router forms

C#:
  ASP.NET endpoints/controllers
```

Every semantic result must carry:

- framework;
- semantic kind;
- method/route where recoverable;
- source span;
- evidence ID;
- fact class;
- confidence;
- provider.

String/lexical markers may remain only as lower-authority inferred evidence.

## Negative tests are mandatory

The following must not become deterministic route facts:

```text
comments containing app.get(
string literals containing @GetMapping
documentation examples
dead unrelated text
identifiers with similar names
```

## LOOP

For each adapter:

```text
write positive fixture
→ write deceptive negative fixture
→ implement syntax-aware extraction
→ prove positives
→ prove negatives
→ test malformed source
→ test aliases/imports when recoverable
→ regress
```

## ACCEPTANCE GATE

No production deterministic framework semantic may depend only on `source.contains(...)`.

Then immediately start Phase 04.

---

# 12. Phase 04 — Truthful Graph Algorithms

## GOAL

Replace mislabeled graph surrogates with real algorithms or truthfully remove/rename them.

## PLAN

### Leiden

Implement genuine Leiden community detection.

Preferred approach:

- use a mature maintained implementation if license/integration is acceptable;
- preserve deterministic seed/configuration;
- support weighted graphs where required;
- wrap results in CodeFlow IDs and inferred provenance.

Do not call connected components “Leiden.”

### Agglomerative clustering

Use a true hierarchical agglomerative algorithm with:

- declared feature/distance representation;
- explicit linkage;
- deterministic configuration.

Do not delegate to Leiden/connected components.

### HDBSCAN

Use a genuine HDBSCAN implementation over an explicit feature or distance representation.

Preserve:

- cluster labels;
- noise/outlier semantics;
- relevant configuration;
- deterministic/reproducible settings where supported.

Do not emulate HDBSCAN with a degree threshold.

## Golden fixtures

At minimum:

1. two dense communities with weak bridge;
2. disconnected communities;
3. isolated nodes;
4. weighted bridge;
5. ambiguous boundary node;
6. density noise;
7. mixed-density clusters.

Each algorithm must produce behavior that distinguishes it from connected components.

## LOOP

For each algorithm:

```text
select mature implementation or justified internal implementation
→ create adapter
→ golden tests
→ deterministic repeat tests
→ large synthetic benchmark
→ verify inference-only semantics
→ regress
```

## ACCEPTANCE GATE

Production names and actual semantics must match.

If a real implementation cannot be justified, remove the misleading production name and explicitly demote the capability instead of faking it.

Then immediately start Phase 05.

---

# 13. Phase 05 — Truthful Classical Machine Learning

## GOAL

Make the classical ML layer mathematically honest, reproducible, and useful as the non-DL baseline.

## PLAN

### Naive Bayes

Replace the existing nearest-class-mean behavior with a real probabilistic Naive Bayes model appropriate to feature distribution.

Possible valid choices:

- GaussianNB for continuous normalized features;
- MultinomialNB for non-negative count/frequency features.

The chosen model must be documented by feature contract.

### Logistic model

Harden logistic regression:

- preprocessing;
- deterministic seed;
- convergence behavior;
- regularization;
- probability output.

### Tree baseline

Replace linear `tree_rank` behavior with an actual tree-based model, such as:

- DecisionTree;
- RandomForest;
- HistGradientBoosting;

or rename/remove the old function if tree semantics are not required.

### Evaluation

Split by repository/project, not random entities, when possible.

Prevent leakage.

Record:

- macro F1;
- micro F1/accuracy where meaningful;
- per-class precision/recall;
- PR-AUC for imbalanced tasks;
- Brier score/calibration;
- inference latency;
- model version;
- feature schema;
- seed.

## LOOP

```text
define feature contract
→ construct project-separated fixture/dataset
→ train
→ evaluate
→ inspect leakage
→ calibrate if needed
→ compare with deterministic baseline
→ repair
```

## ACCEPTANCE GATE

Every public model name corresponds to a real algorithm.

UNKNOWN remains permitted.

No score is promoted to deterministic truth.

Then immediately start Phase 06.

---

# 14. Phase 06 — Real Trainable Graph Learning

## GOAL

Replace arithmetic GNN surrogates with genuinely trainable graph representation models.

## Dependency strategy

Prefer PyTorch + PyTorch Geometric or another mature equivalent.

Core CodeFlow must still run without the optional ML/DL environment.

CPU execution is mandatory.

GPU acceleration is optional.

## Required paths

Implement and evaluate:

1. real node2vec or MetaPath2Vec-style representation baseline;
2. Graph Autoencoder with GCN encoder;
3. Graph Autoencoder with GAT encoder;
4. heterogeneous graph model that respects node and edge types;
5. typed link reconstruction;
6. negative sampling;
7. at least one self-supervised auxiliary objective where useful;
8. bounded mini-batch or neighbor-sampling path for larger graphs.

## Trainability definition

A model is trainable only if the production experiment path has:

```text
learnable parameters
loss
zero_grad
forward
backward
optimizer.step
epochs
validation/evaluation
seed
checkpoint
reload
```

Fixed arithmetic transforms do not qualify.

## Heterogeneous graph requirement

Typed UPSM relations must not be discarded into one anonymous adjacency matrix when testing the heterogeneous model.

At minimum distinguish meaningful node/edge types.

## Required proof

On a tiny deterministic fixture:

- initial loss is recorded;
- training executes;
- loss/objective improves or a task metric improves;
- checkpoint saves;
- new process/model instance reloads;
- inference output shape/schema is valid.

## LOOP

```text
build graph dataset adapter
→ validate IDs and relation types
→ implement baseline
→ train tiny fixture
→ fix instability
→ checkpoint/reload
→ held-out evaluation
→ resource benchmark
→ next model
```

## ACCEPTANCE GATE

No function may be described as GCN/GAT/heterogeneous GNN unless it has real learnable parameters and a real training/evaluation path.

Then immediately start Phase 07.

---

# 15. Phase 07 — Scientific Model Retention Gate

## GOAL

Decide whether Deep Learning belongs in CodeFlow's default product path based on measured value.

## PLAN

Before final held-out evaluation, freeze:

- dataset split;
- primary task;
- primary metric;
- latency budget;
- memory budget;
- material-improvement threshold.

Compare:

```text
deterministic structural baseline
classical ML
node2vec / representation baseline
GAE + GCN
GAE + GAT
heterogeneous model
```

Default threshold unless repository evidence supports a better pre-registered threshold:

```text
+0.03 absolute macro F1
OR
+0.05 absolute PR-AUC on the declared imbalanced primary task
```

while staying inside resource budgets.

## LOOP

If DL underperforms:

- inspect dataset leakage;
- inspect feature quality;
- inspect typed graph construction;
- inspect training stability;
- repair legitimate defects;
- rerun only under the frozen protocol.

Do not move the goalposts after seeing final held-out results.

## ACCEPTANCE GATE

One of two valid outcomes:

### Outcome A

DL materially wins and is retained in the default inference path.

### Outcome B

DL does not materially win and is explicitly optional/experimental or removed from the default product path.

Both outcomes are successful if evidence is honest.

Then immediately start Phase 08.

---

# 16. Phase 08 — Workflow and Runtime Truth

## GOAL

Ensure CodeFlow can reconstruct bounded workflows without confusing possible, reconstructed, and observed behavior.

## Required states

At minimum preserve:

```text
POSSIBLE
RECONSTRUCTED
OBSERVED
UNKNOWN
```

Do not label static inference as observed runtime behavior.

## PLAN

Harden:

- interprocedural path reconstruction;
- branch/decision reconstruction;
- recursion/cycle handling;
- event-driven edges;
- runtime trace ingestion;
- static/runtime reconciliation;
- revision consistency.

Add budgets:

- max path count;
- max depth;
- max expansion;
- max cycle revisits;
- timeout/cancellation.

## LOOP

Test fixtures:

```text
linear workflow
branch success/failure
nested call
recursive function
event emit/listen
missing runtime branch
runtime-only observed path
static/runtime contradiction
stale runtime revision
```

Repair until classifications remain truthful.

## ACCEPTANCE GATE

No observed claim may be generated without runtime evidence.

No static traversal may be unbounded.

Then immediately start Phase 09.

---

# 17. Phase 09 — Frontend View Contract Freeze

## GOAL

Produce the stable, versioned data contract that the future frontend will consume.

## Required view families

```text
Project
Revision
Capabilities
Graph Node
Graph Edge
HAG Feature
Subsystem
Workflow
Evidence Reference
Query Result
Semantic Zoom
Pagination Cursor
Error
Partial/Unknown Status
```

Every revision-dependent response must identify its revision.

Every relation exposed to the frontend must expose enough metadata to distinguish:

```text
deterministic
observed
inferred
reconstructed
unknown
```

## PLAN

1. define v1 schemas;
2. deterministic ordering;
3. stable IDs;
4. cursor semantics;
5. bounded query/zoom;
6. compatibility tests;
7. golden serialization fixtures.

## LOOP

```text
serialize fixture
→ deserialize/validate
→ compare deterministic ordering
→ mutate repository revision
→ verify cursor/revision safety
→ compatibility test
→ repair
```

## ACCEPTANCE GATE

Freeze v1 after this phase.

Post-freeze breaking changes require a new API/schema version.

Then immediately start Phase 10.

---

# 18. Phase 10 — Production `codeflowd`

## GOAL

Create a real local backend daemon that owns project lifecycle, analysis, revisions, and queries.

## Architecture preference

Implement `codeflowd` as a Rust binary integrated with canonical Rust backend crates unless current repository evidence proves a stronger alternative.

Suggested location:

```text
crates/codeflowd/
```

## Required lifecycle

```text
start
health-ready
open/init project
analyze
commit revision
query committed revision
watch/update
publish new revision
cancel
shutdown
restart
recover persisted state
```

## Concurrency invariants

- in-progress analysis cannot corrupt the current readable revision;
- revision commit is atomic from client perspective;
- query sees a coherent committed revision;
- cancellation leaves the previous committed state valid;
- bounded concurrent analyses;
- bounded queues.

Bind to loopback by default.

## LOOP

Use a real spawned daemon process in tests.

```text
spawn
→ wait for health
→ create project
→ analyze fixture
→ assert committed revision
→ query
→ kill gracefully
→ restart
→ query persisted state
→ inject analysis failure
→ prove previous revision still readable
```

## ACCEPTANCE GATE

A real process E2E test passes.

Then immediately start Phase 11.

---

# 19. Phase 11 — Real HTTP API v1

## GOAL

Expose CodeFlow through a real versioned HTTP API suitable for the future frontend.

## Recommended stack

A Rust HTTP stack such as Axum/Tokio is preferred because the canonical backend is already Rust.

The implementation may use another mature Rust stack if justified.

## Minimum API

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

## Contract requirements

- typed error envelope;
- explicit API version;
- explicit revision IDs;
- request size limits;
- response limits;
- query time budget;
- cursor pagination;
- stale revision errors;
- capability errors;
- deterministic JSON;
- safe malformed payload behavior.

Maintain a machine-readable OpenAPI contract.

## LOOP

Every endpoint must be tested against a real running daemon.

Direct function dispatch tests are not sufficient.

## ACCEPTANCE GATE

All v1 endpoints have:

- success test;
- invalid input test;
- typed failure test;
- revision consistency test where relevant.

Then immediately start Phase 12.

---

# 20. Phase 12 — SSE Revision/Event Stream

## GOAL

Allow future frontend clients to observe revision changes without polling.

## Transport

Use Server-Sent Events unless a real bidirectional requirement is proven.

Suggested endpoint:

```text
GET /v1/projects/{project_id}/events
```

## Required events

```text
analysis_started
analysis_progress
revision_committed
analysis_completed
analysis_failed
capabilities_changed
```

`analysis_progress` may be coalesced.

## Required semantics

- monotonic event ID/cursor;
- project identity;
- revision identity where relevant;
- keepalive;
- reconnect/resume;
- bounded per-client queue;
- slow-consumer policy;
- no raw source code in events;
- committed event only after revision is queryable.

## LOOP

Real integration scenario:

```text
connect
→ trigger analysis
→ observe started
→ observe committed revision
→ disconnect
→ trigger another revision
→ reconnect with cursor
→ receive missed committed event
→ query that exact revision
```

## ACCEPTANCE GATE

Reconnect behavior and revision consistency are directly proven.

Then immediately start Phase 13.

---

# 21. Phase 13 — Real CLI Integration

## GOAL

Make the CLI a working user interface over the same backend contracts that the frontend will use.

## Commands

```text
codeflow init
codeflow analyze
codeflow watch
codeflow status
codeflow graph
codeflow evidence
```

A thin client architecture is preferred.

The CLI may be Rust or Python, but it must not maintain a second competing semantic implementation.

## Required behavior

### init

Create/open project through canonical backend semantics.

### analyze

Trigger real analysis and return/await a committed revision.

### watch

Observe file changes/revisions through the backend lifecycle.

### status

Return real daemon/project/revision state.

### graph

Return real bounded graph/view data.

### evidence

Resolve real evidence by canonical ID.

## LOOP

Run against a temporary real project.

Do not mock the daemon for final CLI acceptance.

## ACCEPTANCE GATE

This scenario passes:

```text
init
→ analyze
→ status
→ graph
→ evidence
→ watch
→ edit file
→ observe new revision
```

Then immediately start Phase 14.

---

# 22. Phase 14 — Security, Privacy, and Resource Hardening

## GOAL

Prevent the production service boundary from introducing obvious security/privacy/resource defects.

## PLAN

Verify:

### Network

- loopback default;
- explicit bind configuration;
- constrained CORS/origin policy;
- local client policy where necessary.

### Paths

- traversal rejection;
- Windows drive roots;
- UNC/device paths;
- symlink escape policy;
- provider document paths.

### Processes

- no unsafe shell composition;
- timeout;
- kill/reap;
- output cap;
- concurrency cap.

### API

- request size limit;
- malformed JSON behavior;
- query timeout;
- cursor validation;
- internal error sanitization.

### Privacy

- no raw source code in ordinary logs;
- no source snippets in crash messages by default;
- event stream excludes source.

### ML

- training/inference memory bounds;
- dataset path safety;
- checkpoint schema/version checks.

## LOOP

Use property/fuzz-style tests where useful.

For each high-severity defect:

```text
reproduce
→ patch
→ exploit/negative regression test
→ broader regression
```

## ACCEPTANCE GATE

No known high-severity path escape, shell injection, non-loopback exposure, uncontrolled resource explosion, or source-leaking diagnostic remains.

Then immediately start Phase 15.

---

# 23. Phase 15 — Packaging and Clean-Machine Operation

## GOAL

Prove CodeFlow works outside the original development machine.

## Required product profiles

### Core profile

Must run without:

- GPU;
- external LLM;
- Joern;
- remote model API;
- optional graph DL environment.

### ML-enabled profile

Adds classical ML and optional graph-learning dependencies.

## PLAN

1. lock dependencies;
2. document/install optional extras;
3. build release binaries;
4. package daemon/CLI;
5. clean temporary install;
6. create project;
7. run analysis;
8. start API;
9. run CLI/API smoke tests.

## LOOP

Test at least one clean environment per supported OS through CI or equivalent clean runner.

If packaging differs by OS, explicitly test each path.

## ACCEPTANCE GATE

Same source revision produces valid install/startup on Windows, Linux, and macOS for the supported core product.

Then immediately start Phase 16.

---

# 24. Phase 16 — Expanded CI That Tests the Real Product

## GOAL

Make green CI mean substantially more than “Rust compiles.”

## Required matrix

```text
Windows
Ubuntu/Linux
macOS
```

## Required core jobs

At minimum:

```text
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features

python -m compileall -q python scripts
python -m unittest discover -s scripts/tests -v
python -m unittest discover -s python -p "test_*.py" -v
python scripts/no_llm_guard.py
```

Add direct integration jobs for:

- multi-language parsing;
- incremental parsing;
- daemon process E2E;
- HTTP API E2E;
- SSE reconnect;
- CLI E2E;
- clean packaging smoke.

ML-enabled jobs must test real classical ML.

Trainable graph learning may run on a dedicated bounded CPU job if too heavy for every OS.

GPU CI is optional.

## No harness authority

Do not use a harness to mark phases complete.

If `scripts/harness.py` remains, it is not an acceptance gate and must not be the thing that makes CI “green.”

## LOOP

Push candidate:

```text
inspect failing OS/job
→ reproduce or infer platform cause from logs
→ repair
→ rerun
```

Continue until all mandatory jobs pass on the same SHA.

## ACCEPTANCE GATE

The exact candidate SHA has a green mandatory matrix.

Then immediately start Phase 17.

---

# 25. Phase 17 — Final Truth Audit and Backend Freeze

## GOAL

Prove CodeFlow is actually ready for frontend production development.

## 25.1 Create a new final verifier

Create a one-shot verifier such as:

```text
scripts/final_acceptance_03.py
```

This is not an execution harness.

It must not manage phases.

It must only run/check direct mandatory acceptance commands and exit success/failure.

It must never substitute file-existence checks for behavior.

## 25.2 Direct final commands

From a clean checkout:

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

Run the real:

- parser E2E;
- incremental equivalence E2E;
- graph algorithm truth tests;
- classical ML evaluation;
- DL training smoke/evaluation;
- daemon E2E;
- API E2E;
- SSE reconnect E2E;
- CLI E2E;
- packaging smoke.

## 25.3 Placeholder audit

Search production code for:

```text
TODO
FIXME
placeholder
stub
surrogate
fake
mock
not implemented
```

Each match must be classified.

Mandatory product behavior may not still depend on a placeholder/surrogate.

## 25.4 Algorithm-name audit

Manually/directly verify:

```text
Leiden means Leiden
Agglomerative means Agglomerative
HDBSCAN means HDBSCAN
NaiveBayes means probabilistic NB
tree model means a tree model
GCN means trainable GCN
GAT means trainable GAT
heterogeneous GNN uses relation types
```

## 25.5 Parser audit

Prove production parsing for every supported language.

Prove old-tree incremental parsing.

## 25.6 Service audit

Prove:

```text
codeflowd exists
real process starts
health endpoint works
project opens
analysis commits revision
graph query returns meaningful data
evidence query resolves provenance
SSE emits revision
restart preserves expected state
```

## 25.7 Full frontend-readiness scenario

A clean multi-language fixture project must pass:

```text
install/start CodeFlow
        ↓
initialize project
        ↓
analyze repository
        ↓
commit revision R1
        ↓
receive R1 through event stream
        ↓
query capabilities
        ↓
query graph view
        ↓
semantic zoom / bounded query
        ↓
resolve evidence to source location
        ↓
edit source
        ↓
incremental parse/analyze
        ↓
commit R2
        ↓
receive R2 through resumed/live stream
        ↓
query R2 consistently
        ↓
prove R1/R2 revision separation
        ↓
shutdown
        ↓
restart
        ↓
query persisted latest revision
```

Assertions must inspect real semantic data.

HTTP 200 alone is not enough.

## 25.8 Same-SHA CI gate

Windows/Linux/macOS mandatory jobs must be green on the exact accepted SHA.

## 25.9 Final state

Only after all requirements pass:

```json
{
  "frontend_readiness": "READY"
}
```

Otherwise:

```json
{
  "frontend_readiness": "NOT_READY"
}
```

and list exact blockers.

No soft language.

No “mostly ready.”

No “ready with mandatory limitations.”

---

# 26. Global Repair Rules

These rules apply in every loop.

## 26.1 Root cause over patches

When something fails:

1. reproduce;
2. trace the failing data/control path;
3. identify the owning abstraction;
4. correct the abstraction;
5. add a regression test;
6. run broader regression.

Do not stack workarounds around a broken core assumption.

## 26.2 Replace misleading code

When a surrogate has the name of a real algorithm:

- replace it with the real implementation;
- or rename/remove it.

Do not leave both active production paths without an explicit experimental boundary.

## 26.3 Delete dead experiments

Temporary experiments must end in one of:

```text
promoted
quarantined under explicit experimental namespace
deleted
```

Do not accumulate abandoned alternate engines.

## 26.4 No silent fallbacks

If an optional capability is absent:

```text
capability unavailable
```

must be visible.

Do not silently downgrade to a materially different algorithm under the same name.

## 26.5 No hidden scope reduction

If implementing the exact capability is impossible, do not quietly weaken tests.

A product-contract reduction must be:

- explicit;
- justified;
- versioned;
- reflected in docs/API/capabilities;
- reported in Final Report.

---

# 27. Context Engineering Rules

Long execution requires deliberate context control.

At the beginning of every Goal, reload only:

1. global invariants;
2. current Goal;
3. relevant architecture contracts;
4. affected production files;
5. relevant tests;
6. previous phase output only when it constrains the current Goal.

Do not repeatedly load the entire historical execution documents unless needed.

Repository evidence is more authoritative than conversational memory.

After major changes, summarize internally:

```text
what changed
what contract changed
what remains invariant
what tests prove it
what the next Goal depends on
```

If three repair attempts fail against the same symptom, discard the active plan and rebuild context from the repository.

---

# 28. Plan Engineering Rules

A plan is not a promise to follow the original approach.

Each Goal plan should usually be 3–12 concrete steps.

Every step should state:

```text
action
affected boundary
verification
```

Example:

```text
1. Add Tree-sitter Python grammar provider.
   Boundary: codeflow-analysis grammar registry.
   Verify: valid + malformed Python fixtures.

2. Add old-tree cache structure.
   Boundary: parse-session state.
   Verify: second parse receives edited previous tree.

3. Add incremental equivalence test.
   Boundary: normalized SyntaxEvidence.
   Verify: full(final) == incremental(initial + edit).
```

Do not write vague plans such as:

```text
Implement parser improvements.
```

---

# 29. Loop Engineering Rules

A loop must converge toward a Goal, not merely repeat commands.

Every failed iteration must produce new information.

Track:

- hypothesis;
- command;
- observed result;
- cause;
- next change.

If an iteration produces the same failure without new evidence, change the method.

Prefer narrow tests first.

Run broad tests after local confidence.

Do not run a 20-minute suite after every one-line change if a 2-second targeted test can reject the change.

Do run the full required regression before closing the Goal.

---

# 30. Machine Learning and Deep Learning Rules

ML/DL exists to improve semantic abstraction, not to decorate the product.

## Deterministic layer

The following should remain deterministic when statically recoverable:

```text
AST structure
source spans
imports
definitions/references
calls
control relationships
data relationships
route syntax
provider evidence
revision identity
```

## Classical ML layer

May:

```text
rank
cluster
classify candidates
fuse signals
estimate confidence
identify infrastructure candidates
```

## Deep Learning layer

May:

```text
learn graph/code representations
propose subsystem similarity
improve feature candidate ranking
improve architecture recovery
```

DL may not:

```text
invent deterministic source facts
overwrite observed runtime evidence
silently become required for core analysis
```

Model outputs require provenance.

---

# 31. Research-Grounded Implementation Anchors

Execution 03 is self-contained, but the following implementation principles are intentionally aligned with established tooling:

## Tree-sitter

True incremental parsing requires:

```text
edit previous syntax tree
→ pass edited previous tree to parser
→ reuse unchanged structure
```

Caching normalized output alone is not sufficient.

## Leiden

Use a genuine Leiden implementation when exposing Leiden semantics.

Connected components are not a substitute.

## HDBSCAN

Use a real density-based hierarchical clustering implementation that can represent noise/outliers.

Degree thresholding is not HDBSCAN.

## Graph learning

A real GAE/GAT/heterogeneous GNN requires real learnable parameters and training.

PyTorch Geometric-style concepts are valid implementation references:

```text
GAE
GATConv
HeteroData
HeteroConv
neighbor sampling
```

## HTTP/SSE

A Rust stack such as Axum can expose:

```text
typed HTTP routing
JSON
SSE
keepalive
Tower middleware
timeouts
tracing
```

CodeFlow should use mature service primitives rather than inventing an HTTP protocol stack.

---

# 32. Explicit Non-Goals

Execution 03 must not:

- build the production frontend;
- redesign the UI;
- add chat as the primary UX;
- add an LLM to compensate for incomplete static analysis;
- move the core requirement to a cloud service;
- require a GPU;
- introduce billing/accounts;
- add unrelated languages beyond the declared supported set before closure;
- expand into repository hosting;
- rewrite UPSM/HAG merely for aesthetic reasons;
- replace working persistence without evidence;
- build a new harness.

---

# 33. Definition of Done

Execution 03 is done only when CodeFlow has:

- portable cross-platform core semantics;
- green same-SHA CI on Windows/Linux/macOS;
- real multi-language Tree-sitter parsing;
- real incremental old-tree reparsing;
- parse-aware framework extraction;
- truthful external analyzer integration;
- genuine Leiden or explicitly removed Leiden capability;
- genuine agglomerative clustering;
- genuine HDBSCAN;
- truthful classical ML models;
- project-separated evaluation where feasible;
- real trainable graph-learning experiments;
- evidence-based decision on whether DL is default;
- bounded workflow/runtime reconstruction;
- stable versioned frontend view contracts;
- production `codeflowd`;
- real HTTP API v1;
- real SSE revision stream;
- real CLI workflow;
- security/privacy/resource hardening;
- clean-machine packaging;
- expanded CI testing actual product behavior;
- direct final E2E;
- complete phase evidence 00–17;
- `Execution 03 Final Report.md`;
- final accepted SHA;
- `frontend_readiness: READY`.

Anything less is incomplete.

---

# 34. Final Execution Prompt

Use the following prompt to start Execution 03 with Claude Code, Codex, OpenCode, or another capable coding agent:

```text
Execute `Execution/Execution 03.md` from the current repository state.

Operate exclusively in Goal–Plan–Loop mode:

GOAL
→ inspect the current repository and state the exact observable target for the active phase.

PLAN
→ create a short repository-grounded implementation plan with direct verification for each step.

LOOP
→ reproduce the current gap, implement the next step, run targeted verification, run relevant regressions, compare against the Goal, root-cause failures, revise the Plan when evidence disproves it, and repeat until the Goal is directly proven.

PROOF
→ close a phase only with behavioral evidence from real tests, builds, benchmarks, spawned processes, API/E2E tests, model training/evaluation, or same-SHA CI as required.

ADVANCE
→ after proof, record passive evidence and immediately begin the next phase. Do not ask for permission. Do not stop at a phase boundary.

Do not use `scripts/harness.py` or any replacement harness as an execution controller. Do not create a phase runner that waits for human confirmation. State/evidence files are audit records only.

Do not restart correct completed foundations. Inspect existing code first and preserve working UPSM, HAG, persistence, identity, and evidence contracts unless direct evidence proves they must change.

Do not accept function names, file names, comments, adapters, interfaces, TODO removal, or reports as proof of implementation.

Known mandatory closure areas include multi-language and true incremental Tree-sitter parsing, parse-aware framework semantics, genuine graph algorithms, truthful classical ML, genuinely trainable graph learning and its retention benchmark, workflow truth hardening, frontend-view contract freeze, production codeflowd, real HTTP v1, SSE revision streaming, real CLI integration, security/resource hardening, clean packaging, expanded same-SHA cross-platform CI, and final full-system acceptance.

A failure is not a stopping condition. Treat it as loop input.

A background process is not a stopping condition. Health-check it, keep its logs bounded, continue independent work, and return to its result when needed.

Only stop normally when Execution 03 Final Acceptance passes and repository evidence proves:

frontend_readiness: READY

If a genuine external blocker exists, continue all independent work before reporting it.

Do not reduce scope to an MVP.
Do not introduce a mandatory LLM.
Do not require GPU availability for the core product.
Do not build the frontend in this execution.

Continue autonomously until all mandatory gaps are closed and the final accepted commit is proven.
```

---

# 35. Final Product Transition

When Execution 03 passes:

```text
Backend architecture
        ↓
Frozen v1 view/API contracts
        ↓
Frontend readiness proven
        ↓
Backend feature expansion pauses
        ↓
Frontend Execution 01 begins
```

At that point, frontend development should consume the backend rather than inventing backend semantics itself.

The future frontend must not duplicate:

- graph interpretation;
- confidence logic;
- revision logic;
- evidence resolution;
- feature reconstruction;
- workflow reconstruction.

Those remain backend responsibilities.

Execution 03 therefore represents the backend closure line for the current CodeFlow product architecture.
