"""Small dependency-free, deterministic classical ML baselines."""
from __future__ import annotations

from dataclasses import dataclass
from math import exp, log, pi
from time import perf_counter_ns
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


@dataclass(frozen=True)
class SignalVector:
    structural: float = 0.0
    lexical: float = 0.0
    organizational: float = 0.0
    runtime: float = 0.0
    classical_ml: float = 0.0
    deep_learning: float = 0.0

    def values(self) -> tuple[float, ...]:
        return (self.structural, self.lexical, self.organizational, self.runtime, self.classical_ml, self.deep_learning)


def fuse_signals(signal: SignalVector, weights: Sequence[float] | None = None) -> tuple[float, bool]:
    values = signal.values()
    weights = tuple(weights) if weights is not None else adaptive_weights([1.0] * len(values))
    if len(weights) != len(values):
        raise ValueError("signal weights differ")
    score = sum(value * weight for value, weight in zip(values, weights))
    spread = max(values, default=0.0) - min(values, default=0.0)
    return max(0.0, min(1.0, score)), spread > 0.7


def calibration_curve(predicted: Sequence[float], observed: Sequence[bool], bins: int = 10) -> tuple[tuple[float, float, int], ...]:
    if len(predicted) != len(observed) or bins <= 0:
        raise ValueError("calibration inputs invalid")
    result = []
    for index in range(bins):
        members = [(score, truth) for score, truth in zip(predicted, observed) if min(bins - 1, int(score * bins)) == index]
        if members:
            result.append((sum(score for score, _ in members) / len(members), sum(truth for _, truth in members) / len(members), len(members)))
    return tuple(result)


def evidence_summary(signal: SignalVector) -> dict[str, object]:
    score, contradictory = fuse_signals(signal)
    return {"score": score, "contradictory": contradictory, "signals": signal.values()}


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
    """Gaussian naive Bayes for continuous, normalized feature matrices."""

    def fit(self, matrix: FeatureMatrix, labels: Sequence[str]) -> "NaiveBayes":
        if len(labels) != len(matrix.values):
            raise ValueError("labels and rows differ")
        self.classes = tuple(sorted(set(labels)))
        self.priors = {label: labels.count(label) / len(labels) for label in self.classes} if labels else {}
        self.means = {}
        self.variances = {}
        for label in self.classes:
            rows = [row for row, item in zip(matrix.values, labels) if item == label]
            means = tuple(sum(row[index] for row in rows) / len(rows) for index in range(len(rows[0])))
            self.means[label] = means
            self.variances[label] = tuple(
                max(1e-9, sum((row[index] - means[index]) ** 2 for row in rows) / len(rows))
                for index in range(len(means))
            )
        return self

    def predict_log_proba(self, row: Sequence[float]) -> dict[str, float]:
        if not self.classes:
            return {}
        return {
            label: log(self.priors[label]) - sum(
                0.5 * log(2.0 * pi * variance) + (float(value) - mean) ** 2 / (2.0 * variance)
                for value, mean, variance in zip(row, self.means[label], self.variances[label])
            )
            for label in self.classes
        }

    def predict_proba(self, row: Sequence[float]) -> dict[str, float]:
        scores = self.predict_log_proba(row)
        if not scores:
            return {}
        maximum = max(scores.values())
        weights = {label: exp(score - maximum) for label, score in scores.items()}
        total = sum(weights.values())
        return {label: weight / total for label, weight in weights.items()}

    def predict(self, row: Sequence[float]) -> str | None:
        probabilities = self.predict_proba(row)
        return max(probabilities, key=lambda label: (probabilities[label], label)) if probabilities else None


class LogisticBaseline:
    def fit(self, matrix: FeatureMatrix, labels: Sequence[int], epochs: int = 100, rate: float = 0.1, l2: float = 1e-4, tolerance: float = 1e-8) -> "LogisticBaseline":
        if len(labels) != len(matrix.values):
            raise ValueError("labels and rows differ")
        width = len(matrix.values[0]) if matrix.values else 0
        self.weights = [0.0] * (width + 1)
        self.converged = False
        for _ in range(epochs):
            largest_update = 0.0
            for row, label in zip(matrix.values, labels):
                p = self.predict_proba(row)
                error = float(label) - p
                update = rate * error
                self.weights[0] += update
                largest_update = max(largest_update, abs(update))
                for i, value in enumerate(row, 1):
                    update = rate * (error * value - l2 * self.weights[i])
                    self.weights[i] += update
                    largest_update = max(largest_update, abs(update))
            if largest_update < tolerance:
                self.converged = True
                break
        return self

    def predict_proba(self, row: Sequence[float]) -> float:
        score = self.weights[0] + sum(weight * float(value) for weight, value in zip(self.weights[1:], row))
        return 1.0 / (1.0 + exp(-max(-60.0, min(60.0, score))))


