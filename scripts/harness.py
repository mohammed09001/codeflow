#!/usr/bin/env python3
"""Deterministic validator for the Execution 01 harness."""
from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
REQUIRED_LEDGER = {"execution", "schema_version", "current_phase", "phases", "blocked_items", "accepted_deviations", "tool_versions", "last_verified_commit"}
REQUIRED_EVIDENCE = {"schema_version", "phase", "completed_at", "files_changed", "commands_run", "tests", "test_outcomes", "benchmark_outcomes", "known_limitations", "no_llm_compliance", "schema_changes", "next_phase_dependencies"}

def audit() -> int:
    errors: list[str] = []
    for name in ("AGENTS.md", "CLAUDE.md", "schemas/phase-evidence.schema.json", "scripts/commands.json", "scripts/no_llm_guard.py"):
        if not (ROOT / name).is_file(): errors.append(f"missing {name}")
    try:
        ledger = json.loads((ROOT / "Execution/Execution 01.state.json").read_text(encoding="utf-8"))
        missing = REQUIRED_LEDGER - ledger.keys()
        if missing: errors.append(f"ledger missing keys: {sorted(missing)}")
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(f"invalid ledger: {exc}")
    for evidence in sorted((ROOT / "Execution/evidence").glob("phase-*.json")):
        try:
            data = json.loads(evidence.read_text(encoding="utf-8"))
            missing = REQUIRED_EVIDENCE - data.keys()
            if missing: errors.append(f"{evidence.relative_to(ROOT)} missing keys: {sorted(missing)}")
        except json.JSONDecodeError as exc:
            errors.append(f"invalid evidence {evidence.relative_to(ROOT)}: {exc}")
    result = subprocess.run([sys.executable, str(ROOT / "scripts/no_llm_guard.py")], cwd=ROOT, capture_output=True, text=True)
    if result.returncode: errors.append(result.stderr.strip())
    if errors:
        print("harness audit: FAIL\n" + "\n".join(errors), file=sys.stderr)
        return 1
    print("harness audit: PASS")
    return 0

if __name__ == "__main__":
    if len(sys.argv) != 2 or sys.argv[1] != "self-audit":
        raise SystemExit("usage: harness.py self-audit")
    raise SystemExit(audit())
