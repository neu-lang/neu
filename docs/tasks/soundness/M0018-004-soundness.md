# Soundness Report: M0018-004

## Metadata

- Task ID: `M0018-004`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-004-type-checking-core-concrete-draft.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- Proposal: `docs/adr/proposals/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md`
  - `docs/tests/m0018-type-checking-core-concrete-draft.sh`
- Ordinary test results:
  - `sh docs/tests/m0018-type-checking-core-concrete-draft.sh`: pass

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
Attack: Treat the concrete draft as accepted semantics.
Expected result: ADR-0027 remains a draft and accepted ADR-0027 is absent.
Actual result: Validator requires draft status and absence of docs/adr/ADR-0027-type-checking-core.md.
Source of truth: docs/ambiguities/M0018-type-checking-core.md.
Outcome: pass
```

```text
Attack: Let primitive type identities leak into ABI or layout.
Expected result: Draft states primitive identities have no ABI or layout meaning.
Actual result: Validator requires both "type-checking identity only" and "no ABI or layout meaning".
Source of truth: ADR-0027 review findings.
Outcome: pass
```

```text
Attack: Accidentally authorize calls, function application, overloads, conversions, ownership, HIR, MIR, or backend behavior.
Expected result: Draft explicitly defers those areas.
Actual result: Validator requires direct call and function application deferrals plus broader deferral list.
Source of truth: ADR-0027 review findings and M0018 out-of-scope list.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0018-type-checking-core-concrete-draft.sh`
- Tests run:
  - `sh docs/tests/m0018-type-checking-core-concrete-draft.sh`
- Result:
  - pass

## Findings

No soundness findings.

## Ambiguities

- M0018 remains blocked pending accepted ADR-0027.

## Decision

Pass. The concrete draft narrows future implementation without authorizing it.
