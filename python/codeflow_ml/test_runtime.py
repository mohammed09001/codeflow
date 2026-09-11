import unittest

from .runtime import align_static_dynamic, correlate_events, compress_repeated, ingest_trace, normalize_sessions, path_frequencies, redact_trace


class RuntimeTests(unittest.TestCase):
    def test_ingest_correlation_and_privacy(self):
        payload = '[{"timestamp_ns": 1, "kind": "call", "entity_id": "fn:a", "target_id": "fn:b", "state": "secret"}]'
        events = ingest_trace(payload)
        self.assertEqual(len(correlate_events(events, {"fn:b"})), 1)
        self.assertIsNone(redact_trace(events, {"state"})[0].state)

    def test_malformed_trace_rejected(self):
        with self.assertRaises(ValueError):
            ingest_trace('{"bad": true}')

    def test_trace_abstraction_and_alignment(self):
        events = ingest_trace('[{"timestamp_ns": 2, "kind": "call", "entity_id": "a", "evidence_id": "s"}, {"timestamp_ns": 1, "kind": "call", "entity_id": "a", "evidence_id": "s"}]')
        sessions = normalize_sessions(events)
        self.assertEqual(compress_repeated(("a", "a")), ("a*2",))
        frequencies = path_frequencies(sessions)
        self.assertEqual(next(iter(frequencies.values())), 1)
        self.assertIn(("a", "b"), align_static_dynamic((("a", "b"),), {("a",): 1}))
