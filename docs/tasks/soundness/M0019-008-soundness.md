# Soundness Report: M0019-008

## Metadata

- Task ID: `M0019-008`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-008-null-test-recognition.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-null-test-recognition.sh`
- Ordinary test results:
  - `cargo test -p newlang --test type_check m0019_null_test`: pass
  - `cargo test -p newlang --test type_check`: pass
  - `sh docs/tests/m0019-null-test-recognition.sh`: pass

## Safety Invariants Checked

- [x] Nullability refinements are not applied by this slice.
- [x] Recognized null tests are limited to direct equality and inequality against the null literal.
- [x] Name-name, null-null, non-null literal, and non-equality comparisons create no flow fact.
- [x] Recognition does not type check, resolve names, invoke overload behavior, or infer assignment compatibility.
- [x] Diagnostics are not introduced or suppressed by this slice.

## Attacks Attempted

```text
Attack: Compare two names with `left == right`.
Expected result: No recognized null test.
Actual result: No recognized null test.
Source of truth: ADR-0028 excludes generalized boolean refinements.
Outcome: pass
```

```text
Attack: Compare `null == null`.
Expected result: No recognized null test because no local name is refined.
Actual result: No recognized null test.
Source of truth: ADR-0028 refines immutable local bindings, not literals.
Outcome: pass
```

```text
Attack: Compare `maybe < null`.
Expected result: No recognized null test because only direct equality and inequality are accepted.
Actual result: No recognized null test.
Source of truth: ADR-0028 direct null-test recognition rules.
Outcome: pass
```

```text
Attack: Compare `maybe == 1`.
Expected result: No recognized null test because the compared literal is not null.
Actual result: No recognized null test.
Source of truth: SPEC.md M0019 null-test recognition requirement.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test -p newlang --test type_check m0019_null_test`
  - `cargo test -p newlang --test type_check`
  - `sh docs/tests/m0019-null-test-recognition.sh`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-008-null-test-recognition.md`
- Result:
  - pass

## Findings

No blocking, high, medium, or low soundness findings.

## Ambiguities

None. This task does not decide eligibility, branch-region application, invalidation, or diagnostics.

## Decision

Pass. The recognizer is a passive syntactic side-table consumer and does not weaken ownership, borrowing, nullability, or diagnostic guarantees.
