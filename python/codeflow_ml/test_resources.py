import unittest

from .resources import ResourceBudget, bounded_batch, progress_events, select_device, size_tier


class ResourceTests(unittest.TestCase):
    def test_tiers_batches_device_and_cancel(self):
        self.assertEqual(size_tier(10000), "large")
        self.assertEqual(len(bounded_batch(list(range(5)), ResourceBudget(max_items=2))), 3)
        self.assertIn(select_device(), ("cpu", "cuda"))
        self.assertTrue(list(progress_events(10, 2))[-1]["cancelled"])

