# Soundness Report: M0019-013

## Metadata

- Task ID: `M0019-013`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-013-refinement-aware-assignment-checking.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-refinement-aware-assignment-checking.sh`
- Ordinary test results:
  - `cargo test -p newlang --test type_check m0019_refinement_aware_assignment`: pass
  - `cargo test -p newlang --test type_check`: pass
  - `sh docs/tests/m0019-refinement-aware-assignment-checking.sh`: pass

## Safety Invariants Checked

- [x] A refined assignment uses only a record attached to the exact assigned value expression.
- [x] The record's original type must equal the value's original type.
- [x] The original type must be a nullable wrapper in the active type arena.
- [x] The refined type must equal that wrapper's non-null base.
- [x] Multiple refined records for one value are rejected instead of selected by insertion order.
- [x] A refined view must link to exactly one refinement-region record with matching types.
- [x] The value must resolve exactly once to the same local binding as the refinement record.
- [x] The value expression must be a name expression inside the recorded AST branch region.
- [x] Unrefined `T? -> T` reports `InvalidNullableUse` with `NullableAssignmentWithoutRefinement`.
- [x] Successful assignment output records the effective refined type without changing original expression types.
- [x] Exact assignments and ADR-0027 nullable-target exceptions remain accepted.
- [x] `Null -> T` and unrelated `U? -> T` remain ordinary type mismatches.

## Attacks Attempted

```text
Attack: Attach a refined view whose original nullable type does not match the assigned expression's original type.
Expected result: Ignore the view and diagnose the unrefined nullable assignment.
Actual result: No assignment check is recorded and NullableAssignmentWithoutRefinement is reported.
Source of truth: ADR-0028 per-use refined output and nullable-use rules.
Outcome: pass
```

```text
Attack: Attach a refined view whose refined type is not the nullable wrapper's base.
Expected result: Ignore the inconsistent view rather than permitting assignment with an unrelated type.
Actual result: The view is ignored and the nullable-use diagnostic is emitted.
Source of truth: ADR-0028 refined output shape.
Outcome: pass
```

```text
Attack: Supply two refined views for the same assigned expression.
Expected result: Do not choose by insertion order; treat the value as unrefined.
Actual result: No assignment succeeds and NullableAssignmentWithoutRefinement is reported.
Source of truth: ADR-0028 ambiguity handling and exact per-use view requirement.
Outcome: pass
```

```text
Attack: Fabricate a type-consistent refined view for a value outside the recorded branch region.
Expected result: Reject the view and diagnose the value as an unrefined nullable assignment.
Actual result: AST region validation rejects the view and NullableAssignmentWithoutRefinement is reported.
Source of truth: ADR-0028 branch-region boundaries and per-use refined output.
Outcome: pass
```

```text
Attack: Produce real flow facts for one inside-branch use and one after-branch use of the same binding.
Expected result: Only the inside use may satisfy the non-null target.
Actual result: The inside assignment succeeds with the base type; the after-branch assignment reports InvalidNullableUse.
Source of truth: ADR-0028 branch-region and region-exit rules.
Outcome: pass
```

```text
Attack: Reclassify Null -> T or unrelated U? -> T as an M0019 nullable-use diagnostic.
Expected result: Preserve ADR-0027 TypeMismatch classification.
Actual result: Both remain TypeMismatch with their original actual types.
Source of truth: ADR-0027 assignment compatibility and ADR-0028 nullable-use rules.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test -p newlang --test type_check m0019_refinement_aware_assignment`
  - `cargo test -p newlang --test type_check`
  - `sh docs/tests/m0019-refinement-aware-assignment-checking.sh`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-013-refinement-aware-assignment-checking.md`
- Result:
  - pass

## Findings

Resolved medium finding: the initial checker trusted type-consistent `RefinedExpressionType` records without proving branch or binding provenance. Independent adversarial review found the gap before commit. The checker now requires a unique linked refinement record, exact binding resolution, and AST branch containment.

## Ambiguities

None. Local declaration initializers, grouped refined values, invalidation, and full flow orchestration remain explicit later tasks.

## Decision

Pass. Assignment statements consume only unique, provenance-validated per-use refinement views and preserve accepted diagnostic boundaries.
