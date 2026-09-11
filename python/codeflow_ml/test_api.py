import unittest

from .api import dispatch, paginate


class ApiTests(unittest.TestCase):
    def test_versioned_pagination_and_dispatch(self):
        self.assertEqual(paginate([{"id": "a"}, {"id": "b"}], 1)["next_cursor"], 1)
        self.assertIn("unsupported_api_version", dispatch({"api_version": "v9", "operation": "overview"}, {}))
        self.assertIn('"result": []', dispatch({"operation": "overview"}, {"overview": []}))

