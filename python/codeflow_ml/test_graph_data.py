import unittest

from .graph_data import build_heterodata, gat_encode, gcn_encode, hetero_encode, lexical_hash, link_reconstruction_score, masked_feature_loss, neighbor_sample, node2vec_baseline, normalize_scalars


class GraphDataTests(unittest.TestCase):
    def test_round_trip_shape_and_leakage_safe_features(self):
        graph = build_heterodata([("b", "function", "checkout"), ("a", "file", "src")], [("a", "b", "calls"), ("x", "b", "bad")])
        self.assertEqual(graph.node_ids, ("a", "b"))
        self.assertEqual(len(graph.node_features), 2)
        self.assertEqual(graph.edge_index, ((0, 1),))
        self.assertEqual(len(lexical_hash("checkout")), 16)
        self.assertEqual(normalize_scalars([2, 2]), (0.0, 0.0))

    def test_representation_learning_contracts_are_deterministic(self):
        graph = build_heterodata([("a", "file", "src"), ("b", "function", "checkout")], [("a", "b", "calls")])
        self.assertEqual(node2vec_baseline(graph), node2vec_baseline(graph))
        self.assertEqual(set(gcn_encode(graph)), {"a", "b"})
        self.assertEqual(set(gat_encode(graph)), set(hetero_encode(graph)))
        self.assertGreaterEqual(link_reconstruction_score((1, 0), (1, 0)), 1)
        self.assertGreater(masked_feature_loss((1, 2), (True, False)), 0)
        self.assertEqual(neighbor_sample(graph, ["a"], 1), ("a", "b"))
