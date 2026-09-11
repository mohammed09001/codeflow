# Goal

E05-G00: parse-aware framework semantics.

# Current Failure

`DeterministicWebAdapter` promoted raw marker matches to deterministic route facts.

# Root Cause Hypothesis

The adapter had no parsed-node or source-span input, so it could not distinguish an invocation or annotation from a comment or string literal.

# Plan Revision

1. Parse each supported input with its Tree-sitter grammar and inspect only route-bearing AST node kinds.
2. Attach method, route, source span, provider, fact class, confidence, and a stable evidence ID to extracted facts.
3. Exercise positive routes and deceptive comments, literals, and unrelated calls through the production adapter.
