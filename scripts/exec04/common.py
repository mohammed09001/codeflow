"""Shared, non-controlling helpers for Execution 04 behavioral verifiers."""
from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
GOAL_IDS = {f"G{number:02d}" for number in range(17)}
ALLOWED_STATES = {"OPEN", "ACTIVE", "PASS", "STALE", "BLOCKED"}


def load_json(relative: str) -> dict:
    return json.loads((ROOT / relative).read_text(encoding="utf-8"))


def head_sha() -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=False
    )
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or "git rev-parse HEAD failed")
    return result.stdout.strip()


def fail(message: str) -> int:
    print(f"FAIL: {message}", file=sys.stderr)
    return 1
