import unittest
import time

from .observability import correlation_id, diagnostic_bundle, quarantine_run, timed_diagnostic


class ObservabilityTests(unittest.TestCase):
    def test_diagnostics_and_quarantine(self):
        cid = correlation_id()
        record = timed_diagnostic("info", "parser unavailable", cid, time.monotonic(), "tree-sitter")
        self.assertEqual(record.correlation_id, cid)
        self.assertIn("quarantined", quarantine_run("run-1", "fault"))
        self.assertIn("parser unavailable", diagnostic_bundle([record]))

