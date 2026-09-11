import json
import subprocess
import sys
from pathlib import Path


def main() -> int:
    root = Path(__file__).parent.parent
    ledger = json.loads((root / "Execution/Execution 01.state.json").read_text())
    missing = [f"{phase:02d}" for phase in range(33) if not (root / f"Execution/evidence/phase-{phase:02d}.json").is_file()]
    if missing:
        print("missing evidence:", ",".join(missing)); return 1
    for command in ((sys.executable, "scripts/no_llm_guard.py"), (sys.executable, "scripts/harness.py", "self-audit")):
        subprocess.run(command, cwd=root, check=True)
    print(f"acceptance preflight: PASS through phase {ledger['current_phase']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
