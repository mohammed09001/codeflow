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


@dataclass
class TrainableGraphEncoder:
    """Small deterministic CPU graph autoencoder with trainable node vectors."""
    embeddings: list[list[float]]

    @classmethod
    def create(cls, graph: HeteroGraphData, dimensions: int = 8, seed: int = 0) -> "TrainableGraphEncoder":
        return cls([[(int(hashlib.sha256(f"{seed}:{node}:{index}".encode()).hexdigest()[:8], 16) % 2001 - 1000) / 10_000.0 for index in range(dimensions)] for node in graph.node_ids])

    def parameters(self) -> tuple[tuple[float, ...], ...]:
        return tuple(tuple(row) for row in self.embeddings)

    def train_epoch(self, graph: HeteroGraphData, rate: float = 0.1) -> float:
        """Minimise positive-edge reconstruction loss with exact SGD gradients."""
        if not graph.edge_index:
            return 0.0
        total = 0.0
        for left, right in graph.edge_index:
            score = sum(a * b for a, b in zip(self.embeddings[left], self.embeddings[right]))
            probability = 1.0 / (1.0 + math.exp(-max(-30.0, min(30.0, score))))
            error = probability - 1.0
            total += -math.log(max(probability, 1e-12))
            old_left, old_right = self.embeddings[left][:], self.embeddings[right][:]
            for index in range(len(old_left)):
                self.embeddings[left][index] -= rate * error * old_right[index]
                self.embeddings[right][index] -= rate * error * old_left[index]
        return total / len(graph.edge_index)

    def encode(self, graph: HeteroGraphData) -> dict[str, tuple[float, ...]]:
        return {node: tuple(self.embeddings[index]) for index, node in enumerate(graph.node_ids)}


@dataclass
class TrainableGcn(TrainableGraphEncoder):
    """CPU GCN-style trainable encoder: aggregate neighbours then optimise edges."""

    def train_epoch(self, graph: HeteroGraphData, rate: float = 0.1) -> float:
        neighbors = [[] for _ in graph.node_ids]
        for left, right in graph.edge_index:
            neighbors[left].append(right)
            neighbors[right].append(left)
        before = [row[:] for row in self.embeddings]
        for index, adjacent in enumerate(neighbors):
            if adjacent:
                self.embeddings[index] = [sum(before[node][feature] for node in adjacent) / len(adjacent) for feature in range(len(before[index]))]
        return super().train_epoch(graph, rate)


@dataclass
class TrainableGat(TrainableGraphEncoder):
    """CPU GAT-style trainable encoder with score-derived attention weights."""

    def train_epoch(self, graph: HeteroGraphData, rate: float = 0.1) -> float:
        neighbors = [[] for _ in graph.node_ids]
        for left, right in graph.edge_index:
            neighbors[left].append(right)
            neighbors[right].append(left)
        before = [row[:] for row in self.embeddings]
        for index, adjacent in enumerate(neighbors):
            if adjacent:
                scores = [sum(a * b for a, b in zip(before[index], before[node])) for node in adjacent]
                peak = max(scores)
                weights = [math.exp(score - peak) for score in scores]
                normalizer = sum(weights)
                self.embeddings[index] = [sum(weight * before[node][feature] for weight, node in zip(weights, adjacent)) / normalizer for feature in range(len(before[index]))]
        return super().train_epoch(graph, rate)


@dataclass
class TrainableHeteroGnn(TrainableGraphEncoder):
    """Trainable relation-aware encoder with learned per-edge-type scalar gates."""
    relation_gates: dict[str, float] | None = None

    @classmethod
    def create(cls, graph: HeteroGraphData, dimensions: int = 8, seed: int = 0) -> "TrainableHeteroGnn":
        base = TrainableGraphEncoder.create(graph, dimensions, seed)
        return cls(base.embeddings, {edge_type: 1.0 for edge_type in sorted(set(graph.edge_types))})

    def train_epoch(self, graph: HeteroGraphData, rate: float = 0.1) -> float:
        assert self.relation_gates is not None
        loss = super().train_epoch(graph, rate)
        for (left, right), edge_type in zip(graph.edge_index, graph.edge_types):
            score = sum(a * b for a, b in zip(self.embeddings[left], self.embeddings[right]))
            gate = self.relation_gates[edge_type]
            self.relation_gates[edge_type] -= rate * (1.0 / (1.0 + math.exp(-score * gate)) - 1.0) * score
        return loss


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


def legacy_degree_embedding(graph: HeteroGraphData, dimensions: int = 8) -> dict[str, tuple[float, ...]]:
    """Non-learning degree heuristic retained only for compatibility diagnostics."""
    degree = {node: 0 for node in graph.node_ids}
    for source, target in graph.edge_index:
        degree[graph.node_ids[source]] += 1
        degree[graph.node_ids[target]] += 1
    return {node: tuple(((degree[node] + index + 1) % (dimensions + 3)) / (dimensions + 3) for index in range(dimensions)) for node in graph.node_ids}


def legacy_neighbor_average(graph: HeteroGraphData, dimensions: int = 8) -> dict[str, tuple[float, ...]]:
    """Non-learning neighbor average; this is not a GCN."""
    embeddings = legacy_degree_embedding(graph, dimensions)
    for source, target in graph.edge_index:
        left, right = graph.node_ids[source], graph.node_ids[target]
        embeddings[left] = tuple((a + b) / 2 for a, b in zip(embeddings[left], embeddings[right]))
    return embeddings


def legacy_attention_scale(graph: HeteroGraphData, dimensions: int = 8) -> dict[str, tuple[float, ...]]:
    """Non-learning fixed scaling; this is not a GAT."""
    return {node: tuple(value * (1.0 + (index % 3) / 10.0) for index, value in enumerate(vector)) for node, vector in legacy_neighbor_average(graph, dimensions).items()}


def legacy_type_offset(graph: HeteroGraphData, dimensions: int = 8) -> dict[str, tuple[float, ...]]:
    """Non-learning type offset; this is not a heterogeneous GNN."""
    return {node: tuple(value + (0.1 if graph.node_types[index] == "function" else 0.0) for value in vector) for index, (node, vector) in enumerate(legacy_attention_scale(graph, dimensions).items())}


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
