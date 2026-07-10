# Soundness Report: M0019-009

## Metadata

- Task ID: `M0019-009`
- Milestone: `M0019`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-009-null-test-eligibility.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-null-test-eligibility.sh`
- Ordinary test results:
  - `cargo test -p newlang --test type_check m0019_null_test_eligibility`: pass
  - `cargo test -p newlang --test type_check`: pass
  - `sh docs/tests/m0019-null-test-eligibility.sh`: pass

## Safety Invariants Checked

- [x] Mutable local bindings do not become eligible for null-test refinement.
- [x] Non-nullable bindings do not produce redundant or invented flow facts.
- [x] Missing resolution or missing declaration type information creates no flow fact.
- [x] Ambiguous local binding matches are rejected with an ambiguity diagnostic.
- [x] Eligibility does not apply branch-region refinements or per-use refined expression types.
- [x] Eligibility does not infer types from source text or untyped expressions.

## Attacks Attempted

```text
Attack: Feed a recognized null test whose name resolves to a `var`.
Expected result: No eligible refinement and `MutableLocalRefinementDeferred`.
Actual result: No eligible refinement and unsupported-flow diagnostic on the name expression.
Source of truth: ADR-0028 rejects mutable binding refinements.
Outcome: pass
```

```text
Attack: Feed a recognized null test whose local binding has non-nullable type `T`.
Expected result: No eligible refinement.
Actual result: No eligible refinement and no diagnostic.
Source of truth: ADR-0028 refines `T?` to `T`; already non-null bindings are not nullable refinements.
Outcome: pass
```

```text
Attack: Feed unresolved and missing-signature recognized null tests.
Expected result: No flow fact is created.
Actual result: No eligible refinement and no diagnostic.
Source of truth: ADR-0028 states missing, unresolved, or unsupported type information creates no flow fact.
Outcome: pass
```

```text
Attack: Feed two local bindings with the same resolved symbol.
Expected result: No eligible refinement and `AmbiguousLocalBindingFlow`.
Actual result: No eligible refinement and ambiguous-flow diagnostic on the name expression.
Source of truth: SPEC.md M0019 flow diagnostics and ADR-0028 ambiguity handling.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test -p newlang --test type_check m0019_null_test_eligibility`
  - `cargo test -p newlang --test type_check`
  - `sh docs/tests/m0019-null-test-eligibility.sh`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-009-null-test-eligibility.md`
- Result:
  - pass

## Findings

No blocking, high, medium, or low soundness findings.

## Ambiguities

None. Branch-region application, invalidation, and nullable-use diagnostics remain later tasks.

## Decision

Pass. The selector narrows recognized null tests to immutable nullable locals without creating active smart casts or weakening nullability safety.
