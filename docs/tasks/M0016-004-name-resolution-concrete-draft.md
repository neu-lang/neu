# Task: M0016-004 Revise name resolution policy into concrete draft

## Task Metadata

- Task ID: `M0016-004`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-004-name-resolution-concrete-draft`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Proposal reviews:
  - `docs/adr/proposals/reviews/ADR-0026-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-spec-compliance-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0026-chief-architect-decision.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-designer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/adversarial-engineer.md`

## Goal

Revise the non-authoritative ADR-0026 proposal into a concrete draft model that answers the review blockers without accepting the ADR or implementing name resolution.

## Motivation

M0016 remains blocked because ADR-0026 gives a broad direction but does not yet enumerate the exact bootstrap subset, binding positions, lookup rules, duplicate behavior, shadowing behavior, or diagnostics that reviewers require before acceptance.

## Scope

- Revise `docs/adr/proposals/ADR-0026-name-resolution-policy.md`.
- Add concrete draft sections for AST node kinds, declaration and binding positions, scope boundaries, declaration order, shadowing, duplicate handling, lookup behavior, visibility, diagnostics, and deferrals.
- Keep ADR-0026 marked as a draft proposal.
- Keep `docs/ambiguities/M0016-name-resolution-policy.md` open.

## Out Of Scope

- Accepting ADR-0026.
- Moving ADR-0026 into `docs/adr/`.
- Updating `docs/SPEC.md`.
- Closing the M0016 ambiguity.
- Adding resolution fixtures.
- Implementing name resolution.
- Implementing resolution diagnostics.

## Required Inputs

- Milestone: `docs/milestones/M0016-name-resolution-pass.md`
- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Ambiguity report: `docs/ambiguities/M0016-name-resolution-policy.md`
- ADR-0026 proposal reviews.
- Accepted ADRs listed in Source Of Truth.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - ADR-0026 proposal includes concrete draft sections required by review.
  - Concrete draft explicitly names included and excluded AST node kinds.
  - Concrete draft defines declaration order, local-before-declaration behavior, shadowing, duplicates, and ambiguity.
- Negative tests:
  - ADR-0026 remains non-authoritative.
  - Accepted ADR-0026 remains absent.
  - `docs/SPEC.md` remains unchanged for ADR-0026.
  - Name-resolution implementation files remain absent.
- Diagnostic tests:
  - Draft diagnostics define primary span, recovery action, source-of-truth citation, and safe suggestion policy.
- Adversarial tests:
  - Draft prevents active import, cross-module, member, overload, extension, and type-directed lookup from being inferred.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0016-name-resolution-concrete-draft.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0026 does not yet contain the concrete draft sections required by review.
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
- [x] M0016 remains blocked pending accepted source-of-truth authority.

## Execution Commands

- Generate tests: `create docs/tests/m0016-name-resolution-concrete-draft.sh`
- Verify tests fail: `docs/tests/m0016-name-resolution-concrete-draft.sh`
- Ordinary tests: `docs/tests/m0016-name-resolution-concrete-draft.sh && docs/tests/m0016-name-resolution-policy-review.sh && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-004-name-resolution-concrete-draft.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-004-name-resolution-concrete-draft.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-concrete-draft.sh && docs/tests/m0016-name-resolution-policy-review.sh && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0016-name-resolution-concrete-draft.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/tasks/M0016-004-name-resolution-concrete-draft.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0016-name-resolution-policy.md`.
- Do not implement name resolution.
- Do not implement resolution diagnostics.
- Do not treat the proposal as source of truth.

## Ambiguities And Dependencies

- ADR-0026 remains a proposal until a later acceptance task.
- M0016 remains blocked until accepted source of truth exists.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 concrete draft revision task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0016-name-resolution-concrete-draft.sh before revising ADR-0026 concrete draft sections.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=docs/tests/m0016-name-resolution-concrete-draft.sh failed before implementation because ADR-0026 did not contain the required concrete draft sections.
2026-07-10 agent=Language-Designer phase=implementation result=pass notes=Revised ADR-0026 proposal with concrete non-authoritative draft sections while keeping M0016 blocked.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=docs/tests/m0016-name-resolution-concrete-draft.sh, policy-review, policy-proposal, and blocked validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-004-soundness.md after ordinary tests were recorded.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0016-004-review.md approved draft-only scope pending final CI gate.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 concrete/review/proposal/blocker validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Language Designer`
- Reason: `Revise ADR-0026 proposal into a concrete draft model.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/adr/proposals/reviews/ADR-0026-*.md`
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
