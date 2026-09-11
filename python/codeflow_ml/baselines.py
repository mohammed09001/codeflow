"""Small dependency-free, deterministic classical ML baselines."""
from __future__ import annotations

from dataclasses import dataclass
from math import exp
from typing import Iterable, Sequence


@dataclass(frozen=True)
class FeatureMatrix:
    entity_ids: tuple[str, ...]
    values: tuple[tuple[float, ...], ...]

    @classmethod
    def from_rows(cls, rows: Iterable[tuple[str, Sequence[float]]]) -> "FeatureMatrix":
        materialized = tuple((entity, tuple(float(value) for value in values)) for entity, values in rows)
        width = len(materialized[0][1]) if materialized else 0
        if any(len(values) != width for _, values in materialized):
            raise ValueError("feature rows must have equal width")
        return cls(tuple(entity for entity, _ in materialized), tuple(values for _, values in materialized))


@dataclass(frozen=True)
class FeatureCandidate:
    feature_id: str
    entity_id: str
    structural: float
    lexical: float
    entry_point: float = 0.0
    side_effect: float = 0.0
    scenario: float = 0.0
    unknown: bool = False

    @property
    def relevance(self) -> float:
        if self.unknown:
            return 0.0
        return signal_quality((self.structural, self.lexical, self.entry_point, self.side_effect, self.scenario))


def seed_feature(entry_point: str, entity_id: str, feature_id: str) -> FeatureCandidate:
    return FeatureCandidate(feature_id, entity_id, 0.0, 0.0, entry_point=1.0)


def merge_feature_candidates(candidates: Sequence[FeatureCandidate]) -> tuple[FeatureCandidate, ...]:
    best: dict[tuple[str, str], FeatureCandidate] = {}
    for candidate in candidates:
        key = (candidate.feature_id, candidate.entity_id)
        old = best.get(key)
        if old is None or candidate.relevance > old.relevance:
            best[key] = candidate
    return tuple(sorted(best.values(), key=lambda item: (-item.relevance, item.feature_id, item.entity_id)))


def infrastructure_centrality(degrees: dict[str, tuple[int, int]]) -> dict[str, float]:
    """Normalize total degree into a stable infrastructure-candidate score."""
    maximum = max((incoming + outgoing for incoming, outgoing in degrees.values()), default=0)
    return {entity: ((incoming + outgoing) / maximum if maximum else 0.0) for entity, (incoming, outgoing) in sorted(degrees.items())}


def domain_specificity(identifier: str, domain_terms: Sequence[str], utility_terms: Sequence[str] = ()) -> float:
    tokens = set(identifier.lower().replace("_", " ").split())
    domain = len(tokens.intersection(term.lower() for term in domain_terms))
    utility = len(tokens.intersection(term.lower() for term in utility_terms))
    return max(0.0, min(1.0, (domain - utility + 1) / 2.0))


def feature_participation(entity_features: dict[str, Sequence[str]]) -> dict[str, int]:
    return {entity: len(set(features)) for entity, features in sorted(entity_features.items())}


def classify_shared_name(identifier: str) -> str:
    lowered = identifier.lower()
    if any(term in lowered for term in ("db", "store", "repository", "cache")):
        return "datastore"
    if any(term in lowered for term in ("event", "message", "queue", "publish", "subscribe")):
        return "messaging"
    if any(term in lowered for term in ("util", "helper", "common", "shared")):
        return "utility"
    return "domain_service"


class NaiveBayes:
    def fit(self, matrix: FeatureMatrix, labels: Sequence[str]) -> "NaiveBayes":
        if len(labels) != len(matrix.values):
            raise ValueError("labels and rows differ")
        self.classes = tuple(sorted(set(labels)))
        self.means = {label: tuple(sum(row[i] for row, item in zip(matrix.values, labels) if item == label) / max(1, labels.count(label)) for i in range(len(matrix.values[0]))) for label in self.classes} if matrix.values else {}
        return self

    def predict(self, row: Sequence[float]) -> str | None:
        if not self.classes:
            return None
        return min(self.classes, key=lambda label: (sum((float(value) - self.means[label][i]) ** 2 for i, value in enumerate(row)), label))


class LogisticBaseline:
    def fit(self, matrix: FeatureMatrix, labels: Sequence[int], epochs: int = 100, rate: float = 0.1) -> "LogisticBaseline":
        if len(labels) != len(matrix.values):
            raise ValueError("labels and rows differ")
        width = len(matrix.values[0]) if matrix.values else 0
        self.weights = [0.0] * (width + 1)
        for _ in range(epochs):
            for row, label in zip(matrix.values, labels):
                p = self.predict_proba(row)
                error = float(label) - p
                self.weights[0] += rate * error
                for i, value in enumerate(row, 1):
                    self.weights[i] += rate * error * value
        return self

    def predict_proba(self, row: Sequence[float]) -> float:
        score = self.weights[0] + sum(weight * float(value) for weight, value in zip(self.weights[1:], row))
        return 1.0 / (1.0 + exp(-max(-60.0, min(60.0, score))))


def tree_rank(features: Sequence[Sequence[float]], weights: Sequence[float]) -> list[int]:
    scores = [sum(float(value) * float(weight) for value, weight in zip(row, weights)) for row in features]
    return sorted(range(len(scores)), key=lambda index: (-scores[index], index))


def clustering_ensemble(partitions: Sequence[Sequence[int]]) -> tuple[float, ...]:
    if not partitions:
        return ()
    width = len(partitions[0])
    return tuple(sorted((sum(partition[i] for partition in partitions) / len(partitions) for i in range(width))))


def signal_quality(values: Sequence[float]) -> float:
    if not values:
        return 0.0
    return max(0.0, min(1.0, sum(max(0.0, min(1.0, value)) for value in values) / len(values)))


def adaptive_weights(values: Sequence[float]) -> tuple[float, ...]:
    total = sum(max(0.0, value) for value in values)
    return tuple((max(0.0, value) / total if total else 1.0 / len(values)) for value in values)
