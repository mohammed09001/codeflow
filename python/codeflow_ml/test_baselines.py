import unittest

from .baselines import DecisionTree, FeatureCandidate, FeatureMatrix, LogisticBaseline, NaiveBayes, SignalVector, adaptive_weights, brier_score, calibration_curve, classification_metrics, classify_shared_name, clustering_ensemble, domain_specificity, evidence_summary, feature_participation, fuse_signals, inference_latency_ns, infrastructure_centrality, linear_rank, merge_feature_candidates, pr_auc, project_split, seed_feature, signal_quality


class BaselineTests(unittest.TestCase):
    def test_reproducible_baselines(self):
        matrix = FeatureMatrix.from_rows([("a", [1, 0]), ("b", [0, 1])])
        self.assertEqual(NaiveBayes().fit(matrix, ["x", "y"]).predict([1, 0]), "x")
        model = LogisticBaseline().fit(matrix, [1, 0], epochs=20)
        self.assertGreater(model.predict_proba([1, 0]), model.predict_proba([0, 1]))
        self.assertEqual(linear_rank([[1, 0], [0, 1]], [1, 0]), [0, 1])
        self.assertEqual(clustering_ensemble([[0, 1], [0, 1]]), (0.0, 1.0))
        self.assertAlmostEqual(signal_quality([0.5, 1.0]), 0.75)
        self.assertEqual(adaptive_weights([1, 3]), (0.25, 0.75))

    def test_invalid_matrix_is_rejected(self):
        with self.assertRaises(ValueError):
            FeatureMatrix.from_rows([("a", [1]), ("b", [1, 2])])

    def test_feature_candidates_are_many_to_many_and_unknown_safe(self):
        seed = seed_feature("route:/checkout", "fn:checkout", "checkout")
        rich = FeatureCandidate("checkout", "fn:checkout", 1.0, 0.8, side_effect=0.7)
        unknown = FeatureCandidate("unknown", "fn:x", 1.0, 1.0, unknown=True)
        merged = merge_feature_candidates([seed, rich, unknown])
        self.assertEqual(len(merged), 2)
        self.assertEqual(merged[0].entity_id, "fn:checkout")
        self.assertEqual(merged[1].relevance, 0.0)

    def test_infrastructure_and_cross_cutting_scoring(self):
        scores = infrastructure_centrality({"shared": (10, 10), "domain": (1, 1)})
        self.assertEqual(scores["shared"], 1.0)
        self.assertGreater(domain_specificity("checkout_service", ["checkout"]), domain_specificity("common_util", ["checkout"], ["util"]))
        self.assertEqual(feature_participation({"db": ["a", "a", "b"]})["db"], 2)
        self.assertEqual(classify_shared_name("event_bus"), "messaging")

    def test_signal_fusion_calibration_and_contradictions(self):
        score, contradictory = fuse_signals(SignalVector(structural=1.0, lexical=0.0))
        self.assertAlmostEqual(score, 1 / 6)
        self.assertTrue(contradictory)
        self.assertEqual(len(calibration_curve([0.1, 0.9], [False, True])), 2)
        self.assertIn("signals", evidence_summary(SignalVector()))

    def test_probabilistic_nb_and_tree_are_not_old_surrogates(self):
        matrix = FeatureMatrix.from_rows([("a", [-2]), ("b", [-1]), ("c", [1]), ("d", [2])])
        labels = ["left", "left", "right", "right"]
        nb = NaiveBayes().fit(matrix, labels)
        probabilities = nb.predict_proba([1.5])
        self.assertAlmostEqual(sum(probabilities.values()), 1.0)
        self.assertEqual(nb.predict([1.5]), "right")
        tree = DecisionTree().fit(matrix, labels)
        self.assertEqual(tree.predict([-1.5]), "left")
        self.assertEqual(tree.predict([1.5]), "right")

    def test_project_split_and_metrics_do_not_leak_entities(self):
        train, validation, test = project_split([("a", "a1"), ("b", "b1"), ("c", "c1")], ["b"], ["c"])
        self.assertEqual((train, validation, test), (("a1",), ("b1",), ("c1",)))
        metrics = classification_metrics(["left", "right"], ["left", "left"])
        self.assertEqual(metrics["accuracy"], 0.5)
        self.assertIn("right", metrics["per_class"])
        with self.assertRaises(ValueError):
            project_split([], ["a"], ["a"])

    def test_probability_metrics_and_inference_latency_are_reportable(self):
        self.assertEqual(brier_score([0.0, 1.0], [False, True]), 0.0)
        self.assertGreater(pr_auc([0.9, 0.1], [True, False]), 0.9)
        self.assertGreaterEqual(inference_latency_ns(lambda row: row[0], [[1.0]]), 0)
