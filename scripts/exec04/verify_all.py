#!/usr/bin/env python3
"""Rerun every Execution 04 behavioral verifier; this does not advance goal state."""
from __future__ import annotations

from pathlib import Path
import subprocess
import sys

from common import ROOT, load_json


def main() -> int:
    failures = []
    for goal in load_json("Execution/exec04/goals.json")["goals"]:
        verifier = ROOT / goal["verifier"]
        result = subprocess.run([sys.executable, str(verifier)], cwd=ROOT, check=False)
        if result.returncode:
            failures.append(goal["id"])
    if failures:
        print("FAIL: " + ", ".join(failures), file=sys.stderr)
        return 1
    print("PASS: all Execution 04 verifiers passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
