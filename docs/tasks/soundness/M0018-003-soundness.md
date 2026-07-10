# Soundness Report: M0018-003

## Metadata

- Task ID: `M0018-003`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-003-type-checking-core-proposal-review.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/adr/proposals/reviews/ADR-0027-*.md`
  - `docs/tests/m0018-type-checking-core-review.sh`
- Ordinary test results:
  - `sh docs/tests/m0018-type-checking-core-review.sh`: pass

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
Attack: Treat review artifacts as implementation authority.
Expected result: Reviews state they are not source of truth and accepted ADR-0027 remains absent.
Actual result: Validator checks accepted ADR absence and non-authority boundaries.
Source of truth: main task rules and docs/ambiguities/M0018-type-checking-core.md.
Outcome: pass
```

```text
Attack: Use review task to resolve the ambiguity.
Expected result: M0018 ambiguity remains open.
Actual result: Validator requires `Status: open`.
Source of truth: docs/ambiguities/M0018-type-checking-core.md.
Outcome: pass
```

```text
Attack: Sneak type checking behavior into a review-only task.
Expected result: Type checker remains diagnostic-only.
Actual result: Validator rejects check_expression, check_declaration, infer_type, literal_type, resolve_call, check_assignment, TypedExpression, TypedProgram, and WellTyped patterns.
Source of truth: task forbidden changes.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0018-type-checking-core-review.sh`
- Tests run:
  - `sh docs/tests/m0018-type-checking-core-review.sh`
- Result:
  - pass

## Findings

No soundness findings.

## Ambiguities

- M0018 remains blocked pending ADR-0027 revision and acceptance.

## Decision

Pass. Review artifacts preserve the source-of-truth boundary and do not weaken safety semantics.
