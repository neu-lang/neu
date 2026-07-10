# Task: M0014-005 Accept module package and visibility model ADR

## Task Metadata

- Task ID: `M0014-005`
- Milestone: `M0014`
- Milestone File: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Status: `complete`
- Owner Agent: `Chief Architect`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0014-005-accept-module-package-visibility-model-adr`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/proposals/reviews/ADR-0025-chief-architect-decision.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/chief-architect.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/spec-compliance-auditor.md`

## Goal

Accept the concrete ADR-0025 module, package, and visibility model into source of truth so M0014 implementation can proceed against explicit semantics.

## Motivation

M0014 is blocked by an open ambiguity report. The proposal has been revised into a concrete draft and reviewed. Acceptance must move the decision into `docs/adr/`, update `docs/SPEC.md`, resolve the ambiguity report, and update validators so later implementation compares against accepted source of truth.

## Scope

- Create accepted `docs/adr/ADR-0025-module-package-visibility-model.md`.
- Update `docs/SPEC.md` with the accepted ADR-0025 semantics.
- Resolve `docs/ambiguities/M0014-module-package-visibility-model.md`.
- Update the Chief Architect decision to approved.
- Update M0014 validators to reflect accepted source of truth.
- Keep module metadata implementation out of scope.

## Out Of Scope

- Implementing module metadata structures.
- Implementing visibility extraction.
- Implementing name resolution.
- Defining package manager, manifest, dependency graph, target-pack artifact, or incremental compilation behavior.
- Changing the existing ADR-0025 proposal history except as required by validators.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
- Reviews: `docs/adr/proposals/reviews/ADR-0025-*.md`
- Ambiguity: `docs/ambiguities/M0014-module-package-visibility-model.md`
- Milestone: `docs/milestones/M0014-module-package-and-visibility-model.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Accepted ADR-0025 exists and has `Status: Accepted`.
  - `docs/SPEC.md` contains ADR-0025 semantics.
  - Ambiguity report is resolved.
  - Chief Architect decision is approved.
- Negative tests:
  - Compiler module implementation remains absent.
  - Accepted ADR does not define manifest, package manager, target-pack artifact, or name-resolution behavior.
- Diagnostic tests:
  - Accepted ADR diagnostic obligations include names, location, recovery, suggestion, and source-of-truth citation.
- Adversarial tests:
  - Acceptance does not broaden beyond M0014 source-of-truth alignment.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0014-module-package-visibility-model-accepted.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted ADR-0025 does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Move the concrete model from proposal to accepted ADR form, add the corresponding SPEC section, close the ambiguity with the accepted ADR as the resolution source, update validators, and do not add compiler behavior.

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
- [x] M0014 ambiguity is resolved by accepted source of truth.

## Execution Commands

- Generate tests: `create docs/tests/m0014-module-package-visibility-model-accepted.sh`
- Verify tests fail: `docs/tests/m0014-module-package-visibility-model-accepted.sh`
- Ordinary tests: `docs/tests/m0014-module-package-visibility-model-accepted.sh && docs/tests/m0014-module-package-visibility-model-concrete-draft.sh && docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0014-005-accept-module-package-visibility-model-adr.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0014-005-accept-module-package-visibility-model-adr.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0014-module-package-visibility-model-accepted.sh && docs/tests/m0014-module-package-visibility-model-concrete-draft.sh && docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0014-module-package-visibility-model-accepted.sh`
  - `docs/tests/m0014-module-package-visibility-model-concrete-draft.sh`
  - `docs/tests/m0014-module-package-visibility-model-review.sh`
  - `docs/tests/m0014-module-package-visibility-model-proposal.sh`
  - `docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0014-module-package-visibility-model.md`
  - `docs/adr/proposals/reviews/ADR-0025-chief-architect-decision.md`
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
  - `docs/tasks/M0014-005-accept-module-package-visibility-model-adr.md`

## Forbidden Changes

- Do not implement module model code.
- Do not implement name resolution.
- Do not weaken or delete existing M0014 validators to hide missing accepted semantics.
- Do not define package manager, manifest, target-pack artifact, or module dependency semantics.

## Ambiguities And Dependencies

- M0014 implementation remains dependent on this task until accepted ADR-0025 and SPEC updates are present.
- Module dependencies, target-pack compatibility, manifest syntax, and name resolution remain deferred to later milestones.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0014 accepted ADR task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created accepted ADR validator before adding accepted source of truth.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=docs/tests/m0014-module-package-visibility-model-accepted.sh failed before implementation because accepted ADR-0025 was missing.
2026-07-10 agent=Chief-Architect phase=implementation result=pass notes=Accepted ADR-0025, updated SPEC, resolved M0014 ambiguity, and approved the Chief Architect decision.
2026-07-10 agent=Chief-Architect phase=ordinary-tests result=pass notes=M0014 accepted, concrete draft, review, proposal, and authority validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0014-005-soundness.md after ordinary-test evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0014-005-review.md approves ADR-0025 acceptance scope.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0014-M0002 validation scripts passed.
```

## Handoff

- Next Agent: `Chief Architect`
- Reason: `Accept ADR-0025 and resolve M0014 ambiguity.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/proposals/reviews/ADR-0025-*.md`
  - `docs/ambiguities/M0014-module-package-visibility-model.md`
