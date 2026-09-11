"""Behavioral verifier for distinct structural clustering implementations."""
from __future__ import annotations

import os
import shutil
import subprocess
import sys
from pathlib import Path


def main() -> int:
    cargo = shutil.which("cargo") or str(Path(os.environ.get("USERPROFILE", "")) / ".cargo" / "bin" / "cargo.exe")
    if not Path(cargo).is_file() and not shutil.which(cargo):
        raise RuntimeError("Cargo was not found")
    return subprocess.run(
        [cargo, "test", "-p", "codeflow-upsm", "--all-features"],
        cwd=Path(__file__).resolve().parents[2], check=False,
    ).returncode


if __name__ == "__main__":
    sys.exit(main())
