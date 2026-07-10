# Soundness Report: M0019-005

## Metadata

- Task ID: `M0019-005`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-005-accept-nullability-flow-adr.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md`
  - `docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md`
  - `docs/tests/m0019-nullability-flow-accepted.sh`
- Ordinary test results:
  - `sh docs/tests/m0019-nullability-flow-accepted.sh`: pass
  - prior M0019 validators: pass

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
Attack: Broaden accepted M0019 behavior beyond the reviewed local immutable subset.
Expected result: Rejected.
Actual result: ADR-0028 accepts only direct null-test recognition and branch-scoped immutable local refinements; members, calls, aliases, suspension, unsafe, FFI, generics, patterns, exclusive borrows, HIR, MIR, and backend behavior remain deferred.
Source of truth: docs/adr/ADR-0028-nullability-and-flow-typing.md.
Outcome: pass
```

```text
Attack: Sneak compiler implementation into the acceptance task.
Expected result: Rejected.
Actual result: Validators confirm flow implementation identifiers are absent from `crates/newlang/src/type_check.rs`.
Source of truth: docs/tasks/M0019-005-accept-nullability-flow-adr.md.
Outcome: pass
```

```text
Attack: Leave ambiguity open after accepting the ADR.
Expected result: Rejected.
Actual result: Ambiguity report is resolved and cites accepted ADR-0028.
Source of truth: docs/ambiguities/M0019-nullability-and-flow-typing.md.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0019-nullability-flow-accepted.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-005-accept-nullability-flow-adr.md`
  - M0019 accepted, concrete, review, proposal, and blocker validators
- Result:
  - pass

## Findings

None.

## Ambiguities

- No blocking ambiguity remains for the accepted M0019 source-of-truth subset.

## Decision

Pass. Implementation may now proceed against accepted ADR-0028 only.
