# Soundness Report: M0019-003

## Metadata

- Task ID: `M0019-003`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-003-nullability-flow-proposal-review.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/adr/proposals/reviews/ADR-0028-*.md`
  - `docs/tasks/M0019-003-nullability-flow-proposal-review.md`
  - `docs/tests/m0019-nullability-flow-review.sh`
- Ordinary test results:
  - `sh docs/tests/m0019-nullability-flow-review.sh`: pass
  - `sh docs/tests/m0019-nullability-flow-proposal.sh`: pass
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
Attack: Use the review task to silently accept ADR-0028.
Expected result: Rejected; review-only task must keep implementation blocked.
Actual result: main task decision requests revision before acceptance and says implementation may not proceed against ADR-0028 until accepted.
Source of truth: docs/tasks/M0019-003-nullability-flow-proposal-review.md.
Outcome: pass
```

```text
Attack: Preserve refinements beyond their sound branch region.
Expected result: Rejected before acceptance.
Actual result: main-task language review, Adversarial, and main task reviews require concrete branch region boundaries before acceptance.
Source of truth: ADR-0011, ADR-0013.
Outcome: pass
```

```text
Attack: Hide nullable safety failures behind vague diagnostics.
Expected result: Rejected by diagnostics review.
Actual result: Diagnostics review requires primary spans, recovery actions, source-of-truth citations, safe suggestion policies, and stable rule identifiers.
Source of truth: ADR-0015.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0019-nullability-flow-review.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-003-nullability-flow-proposal-review.md`
  - `sh docs/tests/m0019-nullability-flow-review.sh`
  - `sh docs/tests/m0019-nullability-flow-proposal.sh`
  - `sh docs/tests/m0019-nullability-flow-blocked.sh`
- Result:
  - pass

## Findings

None.

## Ambiguities

- M0019 remains blocked until ADR-0028 is revised, accepted, and incorporated into source of truth.

## Decision

Pass for the review-only task. No compiler implementation may proceed from ADR-0028 until accepted.
