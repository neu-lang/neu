# Task: M0014-003 Review module package and visibility model proposal

## Task Metadata

- Task ID: `M0014-003`
- Milestone: `M0014`
- Milestone File: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0014-003-module-package-visibility-model-proposal-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Review ADR-0025 as a non-authoritative module, package, and visibility model proposal and identify what must be revised before acceptance.

## Motivation

ADR-0025 proposes a direction but still leaves accepted model details unresolved. Review must prevent implementation from depending on it and must identify missing decisions that block module metadata work.

## Scope

- Add main-task language review review for semantic consistency and missing authority.
- Add main-task build check review for module identity, invocation, target-pack, and artifact compatibility risks.
- Add main-task specification check review for source-of-truth boundaries.
- Add main-task simplicity check review for scope and abstraction control.
- Add main task decision artifact preserving pending status.

## Out Of Scope

- Revising ADR-0025.
- Accepting ADR-0025.
- Changing `docs/SPEC.md`.
- Creating module model fixtures.
- Adding module metadata implementation.
- Resolving `docs/ambiguities/M0014-module-package-visibility-model.md`.

## Required Inputs

- Milestone: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Proposal: `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
- Ambiguity report: `docs/ambiguities/M0014-module-package-visibility-model.md`
- Accepted context:
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - All required review artifacts exist and request revision before acceptance.
- Negative tests:
  - No accepted ADR-0025, spec section, module implementation, name resolution, or module fixtures are added.
- Diagnostic tests:
  - Reviews request concrete diagnostics with primary span or external input location, recovery action, and safe suggestion policy.
- Adversarial tests:
  - Reviews keep M0014 blocked and reject semantic inference from proposal text.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0014-module-package-visibility-model-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0025 review artifacts do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create review artifacts only. Keep ADR-0025 as a draft proposal and leave the ambiguity open.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0014 remains blocked pending accepted model authority.

## Execution Commands

- Generate tests: `create docs/tests/m0014-module-package-visibility-model-review.sh`
- Verify tests fail: `docs/tests/m0014-module-package-visibility-model-review.sh`
- Ordinary tests: `docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Adversarial tests: `docs/tests/m0014-module-package-visibility-model-review.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0014-003-module-package-visibility-model-proposal-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0014-module-package-visibility-model-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/reviews/ADR-0025-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0025-build-engineer-review.md`
  - `docs/adr/proposals/reviews/ADR-0025-spec-compliance-review.md`
  - `docs/adr/proposals/reviews/ADR-0025-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0025-chief-architect-decision.md`
  - `docs/tasks/M0014-003-module-package-visibility-model-proposal-review.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not accept ADR-0025.
- Do not add module metadata implementation.
- Do not add name resolution.

## Ambiguities And Dependencies

- ADR-0025 remains a proposal.
- M0014 remains blocked until accepted source of truth exists.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0014 ADR-0025 review task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0014-module-package-visibility-model-review.sh before adding review artifacts.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Review validator failed before implementation because ADR-0025 review artifacts did not exist.
2026-07-10 main_task=Language-Lawyer phase=implementation result=pass notes=Added required review artifacts and kept ADR-0025 pending revision.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=M0014 review, proposal, and blocker validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0014-003-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0014-003-review.md approves review-only scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Focused M0014 review validation command passed.
```

## Handoff

- Next main task: `main-task semantic design`
- Reason: `Revise ADR-0025 into a concrete draft model after reviews.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/proposals/reviews/ADR-0025-*.md`
