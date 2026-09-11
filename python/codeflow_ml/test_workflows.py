import unittest

from .workflows import WorkflowTruth, bind_side_effects, entry_points, rank_workflows, reconstruct_workflows


class WorkflowTests(unittest.TestCase):
    def test_bounded_reconstruction_and_exception_ranking(self):
        graph = {"route": ("service",), "service": ("store", "error"), "store": (), "error": ()}
        self.assertEqual(entry_points(graph), ("route",))
        paths = reconstruct_workflows(graph, max_depth=4, max_paths=8)
        self.assertLessEqual(len(paths), 8)
        ranked = rank_workflows(paths, {"error"})
        self.assertIn("error", ranked[0])
        self.assertEqual(bind_side_effects(ranked[0], {"store": "write"})[0].entity_id, "route")
        self.assertEqual(bind_side_effects(ranked[0], {})[0].truth, WorkflowTruth.POSSIBLE)

    def test_cycles_are_bounded(self):
        paths = reconstruct_workflows({"a": ("b",), "b": ("a",)}, max_depth=3)
        self.assertTrue(paths)
        self.assertLessEqual(max(map(len, paths)), 4)
