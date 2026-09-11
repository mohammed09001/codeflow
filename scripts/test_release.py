import json
from pathlib import Path


def main() -> int:
    manifest = json.loads(Path("packaging/release-artifacts.json").read_text())
    assert manifest["schema_version"] == 1
    assert {artifact["os"] for artifact in manifest["artifacts"]} == {"windows", "macos", "linux"}
    print("release matrix: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