def project_split(rows: Sequence[tuple[str, object]], validation_projects: Sequence[str], test_projects: Sequence[str]) -> tuple[tuple[object, ...], tuple[object, ...], tuple[object, ...]]:
    """Split by repository/project identity, never by individual entities."""
    validation, test = set(validation_projects), set(test_projects)
    if validation.intersection(test):
        raise ValueError("validation and test projects overlap")
    train_rows, validation_rows, test_rows = [], [], []
    for project, row in rows:
        (validation_rows if project in validation else test_rows if project in test else train_rows).append(row)
    return tuple(train_rows), tuple(validation_rows), tuple(test_rows)


def classification_metrics(actual: Sequence[str], predicted: Sequence[str]) -> dict[str, object]:
    if len(actual) != len(predicted) or not actual:
        raise ValueError("metrics require paired non-empty labels")
    labels = tuple(sorted(set(actual).union(predicted)))
    per_class = {}
    f1 = []
    for label in labels:
        tp = sum(a == label and p == label for a, p in zip(actual, predicted))
        fp = sum(a != label and p == label for a, p in zip(actual, predicted))
        fn = sum(a == label and p != label for a, p in zip(actual, predicted))
        precision, recall = tp / (tp + fp) if tp + fp else 0.0, tp / (tp + fn) if tp + fn else 0.0
        per_class[label] = {"precision": precision, "recall": recall}
        f1.append(2 * precision * recall / (precision + recall) if precision + recall else 0.0)
    accuracy = sum(a == p for a, p in zip(actual, predicted)) / len(actual)
    return {"accuracy": accuracy, "micro_f1": accuracy, "macro_f1": sum(f1) / len(f1), "per_class": per_class}


def brier_score(probabilities: Sequence[float], outcomes: Sequence[bool]) -> float:
    if len(probabilities) != len(outcomes) or not probabilities:
        raise ValueError("Brier score requires paired probabilities")
    if any(score < 0.0 or score > 1.0 for score in probabilities):
        raise ValueError("probabilities must be bounded")
    return sum((score - float(outcome)) ** 2 for score, outcome in zip(probabilities, outcomes)) / len(probabilities)


def pr_auc(scores: Sequence[float], outcomes: Sequence[bool]) -> float:
    if len(scores) != len(outcomes) or not scores:
        raise ValueError("PR-AUC requires paired scores")
    positives = sum(outcomes)
    if not positives:
        return 0.0
    ranked = sorted(zip(scores, outcomes), key=lambda item: item[0], reverse=True)
    area = previous_recall = 0.0
    hits = 0
    for index, (_, outcome) in enumerate(ranked, 1):
        hits += int(outcome)
        recall = hits / positives
        precision = hits / index
        area += (recall - previous_recall) * precision
        previous_recall = recall
    return area


def inference_latency_ns(predict, rows: Sequence[Sequence[float]]) -> int:
    start = perf_counter_ns()
    for row in rows:
        predict(row)
    return perf_counter_ns() - start


def linear_rank(features: Sequence[Sequence[float]], weights: Sequence[float]) -> list[int]:
    scores = [sum(float(value) * float(weight) for value, weight in zip(row, weights)) for row in features]
    return sorted(range(len(scores)), key=lambda index: (-scores[index], index))


@dataclass(frozen=True)
class DecisionStump:
    feature: int
    threshold: float
    left_label: str
    right_label: str


class DecisionTree:
    """Deterministic CART-style one-level tree; unlike a weighted rank it learns a split."""

    def fit(self, matrix: FeatureMatrix, labels: Sequence[str]) -> "DecisionTree":
        if len(matrix.values) != len(labels) or not matrix.values:
            raise ValueError("tree requires labelled rows")
        candidates: list[tuple[float, int, float, str, str]] = []
        for feature in range(len(matrix.values[0])):
            values = sorted(set(row[feature] for row in matrix.values))
            for low, high in zip(values, values[1:]):
                threshold = (low + high) / 2.0
                left = [label for row, label in zip(matrix.values, labels) if row[feature] <= threshold]
                right = [label for row, label in zip(matrix.values, labels) if row[feature] > threshold]
                if not left or not right:
                    continue
                left_label = max(sorted(set(left)), key=left.count)
                right_label = max(sorted(set(right)), key=right.count)
                error = sum((label != (left_label if row[feature] <= threshold else right_label)) for row, label in zip(matrix.values, labels)) / len(labels)
                candidates.append((error, feature, threshold, left_label, right_label))
        if not candidates:
            label = max(sorted(set(labels)), key=labels.count)
            self.stump = DecisionStump(0, float("inf"), label, label)
        else:
            _, feature, threshold, left_label, right_label = min(candidates)
            self.stump = DecisionStump(feature, threshold, left_label, right_label)
        return self

    def predict(self, row: Sequence[float]) -> str:
        return self.stump.left_label if float(row[self.stump.feature]) <= self.stump.threshold else self.stump.right_label


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
