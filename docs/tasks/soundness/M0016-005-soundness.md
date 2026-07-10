# Soundness Report: M0016-005

## Metadata

- Task ID: `M0016-005`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-005-accept-name-resolution-policy-adr.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/adr/proposals/reviews/ADR-0026-chief-architect-decision.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
  - `docs/tasks/M0016-005-accept-name-resolution-policy-adr.md`
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`
  - M0016 historical validators
- Ordinary test results:
  - M0016 accepted, concrete draft, review, proposal, and authority validators passed.

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
Attack: Accept broader resolution semantics than the reviewed draft.
Expected result: Accepted ADR-0026 keeps imports syntax-only and leaves cross-module, member, overload, extension, and type-directed lookup unsupported.
Actual result: Accepted ADR-0026 explicitly defers those lookup forms and the accepted-state validator enforces the deferrals.
Source of truth: docs/adr/ADR-0026-name-resolution-policy.md, docs/tests/m0016-name-resolution-policy-accepted.sh
Outcome: pass

Attack: Leave the ambiguity open while accepting implementation authority.
Expected result: M0016 ambiguity is resolved with ADR-0026 as the resolution source.
Actual result: Ambiguity status is resolved, cites docs/adr/ADR-0026-name-resolution-policy.md, and records implementation may define name resolution only as specified by accepted ADR-0026.
Source of truth: docs/ambiguities/M0016-name-resolution-policy.md
Outcome: pass

Attack: Permit silent misbinding through unspecified lookup order, shadowing, duplicates, or ambiguity behavior.
Expected result: Accepted ADR defines lookup order, local-before-declaration, shadowing, duplicate behavior, and ambiguity rejection.
Actual result: Accepted ADR-0026 contains those sections and diagnostics.
Source of truth: docs/adr/ADR-0026-name-resolution-policy.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`
- Tests run:
  - `docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-concrete-draft.sh && docs/tests/m0016-name-resolution-policy-review.sh && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- Imports, cross-module lookup, member lookup, overloads, extensions, and type-directed lookup remain explicitly deferred.

## Decision

Pass for ADR-0026 acceptance.
