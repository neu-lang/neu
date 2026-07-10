# Soundness Report: M0019-004

## Metadata

- Task ID: `M0019-004`
- Milestone: `M0019`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-004-nullability-flow-concrete-draft.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/tasks/M0019-004-nullability-flow-concrete-draft.md`
  - `docs/tests/m0019-nullability-flow-concrete-draft.sh`
- Ordinary test results:
  - `sh docs/tests/m0019-nullability-flow-concrete-draft.sh`: pass
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
Attack: Let a refinement escape beyond the branch that proved it.
Expected result: Blocked by concrete branch region boundaries.
Actual result: ADR-0028 draft limits refinement to the then or else branch block and ends it at the closing brace.
Source of truth: ADR-0028 draft, non-authoritative.
Outcome: pass
```

```text
Attack: Treat a refined binding declaration as permanently non-null.
Expected result: Blocked by side-table refined expression entries only.
Actual result: ADR-0028 draft preserves the original nullable type of the binding and makes refined types per-use views.
Source of truth: ADR-0028 draft, non-authoritative.
Outcome: pass
```

```text
Attack: Use general binary expression typing or overloaded equality to create flow facts.
Expected result: Blocked by a flow-specific recognizer.
Actual result: ADR-0028 draft says null-test recognition does not require general binary expression type checking or overload resolution.
Source of truth: ADR-0028 draft, non-authoritative.
Outcome: pass
```

```text
Attack: Hide ambiguous local binding flow facts.
Expected result: Rejected by ambiguous_flow_rule.
Actual result: ADR-0028 draft requires no refinement and an ambiguous_flow_rule diagnostic when a condition references an ambiguous local binding.
Source of truth: ADR-0028 draft, non-authoritative.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0019-nullability-flow-concrete-draft.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-004-nullability-flow-concrete-draft.md`
  - `sh docs/tests/m0019-nullability-flow-concrete-draft.sh`
  - `sh docs/tests/m0019-nullability-flow-review.sh`
  - `sh docs/tests/m0019-nullability-flow-proposal.sh`
  - `sh docs/tests/m0019-nullability-flow-blocked.sh`
- Result:
  - pass

## Findings

None.

## Ambiguities

- M0019 remains blocked until ADR-0028 is accepted and incorporated into source of truth.

## Decision

Pass for the concrete-draft task. No compiler implementation may proceed from ADR-0028 until accepted.
