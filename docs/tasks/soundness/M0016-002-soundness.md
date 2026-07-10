# Soundness Report: M0016-002

## Metadata

- Task ID: `M0016-002`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-002-name-resolution-policy-proposal.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/tasks/M0016-002-name-resolution-policy-proposal.md`
  - `docs/tests/m0016-name-resolution-policy-proposal.sh`
- Ordinary test results:
  - M0016 proposal and blocker validators passed.

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
Attack: Treat the ADR-0026 proposal as accepted source of truth.
Expected result: Proposal clearly marks itself non-authoritative and implementation remains blocked.
Actual result: Proposal has a non-authority notice, accepted ADR-0026 is absent, SPEC is unchanged, and validator rejects name-resolution files.
Source of truth: docs/adr/proposals/ADR-0026-name-resolution-policy.md, docs/ambiguities/M0016-name-resolution-policy.md
Outcome: pass

Attack: Omit diagnostic obligations needed for safe implementation review.
Expected result: Proposal must name unresolved, duplicate, ambiguous, and inaccessible diagnostics for accepted version review.
Actual result: Proposal and validator include unresolved_name, duplicate_name, ambiguous_name, and inaccessible_name.
Source of truth: docs/tests/m0016-name-resolution-policy-proposal.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0016-name-resolution-policy-proposal.sh`
- Tests run:
  - `docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- M0016 remains blocked pending accepted name-resolution policy.

## Decision

Pass for non-authoritative proposal task.
