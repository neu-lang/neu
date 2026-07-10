# Current Language Examples

These examples show the current language surface and the current compiler-supported subset.

ADR-0029 defines `const` as the immutable-local declaration keyword with the
same semantics previously associated with local `val`. The frontend now
accepts that spelling. `const` does not imply compile-time evaluation. Member
declaration syntax is separate, so `parsed_surface.nl` retains its member
`val` example.

- `type_checked.nl` uses the subset currently covered by M0018 type-checking helpers.
- `parsed_surface.nl` uses syntax currently accepted by the frontend parser, including forms whose full semantic checking is intentionally deferred.
- `accepted_nullability_flow.nl` shows the M0019 nullability and flow-typing surface accepted by ADR-0028. The compiler records direct local null refinements and checks direct assignment-statement and annotated local-initializer values using valid per-use refinements. Grouped refinement propagation and end-to-end flow-pass orchestration remain pending.
- `region_exit_refinement_error.nl` shows the accepted ADR-0031 diagnostic case: a later direct use of a nullable local after its guarded refinement branch reports `region_exit_invalidated_refinement` and is still treated as nullable for recovery.
- `accepted_bootstrap_match.nl` shows the ADR-0033 accepted no-payload enum and exhaustive `when` surface. Parser and semantic implementation for this newly accepted subset begin with M0021; payloads, destructuring, and implicit smart casts remain deferred.
- `parsed_bootstrap_enum.nl` is accepted by the current parser and records ordered no-payload enum variants. `when` parsing, variant resolution, and exhaustiveness checking remain pending M0021 work.

The implemented examples show current compiler support. The accepted semantics example also states the remaining integration limits above.
