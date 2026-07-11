# ADR-0057 Adversarial Review

Decision: approve. Rejecting absolute paths, traversal, target mismatches, and
host `PATH` fallback prevents an apparently cross-compiled executable from
silently using unrelated host tools or startup objects.
