# Current Language Examples

These examples show the current language surface and the current compiler-supported subset.

ADR-0029 defines `const` as the immutable-local declaration keyword with the
same semantics previously associated with local `val`. The frontend now
accepts that spelling. `const` does not imply compile-time evaluation. Member
declaration syntax is separate, so `parsed_surface.neu` retains its member
`val` example.

- `type_checked.neu` uses the subset currently covered by M0018 type-checking helpers.
- `parsed_surface.neu` uses syntax currently accepted by the frontend parser, including executable-subset arithmetic and bitwise operators. The M0028 executable type-check core validates those operators as `Int`; compiler-driver integration remains later work.
- `executable_int_arithmetic.neu` shows every bootstrap `Int` arithmetic,
  exponentiation, shift, bitwise, and unary operator accepted by ADR-0042 and
  ADR-0043, including the minimum `Int` spelling.
- `static_integer_diagnostics.neu` is a negative example: its independently
  invalid literal trees produce the ADR-0043 overflow, division-by-zero,
  negative-exponent, and invalid-shift diagnostics. Runtime values are not
  statically evaluated under ADR-0048.
- `executable_entry_point.neu` shows the accepted bootstrap entry signature:
  one top-level `main` in the selected package, with no parameters, an `Int`
  return annotation, and a body. M0028 currently validates candidate selection
  and signature only; return paths and calls remain pending.
- `accepted_nullability_flow.neu` shows the M0019 nullability and flow-typing surface accepted by ADR-0028. The compiler records direct local null refinements and checks direct assignment-statement and annotated local-initializer values using valid per-use refinements. Grouped refinement propagation and end-to-end flow-pass orchestration remain pending.
- `region_exit_refinement_error.neu` shows the accepted ADR-0031 diagnostic case: a later direct use of a nullable local after its guarded refinement branch reports `region_exit_invalidated_refinement` and is still treated as nullable for recovery.
- `accepted_bootstrap_match.neu` shows the accepted ADR-0033 enum and `when` surface with the ADR-0034 typed parameter subject. The compiler validates resolved enum parameter subjects, resolves qualified variant arms, diagnoses duplicate enum and match-arm cases, and requires either every declared variant or a wildcard. Payloads, destructuring, and implicit smart casts remain deferred.
- `accepted_bootstrap_ownership.neu` shows the accepted ADR-0035 surface for copyable `Int` locals and move-only `String` local transfers.
- `use_after_move_error.neu` demonstrates a later bare-name use of a moved `String` local, which reports `use_after_move` with the move origin recorded.
- `duplicate_match_arm_error.neu` demonstrates repeated qualified and wildcard match patterns. Their second occurrences are the primary locations for duplicate-match diagnostics.
- `non_exhaustive_match_error.neu` demonstrates an otherwise valid `when` that omits a declared variant and reports `non_exhaustive_match` on the subject.
- `parsed_bootstrap_enum.neu` is accepted by the current parser and records ordered no-payload enum variants. `when` parsing, qualified-arm resolution, and finite exhaustiveness checking are available.

The implemented examples show current compiler support. The accepted semantics example also states the remaining integration limits above.
