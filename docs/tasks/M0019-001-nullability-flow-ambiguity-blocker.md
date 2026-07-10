# Task: M0019-001 Block Ambiguous Nullability And Flow Typing Rules

## Task Metadata

- Task ID: `M0019-001`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner Agent: `Language Lawyer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0019-001-nullability-flow-ambiguity-blocker`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Record and block ambiguous nullability and flow-sensitive smart-cast rules before implementing M0019 checks.

## Motivation

M0019 requires nullability checks and mutation-invalidation behavior, but the accepted source of truth does not yet define the concrete null-test forms, refinement facts, assignment interactions, or diagnostics needed for implementation.

## Scope

- File an ambiguity report for M0019 nullability and flow typing.
- Identify missing concrete rules for nullable misuse, smart-cast eligibility, refinement invalidation, and diagnostics.
- Add a validator proving the ambiguity report exists and blocks implementation.
- Record that implementation must not proceed until the source of truth is updated.

## Out Of Scope

- Implementing nullability checks.
- Implementing flow refinement tracking.
- Implementing smart casts.
- Implementing mutation invalidation.
- Modifying `docs/SPEC.md` or `docs/adr/`.
- Adding source-language examples for unimplemented behavior.

## Required Inputs

- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification sections:
  - `ADR-0006: Nullability And Absence`
  - `ADR-0011: Flow Typing And Smart Casts`
  - `ADR-0027: Type Checking Core`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0027-type-checking-core.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Validator confirms an open M0019 ambiguity report exists.
  - Validator confirms implementation is blocked until source-of-truth rules are accepted.
- Negative tests:
  - Validator fails before the ambiguity report exists.
- Diagnostic tests:
  - Validator confirms the report names nullable misuse, smart-cast eligibility, mutation invalidation, and diagnostic rules.
- Adversarial tests:
  - Confirm this task does not implement nullability checking, flow tracking, smart casts, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0019-nullability-flow-blocked.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md` does not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create a documentation-only ambiguity report and no compiler behavior. The report must direct semantic resolution to Language Designer and Chief Architect before M0019 implementation continues.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone checklist is reviewed.

## Execution Commands

- Generate tests: `create docs/tests/m0019-nullability-flow-blocked.sh`
- Verify tests fail: `sh docs/tests/m0019-nullability-flow-blocked.sh`
- Ordinary tests: `sh docs/tests/m0019-nullability-flow-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-nullability-flow-blocked.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0019-nullability-flow-blocked.sh`
- Implementation files:
  - None.
- Documentation or checklist files:
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md`
  - `docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md`
  - `docs/tasks/reviews/M0019-001-review.md`
  - `docs/tasks/soundness/M0019-001-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not implement nullability checks.
- Do not implement flow tracking.
- Do not implement smart casts.
- Do not implement mutation invalidation.
- Do not weaken or delete failing tests without reviewer approval.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- Concrete nullable type-use diagnostics beyond M0018 assignment compatibility are unspecified.
- Concrete null-test expression forms that create refinements are unspecified.
- Smart-cast eligibility for immutable bindings versus exclusively borrowed values is unspecified.
- Mutation invalidation rules are unspecified for local assignment, aliasing, member assignment, calls, and suspension.
- Required diagnostic stable identifiers and recovery actions for M0019 are unspecified.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Task references only M0019 and blocks ambiguous nullability and flow typing rules before implementation.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`sh docs/tests/m0019-nullability-flow-blocked.sh` failed because `docs/ambiguities/M0019-nullability-and-flow-typing.md` did not exist.
- 2026-07-10 agent=Language-Lawyer phase=implementation result=pass notes=Filed open ambiguity report for M0019 nullability and flow typing; no compiler behavior changed.
- 2026-07-10 agent=Language-Lawyer phase=ordinary-tests result=pass notes=`sh docs/tests/m0019-nullability-flow-blocked.sh` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md` created a passing soundness report.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md` created review report; concrete review approved blocker after source-of-truth comparison.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0019-nullability-flow-blocked.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.

## Handoff

- Next Agent: `Test Engineer`
- Reason: `Create and run the blocker validator before filing the ambiguity report.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/milestones/M0019-nullability-and-flow-typing.md`
