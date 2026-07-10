# Soundness Report: M0019-002

## Metadata

- Task ID: `M0019-002`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-002-nullability-flow-proposal.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/tasks/M0019-002-nullability-flow-proposal.md`
  - `docs/tests/m0019-nullability-flow-proposal.sh`
- Ordinary test results:
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
Attack: Treat the draft proposal as accepted implementation authority.
Expected result: Rejected by non-authority notice and validator checks.
Actual result: Proposal says no implementation may depend on it until accepted, accepted ADR-0028 remains absent, and docs/SPEC.md is unchanged.
Source of truth: main task rules source-of-truth rules, docs/ambiguities/M0019-nullability-and-flow-typing.md.
Outcome: pass
```

```text
Attack: Preserve refinements across mutation, calls, or coroutine suspension.
Expected result: Rejected or deferred because accepted alias, effect, and suspension rules are missing.
Actual result: Proposal defers calls, member mutation, alias analysis, unsafe/FFI boundaries, and coroutine suspension effects.
Source of truth: ADR-0011, ADR-0013, ADR-0015.
Outcome: pass
```

```text
Attack: Use Kotlin-like property stability or platform-null behavior as implicit authority.
Expected result: Rejected.
Actual result: Proposal states it does not rely on Kotlin, Rust, current parser behavior, current test behavior, or current type_check behavior as implicit authority and defers platform nullability.
Source of truth: ADR-0006, ADR-0011, ADR-0018.
Outcome: pass
```

```text
Attack: Turn nullable misuse diagnostics into unsafe unwrap suggestions.
Expected result: Rejected by diagnostic safe suggestion policy.
Actual result: Proposal requires diagnostics to report expected and actual types without suggesting force unwraps, unsafe operations, API redesigns, or ownership changes.
Source of truth: ADR-0015.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0019-nullability-flow-proposal.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-002-nullability-flow-proposal.md`
  - `sh docs/tests/m0019-nullability-flow-proposal.sh`
  - `sh docs/tests/m0019-nullability-flow-blocked.sh`
- Result:
  - pass

## Findings

None.

## Ambiguities

- M0019 remains blocked until ADR-0028 is reviewed, accepted, and incorporated into source of truth.

## Decision

Pass for the proposal-only task. No compiler implementation may proceed from this proposal until accepted.
