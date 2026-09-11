import unittest

from .graph_data import build_heterodata, lexical_hash, normalize_scalars


class GraphDataTests(unittest.TestCase):
    def test_round_trip_shape_and_leakage_safe_features(self):
        graph = build_heterodata([("b", "function", "checkout"), ("a", "file", "src")], [("a", "b", "calls"), ("x", "b", "bad")])
        self.assertEqual(graph.node_ids, ("a", "b"))
        self.assertEqual(len(graph.node_features), 2)
        self.assertEqual(graph.edge_index, ((0, 1),))
        self.assertEqual(len(lexical_hash("checkout")), 16)
        self.assertEqual(normalize_scalars([2, 2]), (0.0, 0.0))

