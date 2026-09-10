# CodeFlow execution constitution

## Permanent invariants

- Build a backend semantic engine only; production frontend work is out of scope.
- No LLM, generative-AI SDK, remote embedding service, or provider-local semantic truth.
- Deterministic program evidence precedes inferred abstractions. Preserve provenance,
  truth class, confidence dimensions, stable canonical IDs, and explicit `UNKNOWN`.
- Keep UPSM (program evidence) separate from HAG (human abstractions). Do not add
  competing implementations; repair or remove obsolete ownership paths.
- Local operation must work without external graph databases or a Python ML worker.
- Do not log source text or secrets by default. Treat analyzer/provider failures as
  typed, capability-scoped diagnostics.

## Execution protocol

1. Read global invariants plus the current Article and its adjacent interfaces from
   `Execution/Execution 01.md`; do not load the whole execution unnecessarily.
2. Inspect current code and `Execution/Execution 01.state.json` before modifying it.
3. Use the CFEL loop: observe, model, smallest coherent delta, implement, targeted
   verification, phase verification, inspect evidence, repair owner, freeze.
4. After each phase, validate and write `Execution/evidence/phase-XX.json`, then
   update the ledger. Do not mark a phase complete without its gate.
5. Record every command and test outcome in evidence. Keep generated output stable.

## Commands

The canonical registry is `scripts/commands.json`. Run `python scripts/harness.py
self-audit` before phase completion. Once the workspace exists, run the relevant
commands from that registry, including Rust formatting, linting, and tests.

## Boundaries

- Rust owns daemon, CLI, persistence, canonical models, graph/query/API and analysis orchestration.
- Python owns optional learning/evaluation workers and exchanges versioned bulk data only.
- External analyzers are adapters, never canonical ID/schema owners.
