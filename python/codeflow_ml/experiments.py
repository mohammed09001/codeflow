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


def deterministic_split(record_ids: list[str], seed: int = 0) -> tuple[tuple[str, ...], tuple[str, ...], tuple[str, ...]]:
    ordered = sorted(record_ids, key=lambda item: hashlib.sha256(f"{seed}:{item}".encode()).hexdigest())
    n = len(ordered)
    return tuple(ordered[: int(n * 0.6)]), tuple(ordered[int(n * 0.6): int(n * 0.8)]), tuple(ordered[int(n * 0.8):])


def experiment_record(manifest: DatasetManifest, seed: int = 0) -> dict:
    return {"dataset": asdict(manifest), "seed": seed, "python": platform.python_version(), "platform": platform.platform()}
