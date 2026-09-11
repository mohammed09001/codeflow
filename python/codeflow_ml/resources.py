"""Explicit resource governance primitives for bounded local execution."""
from dataclasses import dataclass
import os


@dataclass(frozen=True)
class ResourceBudget:
    max_items: int = 10000
    max_bytes: int = 64 * 1024 * 1024
    concurrency: int = 1

    def validate(self) -> None:
        if min(self.max_items, self.max_bytes, self.concurrency) <= 0:
            raise ValueError("resource limits must be positive")


def size_tier(items: int) -> str:
    return "small" if items < 1000 else "medium" if items < 10000 else "large"


def bounded_batch(items: list[object], budget: ResourceBudget) -> tuple[tuple[object, ...], ...]:
    budget.validate()
    return tuple(tuple(items[index:index + budget.max_items]) for index in range(0, len(items), budget.max_items))


def select_device(prefer_gpu: bool = True) -> str:
    return "cuda" if prefer_gpu and os.environ.get("CUDA_VISIBLE_DEVICES", "") not in ("", "-1") else "cpu"


def progress_events(total: int, cancel_after: int | None = None):
    for completed in range(total + 1):
        if cancel_after is not None and completed >= cancel_after:
            yield {"completed": completed, "total": total, "cancelled": True}
            return
        yield {"completed": completed, "total": total, "cancelled": False}
