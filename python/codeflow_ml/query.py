"""Bounded deterministic graph view query primitives for backend clients."""
from collections import deque
import json


def overview(nodes: list[dict], edges: list[dict], budget: int = 100) -> dict:
    ordered = sorted(nodes, key=lambda node: node.get("id", ""))[:budget]
    ids = {node.get("id") for node in ordered}
    return {"nodes": ordered, "edges": sorted((edge for edge in edges if edge.get("source") in ids and edge.get("target") in ids), key=lambda edge: (edge.get("source", ""), edge.get("target", ""))), "schema_version": 1}


def expand_node(node_id: str, edges: list[dict], depth: int = 1, budget: int = 100) -> tuple[str, ...]:
    seen = {node_id}
    queue = deque([(node_id, 0)])
    while queue and len(seen) < budget:
        node, level = queue.popleft()
        if level >= depth:
            continue
        for edge in sorted(edges, key=lambda item: (item.get("source", ""), item.get("target", ""))):
            if edge.get("source") == node:
                target = edge.get("target")
                if target not in seen:
                    seen.add(target)
                    queue.append((target, level + 1))
    return tuple(sorted(seen))


def dependency_path(source: str, target: str, edges: list[dict], budget: int = 100) -> tuple[str, ...] | None:
    queue = deque([(source, (source,))])
    while queue and budget > 0:
        node, path = queue.popleft(); budget -= 1
        if node == target:
            return path
        for edge in sorted(edges, key=lambda item: (item.get("source", ""), item.get("target", ""))):
            if edge.get("source") == node and edge.get("target") not in path:
                queue.append((edge["target"], path + (edge["target"],)))
    return None


def reverse_impact(node_id: str, edges: list[dict]) -> tuple[str, ...]:
    return tuple(sorted({edge["source"] for edge in edges if edge.get("target") == node_id}))


def revision_diff(before: set[str], after: set[str]) -> dict[str, list[str]]:
    return {"added": sorted(after - before), "removed": sorted(before - after), "unchanged": sorted(before & after)}


def evidence_view(records: list[dict], source_id: str) -> str:
    return json.dumps(sorted((record for record in records if record.get("source_id") == source_id), key=lambda record: record.get("evidence_id", "")), sort_keys=True, separators=(",", ":"))
