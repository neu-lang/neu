# Soundness Report: M0019-012

## Metadata

- Task ID: `M0019-012`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-012-refined-expression-type-records.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0019-refined-expression-type-records.sh`
- Ordinary test results:
  - `cargo test -p compiler --test type_check m0019_refined_expression_type_records`: pass
  - `cargo test -p compiler --test type_check`: pass
  - `sh docs/tests/m0019-refined-expression-type-records.sh`: pass

## Safety Invariants Checked

- [x] Refined expression types are recorded only for exact matching local binding identities.
- [x] The null-test condition itself is outside the branch refinement region.
- [x] Direct uses inside the branch receive per-use non-null views.
- [x] Nested blocks inside the branch inherit the active refinement.
- [x] Same-name uses resolving to a nested shadowing binding do not inherit the outer refinement.
- [x] Uses after the branch do not retain the refinement.
- [x] A use matching multiple active regions receives no refined type and reports `ambiguous_null_test_region`.
- [x] Forged non-name resolution records and cross-source-file spans cannot receive refinements.
- [x] Original expression types and declaration signatures are not modified.

## Attacks Attempted

```text
Attack: Match all uses by source name while a nested same-name binding shadows the refined binding.
Expected result: Only uses resolving to the exact original LocalBinding receive the refined type.
Actual result: Outer-binding uses are refined; the shadowing-binding use is not.
Source of truth: ADR-0028 Shadowing And Nested Scope Rules.
Outcome: pass
```

```text
Attack: Carry the refinement into the condition or a use after the if expression.
Expected result: Neither use receives a refined expression type.
Actual result: AST block containment excludes both uses.
Source of truth: ADR-0028 Branch Region Boundaries.
Outcome: pass
```

```text
Attack: Place one exact binding use inside two applicable nested refinement regions.
Expected result: No type is guessed; report AmbiguousFlowRule with AmbiguousNullTestRegion.
Actual result: No refined type is recorded and the stable ambiguity diagnostic is emitted at the use.
Source of truth: ADR-0028 ambiguous flow diagnostics.
Outcome: pass
```

```text
Attack: Forge resolved-local metadata for a literal node or for a same-offset name in another source file.
Expected result: Neither record is treated as a name use inside the branch.
Actual result: Both are ignored without refined output.
Source of truth: ADR-0028 refined expression entries and source-region boundaries.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/type_check.rs`
- Tests run:
  - `cargo test -p compiler --test type_check m0019_refined_expression_type_records`
  - `cargo test -p compiler --test type_check`
  - `sh docs/tests/m0019-refined-expression-type-records.sh`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-012-refined-expression-type-records.md`
- Result:
  - pass

## Findings

No blocking, high, medium, or low soundness findings.

## Ambiguities

None. ADR-0028 defines exact binding identity, branch containment, nested inheritance, shadowing exclusion, and ambiguity diagnostics for this slice.

## Decision

Pass. Per-use refinement records are limited to unambiguous exact-binding uses inside active branch regions.
