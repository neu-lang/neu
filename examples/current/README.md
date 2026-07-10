# Current Language Examples

These examples show the current language surface and the current compiler-supported subset.

- `type_checked.nl` uses the subset currently covered by M0018 type-checking helpers.
- `parsed_surface.nl` uses syntax currently accepted by the frontend parser, including forms whose full semantic checking is intentionally deferred.
- `accepted_nullability_flow.nl` shows the M0019 nullability and flow-typing surface accepted by ADR-0028. Compiler implementation for this semantic pass is pending follow-up tasks.

The implemented examples show current compiler support. The accepted semantics example records source that is now part of the language design but still pending compiler implementation.
