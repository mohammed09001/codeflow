import unittest

from .query import dependency_path, evidence_view, expand_node, overview, revision_diff, reverse_impact


class QueryTests(unittest.TestCase):
    def test_bounded_views_and_paths(self):
        nodes = [{"id": "b"}, {"id": "a"}]
        edges = [{"source": "a", "target": "b"}, {"source": "b", "target": "c"}]
        self.assertEqual([node["id"] for node in overview(nodes, edges)["nodes"]], ["a", "b"])
        self.assertEqual(expand_node("a", edges, 1), ("a", "b"))
        self.assertEqual(dependency_path("a", "c", edges), ("a", "b", "c"))
        self.assertEqual(reverse_impact("b", edges), ("a",))
        self.assertEqual(revision_diff({"a"}, {"b"})["added"], ["b"])
        self.assertEqual(evidence_view([{"source_id": "x", "evidence_id": "e"}], "x"), '[{"evidence_id":"e","source_id":"x"}]')
