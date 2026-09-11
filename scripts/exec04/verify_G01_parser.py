#!/usr/bin/env python3
"""Behaviorally verify multi-language and genuine incremental parser closure."""
from __future__ import annotations

import json
from pathlib import Path
import shutil
import subprocess
import sys

from common import ROOT, fail


def cargo() -> str | None:
    return shutil.which("cargo") or next(
        (str(path) for path in (Path.home() / ".cargo" / "bin" / "cargo.exe",) if path.is_file()), None
    )


def run_test(name: str) -> bool:
    executable = cargo()
    if executable is None:
        return False
    result = subprocess.run(
        [executable, "test", "-p", "codeflow-analysis", "--all-features", name],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode:
        print(result.stdout, file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        return False
    return True


def main() -> int:
    if cargo() is None:
        return fail("Cargo is unavailable")
    language_test = "every_declared_language_has_a_production_grammar_and_recovers"
    incremental_test = "incremental_parse_reuses_edited_tree_and_matches_clean_parse"
    if not run_test(language_test):
        return fail("ten-language parser fixture failed")
    if not run_test(incremental_test):
        return fail("incremental parser fixture failed")
    print(json.dumps({"languages_passed": 10, "incremental_equivalence": True, "old_tree_reuse_proven": True}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
