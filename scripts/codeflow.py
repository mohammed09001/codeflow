"""Minimal offline CodeFlow CLI boundary; analysis remains delegated to Rust engines."""
import argparse
import json
from pathlib import Path


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(prog="codeflow")
    parser.add_argument("command", choices=("init", "analyze", "watch", "status", "graph", "evidence"))
    parser.add_argument("root", nargs="?", default=".")
    args = parser.parse_args(argv)
    root = Path(args.root).resolve()
    if args.command == "init":
        (root / ".codeflow").mkdir(exist_ok=True)
        (root / ".codeflow" / "project.json").write_text(json.dumps({"schema_version": 1, "root": str(root)}, sort_keys=True))
    print(json.dumps({"command": args.command, "root": str(root), "api_version": "v1"}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
