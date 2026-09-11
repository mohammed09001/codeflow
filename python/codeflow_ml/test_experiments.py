import unittest

from .experiments import DatasetManifest, RetentionProtocol, deterministic_split, experiment_record, retention_decision


class ExperimentTests(unittest.TestCase):
    def test_manifest_lineage_and_reproducible_split(self):
        manifest = DatasetManifest.from_records("synthetic", "1", [{"id": "a"}, {"id": "b"}], ("fixture",))
        self.assertEqual(manifest.records, 2)
        self.assertEqual(manifest, DatasetManifest.from_records("synthetic", "1", [{"id": "a"}, {"id": "b"}], ("fixture",)))
        self.assertEqual(deterministic_split(["a", "b", "c"], 42), deterministic_split(["a", "b", "c"], 42))
        self.assertEqual(experiment_record(manifest, 42)["seed"], 42)

    def test_retention_requires_preregistered_gain_and_budget(self):
        protocol = RetentionProtocol("macro_f1", ("validation",), ("test",), 7, 100)
        self.assertTrue(retention_decision(protocol, {"macro_f1": .5, "pr_auc": .5}, {"macro_f1": .54, "pr_auc": .5, "latency_ns": 10})["default"])
        self.assertEqual(retention_decision(protocol, {"macro_f1": .5, "pr_auc": .5}, {"macro_f1": .51, "pr_auc": .51, "latency_ns": 10})["decision"], "optional_experimental")
