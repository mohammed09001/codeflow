import unittest

from .zoom import ZoomNode, bundle_edges, materialize_zoom, stable_parent


class ZoomTests(unittest.TestCase):
    def test_bounded_stable_zoom_and_bundling(self):
        nodes = [ZoomNode("b", "feature", weight=1), ZoomNode("a", "feature", weight=1), ZoomNode("x", "file")]
        self.assertEqual(tuple(n.node_id for n in materialize_zoom(nodes, "feature", 1)), ("a",))
        self.assertEqual(bundle_edges([("a", "b", 1), ("a", "b", 2)])[("a", "b")], 3)
        self.assertEqual(stable_parent({"child": ["z", "a"], "orphan": []})["child"], "a")

