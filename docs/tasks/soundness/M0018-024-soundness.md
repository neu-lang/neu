# Soundness Report: M0018-024

## Metadata

- Task ID: `M0018-024`
- Milestone: `M0018`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-024-complete-type-checking-core-milestone.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `docs/milestones/M0018-type-checking-core.md`
  - `docs/tests/m0018-type-checking-core-complete.sh`
- Ordinary test results:
  - `sh docs/tests/m0018-type-checking-core-complete.sh`: pass
  - `cargo test --workspace --all-targets`: pass, 171 tests

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
Attack: Milestone checklist is marked complete without executable evidence.
Expected result: Completion validator checks the checklist plus implementation and test evidence.
Actual result: docs/tests/m0018-type-checking-core-complete.sh validates checklist, type_m0018_core, positive fixture test, negative fixture test, and diagnostic categories.
Source of truth: M0018 milestone acceptance criteria and ADR-0027.
Outcome: pass

Attack: Closure weakens type-checking behavior.
Expected result: Closure changes only milestone metadata and validation script.
Actual result: No compiler source changes were made in this task.
Source of truth: Task forbidden changes.
Outcome: pass

Attack: Examples become stale due to a language-level change.
Expected result: Examples are skipped only when no source-language surface changes.
Actual result: This task changes milestone metadata and validator only, so examples remain correct.
Source of truth: User examples rule.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0018-type-checking-core-complete.sh`
- Tests run:
  - `sh docs/tests/m0018-type-checking-core-complete.sh`
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-024-complete-type-checking-core-milestone.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. M0018 closure is backed by executable validation and does not change compiler semantics.
