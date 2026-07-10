# Soundness Report: M0019-006

## Metadata

- Task ID: `M0019-006`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-006-flow-output-data-model.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-flow-output-data-model.sh`
- Ordinary test results:
  - `cargo test -p newlang --test type_check`: pass
  - `sh docs/tests/m0019-flow-output-data-model.sh`: pass

## Safety Invariants Checked

- [x] Ownership cannot be bypassed.
- [x] Moved values cannot be reused.
- [x] Shared and exclusive borrows cannot conflict.
- [x] Borrowed data cannot outlive its owner.
- [x] Nullability refinements cannot be used after invalidation.
- [x] Thread send/share capabilities are enforced.
- [x] Coroutine scopes cannot outlive allowed ownership or borrow lifetimes.
- [x] Borrows across suspension are rejected unless proven safe by accepted semantics.
- [x] Unsafe and FFI boundaries do not weaken safe-code guarantees.
- [x] Diagnostics do not hide or misstate safety failures.

## Attacks Attempted

```text
Attack: Treat a refinement record as rewriting the binding declaration type.
Expected result: Rejected.
Actual result: RefinementRecord and RefinedExpressionType store original nullable and refined non-null types separately; declaration signatures are unchanged.
Source of truth: ADR-0028 Refined Output Shape.
Outcome: pass
```

```text
Attack: Sneak in branch traversal or smart-cast behavior.
Expected result: Rejected.
Actual result: Docs validator rejects recognize_null_test, apply_smart_cast, check_nullable_use, and walk_if_branch symbols; no such behavior was added.
Source of truth: M0019-006 scope.
Outcome: pass
```

```text
Attack: Hide nullable errors behind existing type diagnostics only.
Expected result: Rejected.
Actual result: TypeCheckDiagnosticKind now includes invalid nullable use, invalidated refinement, unsupported flow rule, and ambiguous flow rule.
Source of truth: ADR-0028 Flow Diagnostics.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0019-flow-output-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-006-flow-output-data-model.md`
  - `cargo test -p newlang --test type_check`
  - `sh docs/tests/m0019-flow-output-data-model.sh`
- Result:
  - pass

## Findings

None.

## Ambiguities

- Branch recognition and branch-region modeling remain future tasks.

## Decision

Pass. The task adds passive output structures and does not alter safety behavior.
