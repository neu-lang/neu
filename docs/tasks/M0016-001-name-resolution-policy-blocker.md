# Task: M0016-001 Record name resolution policy blocker

## Task Metadata

- Task ID: `M0016-001`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Language Lawyer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-001-name-resolution-policy-blocker`

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
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/spec-compliance-auditor.md`
  - `.codex/agents/adversarial-engineer.md`

## Goal

Record that M0016 implementation is blocked until accepted source of truth defines the approved name-resolution subset, lookup order, import semantics, scope boundaries, duplicate-name behavior, and unresolved-name diagnostics.

## Motivation

M0016 asks for approved local, module, and declaration name resolution. Existing accepted ADRs define syntax, module metadata, and symbol infrastructure, but they explicitly defer or omit import lookup, dependency lookup, scope hierarchy, duplicate declaration legality, and resolution diagnostics. Implementing a name-resolution pass now would invent user-visible language semantics.

## Scope

- Add an M0016 ambiguity report.
- Add a blocked-state validator for name resolution work.
- Update the M0016 milestone checklist to record that ambiguous resolution cases are not guessed.

## Out Of Scope

- Implementing name resolution.
- Implementing import resolution.
- Implementing scope stacks.
- Implementing unresolved-name diagnostics.
- Implementing duplicate-name diagnostics.
- Changing `docs/SPEC.md` or accepted ADRs.
- Resolving the ambiguity.

## Required Inputs

- Milestone: `docs/milestones/M0016-name-resolution-pass.md`
- Spec: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Validator proves the M0016 ambiguity report exists and is open.
  - Validator proves M0016 records ambiguous resolution cases are not guessed.
- Negative tests:
  - Validator fails if name-resolution implementation files appear before accepted authority exists.
- Diagnostic tests:
  - Validator confirms unresolved-name and duplicate-name diagnostics are blocked rather than invented.
- Adversarial tests:
  - Validator rejects attempts to infer import, scope, duplicate, or lookup semantics from parser behavior.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0016-name-resolution-blocked.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - M0016 ambiguity report does not yet exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add an ambiguity report and blocked-state validator only. Do not add name-resolution code or diagnostics.

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
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0016-name-resolution-blocked.sh`
- Verify tests fail: `docs/tests/m0016-name-resolution-blocked.sh`
- Ordinary tests: `docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0015-symbol-interner.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-001-name-resolution-policy-blocker.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-001-name-resolution-policy-blocker.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0015-symbol-interner.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0016-name-resolution-blocked.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/ambiguities/M0016-name-resolution-policy.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
  - `docs/tasks/M0016-001-name-resolution-policy-blocker.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs.
- Do not weaken or delete failing tests without reviewer approval.
- Do not implement name resolution.
- Do not implement import resolution.
- Do not implement duplicate-name diagnostics.
- Do not implement unresolved-name diagnostics.
- Do not modify parser behavior.

## Ambiguities And Dependencies

- M0016 is blocked by `docs/ambiguities/M0016-name-resolution-policy.md`.
- The next safe task is for Language Designer to draft a non-authoritative name-resolution policy proposal.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 name resolution policy blocker task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0016-name-resolution-blocked.sh before filing the ambiguity report.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=docs/tests/m0016-name-resolution-blocked.sh failed before implementation because docs/ambiguities/M0016-name-resolution-policy.md was missing.
2026-07-10 agent=Language-Lawyer phase=implementation result=pass notes=Filed M0016 name resolution policy ambiguity report without adding name-resolution implementation.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=M0016 blocker, M0015 name table, and M0015 symbol interner validators passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-001-soundness.md after ordinary-test evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0016-001-review.md approves blocked scope.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0016/M0015/M0002 validation scripts passed.
```

## Handoff

- Next Agent: `Language Lawyer`
- Reason: `File the M0016 name resolution policy ambiguity report.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/milestones/M0016-name-resolution-pass.md`
  - accepted ADRs listed above
