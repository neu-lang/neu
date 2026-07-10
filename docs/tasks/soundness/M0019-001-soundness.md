# Soundness Report: M0019-001

## Metadata

- Task ID: `M0019-001`
- Milestone: `M0019`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md`
  - `docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md`
  - `docs/tests/m0019-nullability-flow-blocked.sh`
- Ordinary test results:
  - `sh docs/tests/m0019-nullability-flow-blocked.sh`: pass

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
Attack: Implement broad Kotlin-like smart casts from high-level ADR text.
Expected result: Blocked because concrete null-test forms and eligibility rules are not accepted.
Actual result: Blocked by docs/ambiguities/M0019-nullability-and-flow-typing.md.
Source of truth: ADR-0006, ADR-0011, docs/milestones/M0019-nullability-and-flow-typing.md.
Outcome: pass
```

```text
Attack: Treat mutable local refinement invalidation as obvious and implement local-only invalidation.
Expected result: Blocked because invalidation cases for assignment, aliasing, member mutation, calls, and suspension are unspecified.
Actual result: Blocked by the ambiguity report and task forbidden changes.
Source of truth: ADR-0011, ADR-0013, ADR-0015.
Outcome: pass
```

```text
Attack: Reopen ADR-0027 nullable assignment compatibility as ambiguous.
Expected result: Rejected because ADR-0027 already accepts that bootstrap behavior.
Actual result: Ambiguity report preserves ADR-0027 nullable assignment compatibility as the temporary valid rule.
Source of truth: ADR-0027.
Outcome: pass
```

```text
Attack: Add diagnostic identifiers for invalid nullable use without source-of-truth authority.
Expected result: Blocked because ADR-0015 makes diagnostics semantic obligations.
Actual result: Report names diagnostic identifiers and recovery actions as missing authority.
Source of truth: ADR-0015.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0019-nullability-flow-blocked.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md`
  - `sh docs/tests/m0019-nullability-flow-blocked.sh`
- Result:
  - pass

## Findings

None.

## Ambiguities

- The M0019 source of truth lacks concrete null-test, nullable misuse, smart-cast eligibility, mutation invalidation, and diagnostic rules. This is filed in `docs/ambiguities/M0019-nullability-and-flow-typing.md`.

## Decision

Pass for the blocker task. M0019 implementation remains blocked pending accepted language semantics.
