# Soundness Report: M0016-001

## Metadata

- Task ID: `M0016-001`
- Milestone: `M0016`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-001-name-resolution-policy-blocker.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
  - `docs/tasks/M0016-001-name-resolution-policy-blocker.md`
  - `docs/tests/m0016-name-resolution-blocked.sh`
- Ordinary test results:
  - M0016 blocker, M0015 name table, and M0015 symbol interner validators passed.

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
Attack: Implement name resolution from M0015 name-table infrastructure alone.
Expected result: Blocked because accepted source of truth does not define lookup policy.
Actual result: Validator rejects name-resolution files and public modules; ambiguity report remains open.
Source of truth: docs/milestones/M0016-name-resolution-pass.md, docs/ambiguities/M0016-name-resolution-policy.md
Outcome: pass

Attack: Invent unresolved-name or duplicate-name diagnostics.
Expected result: Blocked until accepted diagnostics are defined.
Actual result: Ambiguity report lists diagnostics as blocked and no implementation files were added.
Source of truth: docs/ambiguities/M0016-name-resolution-policy.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0016-name-resolution-blocked.sh`
- Tests run:
  - `docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0015-symbol-interner.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- M0016 name resolution policy is ambiguous and recorded in `docs/ambiguities/M0016-name-resolution-policy.md`.

## Decision

Pass for blocker task.
