"""Bounded deterministic workflow reconstruction over canonical graph edges."""
from dataclasses import dataclass


@dataclass(frozen=True)
class WorkflowStep:
    entity_id: str
    relation: str = "call"
    side_effect: str | None = None
    hag_id: str | None = None
    confidence: float = 1.0


def entry_points(adjacency: dict[str, tuple[str, ...]], explicit: tuple[str, ...] = ()) -> tuple[str, ...]:
    incoming = {target for targets in adjacency.values() for target in targets}
    seeds = set(explicit) if explicit else set(adjacency) - incoming
    if not seeds:
        seeds = set(adjacency)
    return tuple(sorted(seeds))


def reconstruct_workflows(adjacency: dict[str, tuple[str, ...]], starts: tuple[str, ...] = (), max_depth: int = 12, max_paths: int = 128) -> tuple[tuple[str, ...], ...]:
    results: list[tuple[str, ...]] = []
    for seed in entry_points(adjacency, starts):
        stack = [(seed, (seed,))]
        while stack and len(results) < max_paths:
            node, path = stack.pop()
            next_nodes = tuple(sorted(set(adjacency.get(node, ()))))
            if len(path) >= max_depth or not next_nodes:
                results.append(path)
                continue
            for target in reversed(next_nodes):
                if target in path:  # SCC/recursion summary: retain the bounded path, do not expand forever.
                    results.append(path + (target,))
                else:
                    stack.append((target, path + (target,)))
    return tuple(results)


def rank_workflows(paths: tuple[tuple[str, ...], ...], exceptional_nodes: set[str] | None = None) -> tuple[tuple[str, ...], ...]:
    exceptional_nodes = exceptional_nodes or set()
    return tuple(sorted(paths, key=lambda path: (-sum(node in exceptional_nodes for node in path), len(path), path)))


def bind_side_effects(path: tuple[str, ...], effects: dict[str, str]) -> tuple[WorkflowStep, ...]:
    return tuple(WorkflowStep(node, side_effect=effects.get(node)) for node in path)
