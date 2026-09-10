import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

class HarnessTests(unittest.TestCase):
    def test_self_audit_passes(self):
        result = subprocess.run([sys.executable, "scripts/harness.py", "self-audit"], cwd=ROOT, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_evidence_schema_has_required_contract(self):
        schema = json.loads((ROOT / "schemas/phase-evidence.schema.json").read_text(encoding="utf-8"))
        self.assertIn("no_llm_compliance", schema["required"])
        self.assertIn("phase", schema["required"])

    def test_guard_rejects_banned_dependency_in_scoped_copy(self):
        guard = (ROOT / "scripts/no_llm_guard.py").read_text(encoding="utf-8")
        self.assertIn("BANNED", guard)
        self.assertIn("openai", guard)

    def test_workspace_configuration_is_pinned_and_cross_platform(self):
        cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
        toolchain = (ROOT / "rust-toolchain.toml").read_text(encoding="utf-8")
        workflow = (ROOT / ".github/workflows/verify.yml").read_text(encoding="utf-8")
        self.assertIn('rust-version = "1.88"', cargo)
        self.assertIn('channel = "1.88.0"', toolchain)
        self.assertIn("windows-latest", workflow)

if __name__ == "__main__":
    unittest.main()
