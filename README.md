# CodeFlow

CodeFlow is a local-first backend for evidence-backed codebase comprehension. It
builds canonical program evidence (UPSM), confidence-scored human abstractions
(HAG), bounded workflow and view graphs, and exposes those backend contracts for
a future frontend. It deliberately has no LLM dependency.

## Development

Install Rust 1.88 and Python 3.11–3.14, then run:

```text
python scripts/harness.py self-audit
python scripts/run.py rust_test
python scripts/run.py python_test
```

The command registry in `scripts/commands.json` is canonical. Phase evidence and
execution state live in `Execution/`.
