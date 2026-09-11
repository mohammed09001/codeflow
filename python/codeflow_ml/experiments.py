"""Reproducible offline experiment metadata and dataset lineage."""
from dataclasses import dataclass, asdict
import hashlib
import json
import platform


@dataclass(frozen=True)
class DatasetManifest:
    name: str
    version: str
    records: int
    checksum: str
    lineage: tuple[str, ...] = ()

    @classmethod
    def from_records(cls, name: str, version: str, records: list[dict], lineage: tuple[str, ...] = ()):
        payload = json.dumps(records, sort_keys=True, separators=(",", ":")).encode()
        return cls(name, version, len(records), hashlib.sha256(payload).hexdigest(), lineage)


@dataclass(frozen=True)
class RetentionProtocol:
    primary_metric: str
    validation_projects: tuple[str, ...]
    test_projects: tuple[str, ...]
    seed: int
    max_latency_ns: int
    min_macro_f1_gain: float = 0.03
    min_pr_auc_gain: float = 0.05

    def validate(self) -> None:
        if not self.primary_metric or set(self.validation_projects).intersection(self.test_projects):
            raise ValueError("invalid pre-registered retention protocol")


def retention_decision(protocol: RetentionProtocol, baseline: dict[str, float], learned: dict[str, float]) -> dict[str, object]:
    """Choose default DL only when its held-out gain and resource budget pass."""
    protocol.validate()
    latency_ok = learned.get("latency_ns", float("inf")) <= protocol.max_latency_ns
    macro_gain = learned.get("macro_f1", 0.0) - baseline.get("macro_f1", 0.0)
    pr_gain = learned.get("pr_auc", 0.0) - baseline.get("pr_auc", 0.0)
    retained = latency_ok and (macro_gain >= protocol.min_macro_f1_gain or pr_gain >= protocol.min_pr_auc_gain)
    return {"default": retained, "decision": "retain_default" if retained else "optional_experimental", "macro_f1_gain": macro_gain, "pr_auc_gain": pr_gain, "latency_ok": latency_ok}


def deterministic_split(record_ids: list[str], seed: int = 0) -> tuple[tuple[str, ...], tuple[str, ...], tuple[str, ...]]:
    ordered = sorted(record_ids, key=lambda item: hashlib.sha256(f"{seed}:{item}".encode()).hexdigest())
    n = len(ordered)
    return tuple(ordered[: int(n * 0.6)]), tuple(ordered[int(n * 0.6): int(n * 0.8)]), tuple(ordered[int(n * 0.8):])


def experiment_record(manifest: DatasetManifest, seed: int = 0) -> dict:
    return {"dataset": asdict(manifest), "seed": seed, "python": platform.python_version(), "platform": platform.platform()}
