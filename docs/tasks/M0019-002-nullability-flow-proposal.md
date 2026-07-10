# Task: M0019-002 Draft Nullability And Flow Typing Proposal

## Task Metadata

- Task ID: `M0019-002`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0019-002-nullability-flow-proposal`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Ambiguity: `docs/ambiguities/M0019-nullability-and-flow-typing.md`

## Goal

Draft a non-authoritative ADR proposal for M0019 nullability checking and flow-sensitive smart-cast rules.

## Motivation

M0019 is blocked because accepted ADRs state the high-level direction but do not define concrete null-test forms, nullable misuse rules, smart-cast eligibility, invalidation rules, or diagnostic obligations.

## Scope

- Add `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`.
- Explain competing designs and trade-offs.
- Recommend a draft bootstrap subset for M0019.
- Specify required accepted content for nullability checks, refinements, invalidation, and diagnostics.
- Keep `docs/ambiguities/M0019-nullability-and-flow-typing.md` open.
- Add a validator for proposal completeness.

## Out Of Scope

- Accepting ADR-0028.
- Updating `docs/SPEC.md`.
- Updating accepted ADRs under `docs/adr/`.
- Implementing nullability checks.
- Implementing flow refinement tracking.
- Adding compiler fixtures that depend on unresolved M0019 rules.
- Resolving the M0019 ambiguity.

## Required Inputs

- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Ambiguity report: `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- Type checking core from M0018.
- Accepted ADRs listed in Source Of Truth.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal file exists with non-authority notice, competing designs, recommended draft choice, concrete draft model, required accepted content, diagnostics, deferrals, consequences, and dependencies.
- Negative tests:
  - Accepted ADR-0028 remains absent.
  - `docs/SPEC.md` is not updated with ADR-0028.
  - The M0019 ambiguity report remains open.
- Diagnostic tests:
  - Proposal names required future diagnostics for invalid nullable use, invalidated refinement, unsupported flow rule, and ambiguous flow rule.
- Adversarial tests:
  - Proposal must not infer behavior from Kotlin, Rust, current parser behavior, current tests, or current type checker behavior as authority.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0019-nullability-flow-proposal.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md` does not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a draft proposal only. Do not promote it into accepted ADRs, update SPEC, close the ambiguity report, or implement nullability and flow behavior.

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

- Generate tests: `create docs/tests/m0019-nullability-flow-proposal.sh`
- Verify tests fail: `sh docs/tests/m0019-nullability-flow-proposal.sh`
- Ordinary tests: `sh docs/tests/m0019-nullability-flow-proposal.sh && sh docs/tests/m0019-nullability-flow-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-002-nullability-flow-proposal.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-002-nullability-flow-proposal.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-nullability-flow-proposal.sh && sh docs/tests/m0019-nullability-flow-blocked.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0019-nullability-flow-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
  - `docs/tasks/M0019-002-nullability-flow-proposal.md`
  - `docs/tasks/reviews/M0019-002-review.md`
  - `docs/tasks/soundness/M0019-002-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0019-nullability-and-flow-typing.md`.
- Do not implement nullability checks.
- Do not implement flow refinement tracking.
- Do not implement smart casts.
- Do not implement mutation invalidation.
- Do not introduce diagnostic identifiers as accepted source of truth.

## Ambiguities And Dependencies

- M0019 remains blocked by `docs/ambiguities/M0019-nullability-and-flow-typing.md`.
- Required follow-up reviews: Language Lawyer, Diagnostics Engineer, Adversarial Engineer, Spec Compliance Auditor, Simplicity Guardian, Chief Architect.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0019 nullability and flow typing proposal task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0019-nullability-flow-proposal.sh before adding ADR-0028 proposal.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Proposal validator failed before implementation because docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md was missing.
- 2026-07-10 agent=Language-Designer phase=implementation result=pass notes=Added non-authoritative ADR-0028 nullability and flow typing proposal; M0019 ambiguity remains open.
- 2026-07-10 agent=Language-Designer phase=ordinary-tests result=pass notes=`sh docs/tests/m0019-nullability-flow-proposal.sh` and `sh docs/tests/m0019-nullability-flow-blocked.sh` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-002-nullability-flow-proposal.md` created a passing soundness report.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0019-002-nullability-flow-proposal.md` created review report; concrete review approved proposal-only scope.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0019-nullability-flow-proposal.sh`, `sh docs/tests/m0019-nullability-flow-blocked.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.

## Handoff

- Next Agent: `Test Engineer`
- Reason: `Verify the proposal validator fails before the proposal exists.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/ambiguities/M0019-nullability-and-flow-typing.md`
  - `docs/milestones/M0019-nullability-and-flow-typing.md`
