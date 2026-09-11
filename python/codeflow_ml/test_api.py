import unittest

from .api import REQUIRED_VIEW_TYPES, dispatch, paginate, revision_cursor, revision_envelope, validate_cursor


class ApiTests(unittest.TestCase):
    def test_versioned_pagination_and_dispatch(self):
        self.assertEqual(paginate([{"id": "a"}, {"id": "b"}], 1)["next_cursor"], 1)
        self.assertIn("unsupported_api_version", dispatch({"api_version": "v9", "operation": "overview"}, {}))
        self.assertIn('"result": []', dispatch({"operation": "overview"}, {"overview": []}))

    def test_revision_safe_contract_and_cursor(self):
        response = revision_envelope("project", "revision", {"nodes": []})
        self.assertEqual(response["revision_id"], "revision")
        self.assertEqual(validate_cursor(revision_cursor("project", "revision", 3), "project", "revision"), 3)
        with self.assertRaises(ValueError):
            validate_cursor(revision_cursor("project", "old", 3), "project", "revision")
        self.assertIn("WorkflowView", REQUIRED_VIEW_TYPES)
