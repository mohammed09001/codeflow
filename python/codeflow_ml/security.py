"""Untrusted-repository safety defaults."""
import re
from pathlib import Path


def safe_relative_path(root: str | Path, candidate: str | Path) -> Path:
    root_path = Path(root).resolve()
    resolved = (root_path / candidate).resolve()
    if resolved != root_path and root_path not in resolved.parents:
        raise ValueError("path escapes repository root")
    return resolved


def command_allowed(command: str, allowlist: set[str] = frozenset()) -> bool:
    return command in allowlist


def redact_secrets(message: str) -> str:
    return re.sub(r"(?i)(token|password|secret|api[_-]?key)=\S+", r"\1=[REDACTED]", message)


def network_offline(environment: dict[str, str] | None = None) -> bool:
    return (environment or {}).get("CODEFLOW_OFFLINE", "1") != "0"
