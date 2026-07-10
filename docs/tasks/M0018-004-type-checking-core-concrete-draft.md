# Task: M0018-004 Revise Type Checking Core Proposal Into Concrete Draft

## Task Metadata

- Task ID: `M0018-004`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task semantic design`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-004-type-checking-core-concrete-draft`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md`
  - `docs/adr/proposals/reviews/ADR-0027-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-spec-compliance-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0027-chief-architect-decision.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`
- Ambiguity: `docs/ambiguities/M0018-type-checking-core.md`

## Goal

Revise the non-authoritative ADR-0027 proposal into a concrete draft that addresses review blockers without accepting it or implementing type checking.

## Motivation

ADR-0027 reviews require concrete decisions for typed output shape, primitive category identity, nullable assignment, call scope, function type application scope, and diagnostic contracts before ADR acceptance.

## Scope

- Revise `docs/adr/proposals/ADR-0027-type-checking-core.md`.
- Add concrete draft sections for typed output, primitive categories, included expressions, assignment compatibility, direct call deferral, function type application deferral, diagnostics, and deferrals.
- Keep ADR-0027 marked as a draft proposal.
- Keep `docs/ambiguities/M0018-type-checking-core.md` open.
- Add a concrete-draft validator.

## Out Of Scope

- Accepting ADR-0027.
- Moving ADR-0027 into `docs/adr/`.
- Updating `docs/SPEC.md`.
- Resolving the M0018 ambiguity.
- Implementing type checking.
- Adding well-typed or ill-typed fixtures that depend on accepted ADR-0027.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0027-type-checking-core.md`
- Proposal reviews: `docs/adr/proposals/reviews/ADR-0027-*.md`
- Ambiguity report: `docs/ambiguities/M0018-type-checking-core.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - ADR-0027 proposal contains concrete draft sections required by review.
  - Draft defines typed output shape.
  - Draft defines primitive category identity without ABI or layout meaning.
  - Draft defines nullable assignment compatibility.
  - Draft explicitly defers direct calls and structural function type application for M0018.
  - Draft diagnostics define primary span, recovery action, source-of-truth citation, and safe suggestion policy.
- Negative tests:
  - ADR-0027 remains non-authoritative.
  - Accepted ADR-0027 remains absent.
  - `docs/SPEC.md` remains unchanged for ADR-0027.
  - Type checking implementation remains absent.
- Adversarial tests:
  - Draft prevents overloads, numeric conversions, member calls, generic solving, ownership, borrowing, HIR, MIR, and backend behavior from being inferred.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0018-type-checking-core-concrete-draft.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0027 does not yet contain the concrete draft sections required by review.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Revise the proposal only. Use review findings as requirements, but keep the result non-authoritative until a later acceptance task.

## Acceptance Criteria

- [ ] Task references exactly one milestone.
- [ ] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0018 remains blocked pending accepted source-of-truth authority.

## Execution Commands

- Generate tests: `create docs/tests/m0018-type-checking-core-concrete-draft.sh`
- Verify tests fail: `sh docs/tests/m0018-type-checking-core-concrete-draft.sh`
- Ordinary tests: `sh docs/tests/m0018-type-checking-core-concrete-draft.sh && sh docs/tests/m0018-type-checking-core-review.sh && sh docs/tests/m0018-type-checking-core-proposal.sh && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-004-type-checking-core-concrete-draft.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-004-type-checking-core-concrete-draft.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-concrete-draft.sh && sh docs/tests/m0018-type-checking-core-review.sh && sh docs/tests/m0018-type-checking-core-proposal.sh && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0018-type-checking-core-concrete-draft.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0027-type-checking-core.md`
  - `docs/tasks/M0018-004-type-checking-core-concrete-draft.md`
  - `docs/tasks/reviews/M0018-004-review.md`
  - `docs/tasks/soundness/M0018-004-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0018-type-checking-core.md`.
- Do not accept ADR-0027.
- Do not implement type checking.

## Ambiguities And Dependencies

- ADR-0027 remains a proposal until a later acceptance task.
- M0018 remains blocked until accepted source of truth exists.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 ADR-0027 concrete draft revision task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0018-type-checking-core-concrete-draft.sh before revising ADR-0027.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Concrete draft validator failed before implementation because ADR-0027 lacked required concrete draft sections.
- 2026-07-10 main_task=Language-Designer phase=ordinary-tests result=pass notes=Revised ADR-0027 proposal into concrete non-authoritative draft; M0018 concrete, review, proposal, and ambiguity blocker validators passed after validator alignment.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-004-type-checking-core-concrete-draft.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-004-type-checking-core-concrete-draft.md` created review and concrete review approved draft-only scope.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-concrete-draft.sh`, `sh docs/tests/m0018-type-checking-core-review.sh`, `sh docs/tests/m0018-type-checking-core-proposal.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
