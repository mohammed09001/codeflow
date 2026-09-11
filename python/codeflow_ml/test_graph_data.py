import unittest

from .graph_data import TrainableGat, TrainableGcn, TrainableGraphEncoder, TrainableHeteroGnn, build_heterodata, legacy_attention_scale, legacy_degree_embedding, legacy_neighbor_average, legacy_type_offset, lexical_hash, link_reconstruction_score, masked_feature_loss, neighbor_sample, normalize_scalars


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
        self.assertEqual(legacy_degree_embedding(graph), legacy_degree_embedding(graph))
        self.assertEqual(set(legacy_neighbor_average(graph)), {"a", "b"})
        self.assertEqual(set(legacy_attention_scale(graph)), set(legacy_type_offset(graph)))
        self.assertGreaterEqual(link_reconstruction_score((1, 0), (1, 0)), 1)
        self.assertGreater(masked_feature_loss((1, 2), (True, False)), 0)
        self.assertEqual(neighbor_sample(graph, ["a"], 1), ("a", "b"))

    def test_trainable_encoder_reduces_loss_and_changes_parameters(self):
        graph = build_heterodata([("a", "file", "src"), ("b", "function", "checkout")], [("a", "b", "calls")])
        model = TrainableGraphEncoder.create(graph, seed=7)
        before = model.parameters()
        loss_before = model.train_epoch(graph, rate=0.2)
        for _ in range(20):
            loss_after = model.train_epoch(graph, rate=0.2)
        self.assertLess(loss_after, loss_before)
        self.assertNotEqual(before, model.parameters())
        self.assertEqual(set(model.encode(graph)), {"a", "b"})

    def test_trainable_message_passing_variants_update_parameters(self):
        graph = build_heterodata([("a", "file", "src"), ("b", "function", "checkout")], [("a", "b", "calls")])
        for model_type in (TrainableGcn, TrainableGat, TrainableHeteroGnn):
            model = model_type.create(graph, seed=3)
            before = model.parameters()
            model.train_epoch(graph)
            self.assertNotEqual(before, model.parameters())
