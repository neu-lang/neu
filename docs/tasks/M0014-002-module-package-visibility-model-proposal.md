# Task: M0014-002 Draft module package and visibility model proposal

## Task Metadata

- Task ID: `M0014-002`
- Milestone: `M0014`
- Milestone File: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Status: `complete`
- Owner Agent: `Language Designer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0014-002-module-package-visibility-model-proposal`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-designer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/build-engineer.md`
  - `.codex/agents/spec-compliance-auditor.md`

## Goal

Draft a non-authoritative ADR proposal for module identity, package mapping, namespace behavior, visibility categories, and module metadata sufficient for later M0014 review.

## Motivation

M0014 is blocked because accepted ADRs define module intent but not concrete frontend model rules. A proposal is needed before review and acceptance can unblock module metadata implementation.

## Scope

- Add `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`.
- Explain competing designs and trade-offs.
- Recommend a draft model direction.
- Enumerate required accepted content before implementation.
- State explicit non-authority notice and implementation prohibition.
- Preserve `docs/ambiguities/M0014-module-package-visibility-model.md` as open.

## Out Of Scope

- Accepting ADR-0025.
- Updating `docs/SPEC.md`.
- Updating accepted ADRs under `docs/adr/`.
- Implementing module model data structures.
- Implementing visibility metadata extraction.
- Resolving the M0014 ambiguity.

## Required Inputs

- Milestone: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Ambiguity report: `docs/ambiguities/M0014-module-package-visibility-model.md`
- Spec sections:
  - `ADR-0017: Modules, Visibility, And API Evolution`
  - `ADR-0022: Declaration Syntax`
- ADRs:
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Proposal file exists with non-authority notice, competing designs, recommended draft choice, required accepted content, downstream consequences, and dependencies.
- Negative tests:
  - Accepted ADR-0025 does not exist.
  - `docs/SPEC.md` is not updated with ADR-0025.
  - Module model implementation files remain absent.
- Diagnostic tests:
  - Proposal names required diagnostics for future accepted module and visibility model.
- Adversarial tests:
  - Proposal must not infer behavior from current parser, file paths, package spelling, or external language behavior as authority.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0014-module-package-visibility-model-proposal.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md` does not exist.
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
- [x] M0014 remains blocked pending accepted model authority.

## Execution Commands

- Generate tests: `create docs/tests/m0014-module-package-visibility-model-proposal.sh`
- Verify tests fail: `docs/tests/m0014-module-package-visibility-model-proposal.sh`
- Ordinary tests: `docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Adversarial tests: `docs/tests/m0014-module-package-visibility-model-proposal.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0014-002-module-package-visibility-model-proposal.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0014-module-package-visibility-model-proposal.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/tasks/M0014-002-module-package-visibility-model-proposal.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not close `docs/ambiguities/M0014-module-package-visibility-model.md`.
- Do not implement module model code.
- Do not treat the proposal as source of truth.

## Ambiguities And Dependencies

- M0014 remains blocked by `docs/ambiguities/M0014-module-package-visibility-model.md`.
- Required follow-up reviews: Language Lawyer, Build Engineer, Spec Compliance Auditor, Simplicity Guardian, Chief Architect.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0014 module/package/visibility model proposal task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created proposal validator before adding proposal file.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=docs/tests/m0014-module-package-visibility-model-proposal.sh failed because ADR-0025 proposal file was missing.
2026-07-10 agent=Language-Designer phase=implementation result=pass notes=Added non-authoritative ADR-0025 module/package/visibility model draft proposal.
2026-07-10 agent=Language-Designer phase=ordinary-tests result=pass notes=docs/tests/m0014-module-package-visibility-model-proposal.sh and docs/tests/m0014-module-package-visibility-model-blocked.sh passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0014-002-soundness.md after ordinary-test evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0014-002-review.md approves non-authoritative proposal scope.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Focused M0014 proposal validation command passed.
```

## Handoff

- Next Agent: `Language Lawyer`
- Reason: `Audit the draft before acceptance.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/ambiguities/M0014-module-package-visibility-model.md`
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
