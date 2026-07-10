# Task: M0016-002 Draft name resolution policy proposal

## Task Metadata

- Task ID: `M0016-002`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-002-name-resolution-policy-proposal`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-designer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/spec-compliance-auditor.md`

## Goal

Draft a non-authoritative ADR proposal for the bootstrap M0016 name-resolution policy sufficient for later review.

## Motivation

M0016 is blocked because accepted source of truth does not define lookup order, import semantics, scope boundaries, duplicate-name behavior, or resolution diagnostics. A concrete proposal is needed before review and acceptance can unblock implementation.

## Scope

- Add `docs/adr/proposals/ADR-0026-name-resolution-policy.md`.
- Explain competing designs and trade-offs.
- Recommend a draft bootstrap resolution subset.
- Enumerate required accepted content before implementation.
- State explicit non-authority notice and implementation prohibition.
- Preserve `docs/ambiguities/M0016-name-resolution-policy.md` as open.

## Out Of Scope

- Accepting ADR-0026.
- Updating `docs/SPEC.md`.
- Updating accepted ADRs under `docs/adr/`.
- Implementing name resolution.
- Implementing resolution diagnostics.
- Resolving the M0016 ambiguity.

## Required Inputs

- Milestone: `docs/milestones/M0016-name-resolution-pass.md`
- Ambiguity report: `docs/ambiguities/M0016-name-resolution-policy.md`
- Spec: `docs/SPEC.md`
- Accepted ADRs listed in Source Of Truth.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal file exists with non-authority notice, competing designs, recommended draft choice, required accepted content, required diagnostics, downstream consequences, and dependencies.
- Negative tests:
  - Accepted ADR-0026 does not exist.
  - `docs/SPEC.md` is not updated with ADR-0026.
  - Name-resolution implementation files remain absent.
- Diagnostic tests:
  - Proposal names required diagnostics for future accepted unresolved-name, duplicate-name, inaccessible-name, and ambiguous-name behavior.
- Adversarial tests:
  - Proposal must not infer behavior from current parser, name table storage, or external language behavior as authority.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0016-name-resolution-policy-proposal.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md` does not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a draft proposal only. Do not move the draft into accepted ADRs or modify compiler behavior.

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

- Generate tests: `create docs/tests/m0016-name-resolution-policy-proposal.sh`
- Verify tests fail: `docs/tests/m0016-name-resolution-policy-proposal.sh`
- Ordinary tests: `docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-002-name-resolution-policy-proposal.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-002-name-resolution-policy-proposal.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0016-name-resolution-policy-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/tasks/M0016-002-name-resolution-policy-proposal.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0016-name-resolution-policy.md`.
- Do not implement name resolution.
- Do not implement resolution diagnostics.
- Do not treat the proposal as source of truth.

## Ambiguities And Dependencies

- M0016 remains blocked by `docs/ambiguities/M0016-name-resolution-policy.md`.
- Required follow-up reviews: Language Lawyer, Diagnostics Engineer, Adversarial Engineer, Spec Compliance Auditor, Simplicity Guardian, Chief Architect.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 name resolution policy proposal task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0016-name-resolution-policy-proposal.sh before adding ADR-0026 proposal.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=docs/tests/m0016-name-resolution-policy-proposal.sh failed before implementation because ADR-0026 proposal file was missing.
2026-07-10 agent=Language-Designer phase=implementation result=pass notes=Added non-authoritative ADR-0026 name resolution policy draft proposal and kept M0016 ambiguity open.
2026-07-10 agent=Language-Designer phase=ordinary-tests result=pass notes=M0016 name resolution policy proposal and blocker validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-002-soundness.md after ordinary-test evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0016-002-review.md approves non-authoritative proposal scope.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0016/M0015/M0002 validation scripts passed.
```

## Handoff

- Next Agent: `Language Designer`
- Reason: `Draft non-authoritative name-resolution policy proposal.`
- Required Context:
  - This task file
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
  - accepted ADRs listed in Source Of Truth
