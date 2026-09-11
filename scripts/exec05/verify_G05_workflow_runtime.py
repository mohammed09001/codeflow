from __future__ import annotations
import subprocess, sys
from pathlib import Path
raise SystemExit(subprocess.run([sys.executable, "-m", "unittest", "python.codeflow_ml.test_workflows", "python.codeflow_ml.test_runtime", "-v"], cwd=Path(__file__).resolve().parents[2]).returncode)
