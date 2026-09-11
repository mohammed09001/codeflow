"""Stable, bounded hierarchy materialization for semantic zoom clients."""
from dataclasses import dataclass


@dataclass(frozen=True)
class ZoomNode:
    node_id: str
    level: str
    parent: str | None = None
    weight: float = 1.0


def materialize_zoom(nodes: list[ZoomNode], level: str, budget: int = 100) -> tuple[ZoomNode, ...]:
    if budget < 0:
        raise ValueError("budget must be non-negative")
    selected = [node for node in nodes if node.level == level]
    return tuple(sorted(selected, key=lambda node: (-node.weight, node.node_id))[:budget])


def bundle_edges(edges: list[tuple[str, str, float]]) -> dict[tuple[str, str], float]:
    bundled: dict[tuple[str, str], float] = {}
    for source, target, weight in edges:
        key = (source, target)
        bundled[key] = bundled.get(key, 0.0) + weight
    return dict(sorted(bundled.items()))


def stable_parent(memberships: dict[str, list[str]]) -> dict[str, str | None]:
    return {node: (sorted(parents)[0] if parents else None) for node, parents in sorted(memberships.items())}
