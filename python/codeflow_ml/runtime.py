"""Optional runtime evidence intake; observed facts never replace static possibilities."""
from dataclasses import dataclass
import hashlib
import json


@dataclass(frozen=True)
class TraceEvent:
    timestamp_ns: int
    kind: str
    entity_id: str | None = None
    target_id: str | None = None
    state: str | None = None
    observed: bool = True
    evidence_id: str = ""

    def validate(self) -> None:
        if self.timestamp_ns < 0 or not self.kind:
            raise ValueError("invalid trace event")


def ingest_trace(payload: str | bytes, sample_rate: float = 1.0) -> tuple[TraceEvent, ...]:
    if not 0.0 <= sample_rate <= 1.0:
        raise ValueError("sample rate must be between 0 and 1")
    raw = json.loads(payload)
    if not isinstance(raw, list):
        raise ValueError("trace must be a list")
    events = []
    for index, item in enumerate(raw):
        event = TraceEvent(**item)
        event.validate()
        digest = int(hashlib.blake2b(f"{event.timestamp_ns}:{index}".encode(), digest_size=2).hexdigest(), 16) / 65535
        if digest <= sample_rate:
            events.append(event)
    return tuple(events)


def correlate_events(events: tuple[TraceEvent, ...], known_entities: set[str]) -> tuple[TraceEvent, ...]:
    return tuple(event for event in events if event.entity_id in known_entities or event.target_id in known_entities)


def redact_trace(events: tuple[TraceEvent, ...], secret_fields: set[str]) -> tuple[TraceEvent, ...]:
    return tuple(TraceEvent(event.timestamp_ns, event.kind, event.entity_id, event.target_id, None if "state" in secret_fields else event.state, event.observed, event.evidence_id) for event in events)


def normalize_sessions(events: tuple[TraceEvent, ...]) -> dict[str, tuple[TraceEvent, ...]]:
    sessions: dict[str, list[TraceEvent]] = {}
    for event in sorted(events, key=lambda item: (item.timestamp_ns, item.evidence_id)):
        sessions.setdefault(event.evidence_id or "default", []).append(event)
    return {key: tuple(value) for key, value in sorted(sessions.items())}


def compress_repeated(sequence: tuple[str, ...]) -> tuple[str, ...]:
    if not sequence:
        return ()
    result: list[str] = []
    index = 0
    while index < len(sequence):
        run = 1
        while index + run < len(sequence) and sequence[index + run] == sequence[index]:
            run += 1
        result.append(f"{sequence[index]}*{run}" if run > 1 else sequence[index])
        index += run
    return tuple(result)


def path_frequencies(sessions: dict[str, tuple[TraceEvent, ...]]) -> dict[tuple[str, ...], int]:
    frequencies: dict[tuple[str, ...], int] = {}
    for events in sessions.values():
        path = compress_repeated(tuple(event.entity_id or event.kind for event in events))
        frequencies[path] = frequencies.get(path, 0) + 1
    return dict(sorted(frequencies.items(), key=lambda item: (-item[1], item[0])))


def align_static_dynamic(static_paths: tuple[tuple[str, ...], ...], observed_paths: dict[tuple[str, ...], int]) -> dict[tuple[str, ...], float]:
    total = max(1, sum(observed_paths.values()))
    return {path: sum(count for observed, count in observed_paths.items() if set(path).issuperset(observed)) / total for path in static_paths}
