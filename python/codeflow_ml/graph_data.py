"""Dependency-free heterogeneous graph tensor mapping (PyG-compatible shape)."""
from dataclasses import dataclass
import hashlib
import math


@dataclass(frozen=True)
class HeteroGraphData:
    node_ids: tuple[str, ...]
    node_types: tuple[str, ...]
    node_features: tuple[tuple[float, ...], ...]
    edge_index: tuple[tuple[int, int], ...]
    edge_types: tuple[str, ...]


def lexical_hash(text: str, width: int = 16) -> tuple[float, ...]:
    values = [0.0] * width
    for token in text.lower().replace("_", " ").split():
        values[int(hashlib.sha256(token.encode()).hexdigest(), 16) % width] += 1.0
    norm = math.sqrt(sum(value * value for value in values)) or 1.0
    return tuple(value / norm for value in values)


def normalize_scalars(values: list[float]) -> tuple[float, ...]:
    if not values:
        return ()
    low, high = min(values), max(values)
    if high == low:
        return tuple(0.0 for _ in values)
    return tuple((value - low) / (high - low) for value in values)


def build_heterodata(nodes: list[tuple[str, str, str]], edges: list[tuple[str, str, str]]) -> HeteroGraphData:
    ordered = sorted(nodes)
    ids = tuple(item[0] for item in ordered)
    positions = {node_id: index for index, node_id in enumerate(ids)}
    valid_edges = sorted((positions[source], positions[target], kind) for source, target, kind in edges if source in positions and target in positions)
    return HeteroGraphData(ids, tuple(item[1] for item in ordered), tuple(lexical_hash(item[2]) for item in ordered), tuple((source, target) for source, target, _ in valid_edges), tuple(kind for _, _, kind in valid_edges))
