import unittest

from .baselines import FeatureCandidate, FeatureMatrix, LogisticBaseline, NaiveBayes, adaptive_weights, classify_shared_name, clustering_ensemble, domain_specificity, feature_participation, infrastructure_centrality, merge_feature_candidates, seed_feature, signal_quality, tree_rank


class BaselineTests(unittest.TestCase):
    def test_reproducible_baselines(self):
        matrix = FeatureMatrix.from_rows([("a", [1, 0]), ("b", [0, 1])])
        self.assertEqual(NaiveBayes().fit(matrix, ["x", "y"]).predict([1, 0]), "x")
        model = LogisticBaseline().fit(matrix, [1, 0], epochs=20)
        self.assertGreater(model.predict_proba([1, 0]), model.predict_proba([0, 1]))
        self.assertEqual(tree_rank([[1, 0], [0, 1]], [1, 0]), [0, 1])
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
