import unittest

from .experiments import DatasetManifest, deterministic_split, experiment_record


class ExperimentTests(unittest.TestCase):
    def test_manifest_lineage_and_reproducible_split(self):
        manifest = DatasetManifest.from_records("synthetic", "1", [{"id": "a"}, {"id": "b"}], ("fixture",))
        self.assertEqual(manifest.records, 2)
        self.assertEqual(manifest, DatasetManifest.from_records("synthetic", "1", [{"id": "a"}, {"id": "b"}], ("fixture",)))
        self.assertEqual(deterministic_split(["a", "b", "c"], 42), deterministic_split(["a", "b", "c"], 42))
        self.assertEqual(experiment_record(manifest, 42)["seed"], 42)

