# Task: M0016-005 Accept name resolution policy ADR

## Task Metadata

- Task ID: `M0016-005`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Chief Architect`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-005-accept-name-resolution-policy-adr`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/adr/proposals/reviews/ADR-0026-chief-architect-decision.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/chief-architect.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/spec-compliance-auditor.md`
  - `.codex/agents/reviewer.md`

## Goal

Accept ADR-0026 as the name-resolution policy source of truth, resolve the M0016 ambiguity, and unblock implementation tasks without implementing name resolution.

## Motivation

ADR-0026 has been drafted, reviewed, and revised into a concrete bootstrap model. M0016 implementation must not proceed until the accepted ADR and SPEC summary provide authoritative lookup, scope, duplicate, ambiguity, deferral, and diagnostic rules.

## Scope

- Create accepted `docs/adr/ADR-0026-name-resolution-policy.md`.
- Update `docs/SPEC.md` with an ADR-0026 summary.
- Resolve `docs/ambiguities/M0016-name-resolution-policy.md`.
- Update `docs/adr/proposals/reviews/ADR-0026-chief-architect-decision.md` to approved.
- Update `docs/milestones/M0016-name-resolution-pass.md` to record accepted source-of-truth readiness.
- Add accepted-state validator for ADR-0026.

## Out Of Scope

- Implementing name resolution.
- Implementing resolution diagnostics.
- Adding resolution fixtures.
- Adding HIR or type checking.
- Activating imports.
- Defining cross-module dependency lookup.
- Defining member lookup, overload resolution, extension lookup, or type-directed lookup.

## Required Inputs

- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Reviews: `docs/adr/proposals/reviews/ADR-0026-*.md`
- Ambiguity: `docs/ambiguities/M0016-name-resolution-policy.md`
- Milestone: `docs/milestones/M0016-name-resolution-pass.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Accepted ADR-0026 exists and has `Status: Accepted`.
  - `docs/SPEC.md` contains ADR-0026 semantics.
  - Ambiguity report is resolved.
  - Chief Architect decision is approved.
  - Milestone records accepted source-of-truth readiness.
- Negative tests:
  - Compiler name-resolution implementation remains absent.
  - Accepted ADR does not activate imports, cross-module lookup, member lookup, overload resolution, extension lookup, or type-directed lookup.
- Diagnostic tests:
  - Accepted ADR diagnostic obligations include primary span, recovery action, safe suggestion policy, and source-of-truth citation.
- Adversarial tests:
  - Acceptance does not broaden beyond the reviewed bootstrap resolution subset.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted `docs/adr/ADR-0026-name-resolution-policy.md` does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Promote the reviewed concrete proposal into an accepted ADR, add the corresponding SPEC summary, resolve the ambiguity, approve the Chief Architect decision, update milestone readiness, and leave implementation for later tasks.

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
- [x] M0016 ambiguity is resolved by accepted source of truth.

## Execution Commands

- Generate tests: `create docs/tests/m0016-name-resolution-policy-accepted.sh`
- Verify tests fail: `docs/tests/m0016-name-resolution-policy-accepted.sh`
- Ordinary tests: `docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-concrete-draft.sh && docs/tests/m0016-name-resolution-policy-review.sh && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-005-accept-name-resolution-policy-adr.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-005-accept-name-resolution-policy-adr.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-concrete-draft.sh && docs/tests/m0016-name-resolution-policy-review.sh && docs/tests/m0016-name-resolution-policy-proposal.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`
  - `docs/tests/m0016-name-resolution-concrete-draft.sh`
  - `docs/tests/m0016-name-resolution-policy-review.sh`
  - `docs/tests/m0016-name-resolution-policy-proposal.sh`
  - `docs/tests/m0016-name-resolution-blocked.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/adr/proposals/reviews/ADR-0026-chief-architect-decision.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
  - `docs/tasks/M0016-005-accept-name-resolution-policy-adr.md`

## Forbidden Changes

- Do not implement name resolution.
- Do not implement resolution diagnostics.
- Do not add resolution fixtures.
- Do not activate imports.
- Do not define cross-module dependency lookup.
- Do not define member lookup, overload resolution, extension lookup, or type-directed lookup.
- Do not weaken or delete existing M0016 validators to hide missing accepted semantics.

## Ambiguities And Dependencies

- M0016 implementation remains dependent on this task until accepted ADR-0026 and SPEC updates are present.
- Imports, cross-module lookup, member lookup, overloads, extensions, and type-directed lookup remain deferred after this task.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created ADR-0026 acceptance task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created accepted-state validator before accepting ADR-0026.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Accepted-state validator failed before implementation because docs/adr/ADR-0026-name-resolution-policy.md was missing.
2026-07-10 agent=Chief-Architect phase=implementation result=pass notes=Accepted ADR-0026, updated SPEC, resolved M0016 ambiguity, approved Chief Architect decision, and updated milestone source-of-truth readiness.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=M0016 accepted, concrete draft, review, proposal, and authority validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-005-soundness.md after ordinary tests were recorded.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0016-005-review.md approved source-of-truth acceptance scope pending final CI gate.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 accepted/concrete/review/proposal/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Chief Architect`
- Reason: `Accept ADR-0026 and resolve M0016 ambiguity.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
  - `docs/adr/proposals/reviews/ADR-0026-*.md`
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
