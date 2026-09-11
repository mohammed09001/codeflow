import unittest

from .security import command_allowed, network_offline, redact_secrets, safe_relative_path


class SecurityTests(unittest.TestCase):
    def test_boundaries_and_redaction(self):
        self.assertEqual(safe_relative_path(".", "src/a.rs").name, "a.rs")
        with self.assertRaises(ValueError):
            safe_relative_path(".", "../../secret")
        self.assertFalse(command_allowed("setup.py"))
        self.assertIn("[REDACTED]", redact_secrets("token=abc password=xyz"))
        self.assertTrue(network_offline())

