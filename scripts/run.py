#!/usr/bin/env python3
"""Cross-platform command dispatcher; commands are intentionally explicit."""
from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]

def main() -> int:
    registry = json.loads((ROOT / "scripts/commands.json").read_text(encoding="utf-8"))
    if len(sys.argv) != 2 or sys.argv[1] not in registry["commands"]:
        print("usage: run.py <" + "|".join(sorted(registry["commands"])) + ">", file=sys.stderr)
        return 2
    return subprocess.run(registry["commands"][sys.argv[1]], cwd=ROOT, shell=True).returncode

if __name__ == "__main__":
    raise SystemExit(main())
