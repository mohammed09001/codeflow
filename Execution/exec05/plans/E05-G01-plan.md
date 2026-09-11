# Goal

E05-G01: real, distinct structural clustering algorithms.

# Current Failure

The UPSM exposed weakly connected components as Leiden, aliased agglomerative clustering to it, and used a degree threshold under the HDBSCAN name.

# Root Cause Hypothesis

The original phase implemented deterministic placeholders without preserving algorithm semantics in either names or behavior.

# Plan Revision

1. Retain weak connectivity only under `connected_components_baseline`.
2. Add deterministic modularity moving plus connectivity refinement for community detection.
3. Add average-linkage agglomeration and mutual-reachability density hierarchy with explicit noise.
4. Prove weak-bridge divergence, deterministic serialization, and noise behavior through production tests.
