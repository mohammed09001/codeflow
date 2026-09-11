import json
import tempfile
import unittest
from pathlib import Path

from scripts.codeflow import main


class CliTests(unittest.TestCase):
    def test_init_creates_project_metadata(self):
        with tempfile.TemporaryDirectory() as directory:
            self.assertEqual(main(["init", directory]), 0)
            self.assertTrue(Path(directory, ".codeflow", "project.json").is_file())

