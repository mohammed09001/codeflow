#!/usr/bin/env python3
"""Fail deterministically when CodeFlow source declares an LLM dependency."""
from __future__ import annotations

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
EXCLUDED = {".git", "target", ".venv", "__pycache__", "Execution", "scripts"}
BANNED = ("openai", "anthropic", "google-generativeai", "google.genai", "langchain", "litellm")
SUFFIXES = {".py", ".rs", ".toml", ".json", ".yaml", ".yml", ".lock"}

def main() -> int:
    findings: list[str] = []
    source_roots = [ROOT / name for name in ("crates", "python", "adapters")]
    manifest_paths = [ROOT / name for name in ("Cargo.toml", "pyproject.toml", "uv.lock")]
    paths = [path for root in source_roots if root.exists() for path in root.rglob("*")]
    paths.extend(path for path in manifest_paths if path.exists())
    for path in sorted(paths):
        if not path.is_file() or any(part in EXCLUDED for part in path.parts) or path.suffix not in SUFFIXES:
            continue
        text = path.read_text(encoding="utf-8", errors="replace").lower()
        for token in BANNED:
            if token in text:
                findings.append(f"{path.relative_to(ROOT)}: banned dependency token {token!r}")
    if findings:
        print("\n".join(findings), file=sys.stderr)
        return 1
    print("no-LLM compliance: PASS")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
