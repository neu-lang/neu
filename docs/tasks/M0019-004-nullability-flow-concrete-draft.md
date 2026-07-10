# Task: M0019-004 Revise Nullability And Flow Typing Proposal Into Concrete Draft

## Task Metadata

- Task ID: `M0019-004`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0019-004-nullability-flow-concrete-draft`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/proposals/reviews/ADR-0028-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-spec-compliance-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0028-chief-architect-decision.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Ambiguity: `docs/ambiguities/M0019-nullability-and-flow-typing.md`

## Goal

Revise the non-authoritative ADR-0028 proposal into a concrete draft that addresses review blockers without accepting it or implementing nullability and flow typing.

## Motivation

ADR-0028 reviews require concrete decisions for branch region boundaries, condition recognition, refined output shape, shadowing, invalidation, and diagnostic contracts before ADR acceptance.

## Scope

- Revise `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`.
- Add concrete draft sections for branch region boundaries, null-test recognition, refined output, shadowing, invalidation, diagnostics, and deferrals.
- Keep ADR-0028 marked as a draft proposal.
- Keep `docs/ambiguities/M0019-nullability-and-flow-typing.md` open.
- Add a concrete-draft validator.

## Out Of Scope

- Accepting ADR-0028.
- Moving ADR-0028 into `docs/adr/`.
- Updating `docs/SPEC.md`.
- Resolving the M0019 ambiguity.
- Implementing nullability checks.
- Implementing flow refinement tracking.
- Adding source examples for unaccepted semantics.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- Proposal reviews: `docs/adr/proposals/reviews/ADR-0028-*.md`
- Ambiguity report: `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - ADR-0028 proposal contains concrete draft sections required by review.
  - Draft defines branch region boundaries.
  - Draft defines accepted null-test recognition without requiring general binary expression typing.
  - Draft defines refined output shape and preserves original binding type.
  - Draft defines shadowing and nested block behavior.
  - Draft diagnostics define primary span, recovery action, source-of-truth citation, safe suggestion policy, and stable rule identifiers.
- Negative tests:
  - ADR-0028 remains non-authoritative.
  - Accepted ADR-0028 remains absent.
  - `docs/SPEC.md` remains unchanged for ADR-0028.
  - M0019 ambiguity remains open.
  - Flow implementation remains absent.
- Adversarial tests:
  - Draft prevents members, calls, aliases, coroutine suspension, unsafe, FFI, generics, patterns, exclusive borrows, HIR, MIR, and backend behavior from being inferred.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0019-nullability-flow-concrete-draft.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0028 does not yet contain the concrete draft sections required by review.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Revise the proposal only. Use review findings as requirements, but keep the result non-authoritative until a later acceptance task.

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
- [x] M0019 remains blocked pending accepted source-of-truth authority.

## Execution Commands

- Generate tests: `create docs/tests/m0019-nullability-flow-concrete-draft.sh`
- Verify tests fail: `sh docs/tests/m0019-nullability-flow-concrete-draft.sh`
- Ordinary tests: `sh docs/tests/m0019-nullability-flow-concrete-draft.sh && sh docs/tests/m0019-nullability-flow-review.sh && sh docs/tests/m0019-nullability-flow-proposal.sh && sh docs/tests/m0019-nullability-flow-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-004-nullability-flow-concrete-draft.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-004-nullability-flow-concrete-draft.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-nullability-flow-concrete-draft.sh && sh docs/tests/m0019-nullability-flow-review.sh && sh docs/tests/m0019-nullability-flow-proposal.sh && sh docs/tests/m0019-nullability-flow-blocked.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0019-nullability-flow-concrete-draft.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/tasks/M0019-004-nullability-flow-concrete-draft.md`
  - `docs/tasks/reviews/M0019-004-review.md`
  - `docs/tasks/soundness/M0019-004-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0019-nullability-and-flow-typing.md`.
- Do not accept ADR-0028.
- Do not implement nullability checks.
- Do not implement flow refinement tracking.

## Ambiguities And Dependencies

- ADR-0028 remains a proposal until a later acceptance task.
- M0019 remains blocked until accepted source of truth exists.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0019 ADR-0028 concrete draft revision task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0019-nullability-flow-concrete-draft.sh before revising ADR-0028.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Concrete draft validator failed before implementation because ADR-0028 lacked required concrete draft sections.
- 2026-07-10 agent=Language-Designer phase=implementation result=pass notes=Revised ADR-0028 proposal with concrete non-authoritative draft sections for null-test recognition, branch boundaries, refined output, shadowing, and diagnostics.
- 2026-07-10 agent=Language-Designer phase=ordinary-tests result=pass notes=`sh docs/tests/m0019-nullability-flow-concrete-draft.sh`, `sh docs/tests/m0019-nullability-flow-review.sh`, `sh docs/tests/m0019-nullability-flow-proposal.sh`, and `sh docs/tests/m0019-nullability-flow-blocked.sh` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-004-nullability-flow-concrete-draft.md` created a passing soundness report.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0019-004-nullability-flow-concrete-draft.md` created review report; concrete review approved draft-only scope.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0019-nullability-flow-concrete-draft.sh`, `sh docs/tests/m0019-nullability-flow-review.sh`, `sh docs/tests/m0019-nullability-flow-proposal.sh`, `sh docs/tests/m0019-nullability-flow-blocked.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.

## Handoff

- Next Agent: `Test Engineer`
- Reason: `Verify the concrete draft validator fails before ADR-0028 is revised.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/proposals/reviews/ADR-0028-*.md`
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md`
