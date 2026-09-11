"""Behavioral verifier for parse-aware framework route extraction."""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
from pathlib import Path


def cargo() -> str:
    found = shutil.which("cargo")
    if found:
        return found
    candidate = Path(os.environ.get("USERPROFILE", "")) / ".cargo" / "bin" / "cargo.exe"
    if candidate.is_file():
        return str(candidate)
    raise RuntimeError("Cargo was not found; framework extraction cannot be verified")


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    result = subprocess.run(
        [cargo(), "test", "-p", "codeflow-analysis", "framework_adapter_requires_parsed_route_constructs", "--all-features"],
        cwd=root,
        check=False,
    )
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())
