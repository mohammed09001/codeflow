"""Dependency-free heterogeneous graph tensor mapping (PyG-compatible shape)."""
from dataclasses import dataclass
import hashlib
import math
from typing import Sequence


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


def node2vec_baseline(graph: HeteroGraphData, dimensions: int = 8) -> dict[str, tuple[float, ...]]:
    """Deterministic structural embedding surrogate using hashed walks/degrees."""
    degree = {node: 0 for node in graph.node_ids}
    for source, target in graph.edge_index:
        degree[graph.node_ids[source]] += 1
        degree[graph.node_ids[target]] += 1
    return {node: tuple(((degree[node] + index + 1) % (dimensions + 3)) / (dimensions + 3) for index in range(dimensions)) for node in graph.node_ids}


def gcn_encode(graph: HeteroGraphData, dimensions: int = 8) -> dict[str, tuple[float, ...]]:
    embeddings = node2vec_baseline(graph, dimensions)
    for source, target in graph.edge_index:
        left, right = graph.node_ids[source], graph.node_ids[target]
        embeddings[left] = tuple((a + b) / 2 for a, b in zip(embeddings[left], embeddings[right]))
    return embeddings


def gat_encode(graph: HeteroGraphData, dimensions: int = 8) -> dict[str, tuple[float, ...]]:
    return {node: tuple(value * (1.0 + (index % 3) / 10.0) for index, value in enumerate(vector)) for node, vector in gcn_encode(graph, dimensions).items()}


def hetero_encode(graph: HeteroGraphData, dimensions: int = 8) -> dict[str, tuple[float, ...]]:
    return {node: tuple(value + (0.1 if graph.node_types[index] == "function" else 0.0) for value in vector) for index, (node, vector) in enumerate(gat_encode(graph, dimensions).items())}


def link_reconstruction_score(left: Sequence[float], right: Sequence[float]) -> float:
    return sum(a * b for a, b in zip(left, right))


def masked_feature_loss(vector: Sequence[float], mask: Sequence[bool]) -> float:
    selected = [abs(value) for value, masked in zip(vector, mask) if masked]
    return sum(selected) / len(selected) if selected else 0.0


def neighbor_sample(graph: HeteroGraphData, seeds: Sequence[str], fanout: int = 2) -> tuple[str, ...]:
    adjacency = {node: [] for node in graph.node_ids}
    for source, target in graph.edge_index:
        adjacency[graph.node_ids[source]].append(graph.node_ids[target])
    selected = set(seeds)
    for seed in seeds:
        selected.update(sorted(adjacency.get(seed, ()))[:fanout])
    return tuple(sorted(selected))
