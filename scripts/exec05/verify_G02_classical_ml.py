from __future__ import annotations
import subprocess
import sys
from pathlib import Path

if __name__ == "__main__":
    raise SystemExit(subprocess.run([sys.executable, "-m", "unittest", "python.codeflow_ml.test_baselines", "-v"], cwd=Path(__file__).resolve().parents[2]).returncode)
