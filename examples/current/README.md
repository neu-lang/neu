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

The implemented examples show current compiler support. The accepted semantics example also states the remaining integration limits above.
