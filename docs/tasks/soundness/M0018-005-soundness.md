# Soundness Report: M0018-005

## Metadata

- Task ID: `M0018-005`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-005-accept-type-checking-core-adr.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- Accepted ADR: `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0018-type-checking-core.md`
  - `docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md`
- Ordinary test results:
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`: pass

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
Attack: Broaden accepted M0018 semantics beyond reviewed draft.
Expected result: Direct calls, function type application, overloads, numeric conversion, generic solving, ownership, borrow checking, HIR, MIR, and backend behavior remain deferred.
Actual result: Accepted-state validator requires these deferrals and checks implementation remains diagnostic-only.
Source of truth: docs/adr/ADR-0027-type-checking-core.md.
Outcome: pass
```

```text
Attack: Let primitive identities imply ABI or layout.
Expected result: ADR-0027 states primitive identities are type-checking only with no ABI or layout meaning.
Actual result: Accepted-state validator requires both phrases.
Source of truth: docs/adr/ADR-0027-type-checking-core.md.
Outcome: pass
```

```text
Attack: Resolve ambiguity without updating source of truth.
Expected result: Accepted ADR and SPEC summary exist before ambiguity is resolved.
Actual result: Accepted-state validator requires all three.
Source of truth: docs/adr/ADR-0027-type-checking-core.md and docs/SPEC.md.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0018-type-checking-core-accepted.sh`
- Tests run:
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`
- Result:
  - pass

## Findings

No soundness findings.

## Ambiguities

- M0018 source-of-truth ambiguity is resolved by accepted ADR-0027.

## Decision

Pass. Acceptance unblocks implementation without adding behavior or weakening safety boundaries.
