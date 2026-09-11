"""Structured diagnostics and recoverable run quarantine."""
from dataclasses import dataclass, asdict
import json
import time
import uuid


@dataclass(frozen=True)
class Diagnostic:
    level: str
    message: str
    correlation_id: str
    capability: str | None = None
    duration_ms: float | None = None


def correlation_id() -> str:
    return uuid.uuid4().hex


def timed_diagnostic(level: str, message: str, correlation: str, started: float, capability: str | None = None) -> Diagnostic:
    return Diagnostic(level, message, correlation, capability, round((time.monotonic() - started) * 1000, 3))


def quarantine_run(run_id: str, reason: str) -> str:
    return json.dumps({"run_id": run_id, "status": "quarantined", "reason": reason}, sort_keys=True)


def diagnostic_bundle(records: list[Diagnostic]) -> str:
    return json.dumps([asdict(record) for record in records], sort_keys=True, separators=(",", ":"))
