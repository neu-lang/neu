# Task: M0014-001 Record module package and visibility model blocker

## Task Metadata

- Task ID: `M0014-001`
- Milestone: `M0014`
- Milestone File: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0014-001-module-package-visibility-model-blocker`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Record that M0014 implementation is blocked until module identity, package mapping, namespace behavior, and visibility semantics have accepted source-of-truth authority.

## Motivation

M0014 requires deterministic module identity and visibility metadata. ADR-0017 selects modules as explicit compilation and visibility units, but it does not define enough concrete rules for implementation. Guessing would create user-visible name resolution and access-control semantics without authority.

## Scope

- Add an M0014 ambiguity report.
- Add a blocked-state validator for module, package, and visibility model work.
- Update the M0014 milestone checklist to record that unspecified visibility rules are recorded.

## Out Of Scope

- Drafting the module model ADR.
- Implementing module metadata data structures.
- Implementing visibility metadata extraction.
- Implementing package or namespace representation.
- Implementing name resolution.
- Changing `docs/SPEC.md` or accepted ADRs.
- Resolving the ambiguity.

## Required Inputs

- Milestone: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Spec sections:
  - `ADR-0017: Modules, Visibility, And API Evolution`
  - `ADR-0022: Declaration Syntax`
- ADRs:
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Validator proves the M0014 ambiguity report exists and is open.
- Negative tests:
  - Validator fails if module model implementation files appear before accepted authority exists.
- Diagnostic tests:
  - Not applicable; diagnostics cannot be implemented before model authority.
- Adversarial tests:
  - Validator rejects attempts to infer module or visibility semantics from parser behavior.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Expected pre-implementation result: `pass`
- Failure reason expected before implementation:
  - Not applicable; blocker-state validator should pass when M0014 is correctly blocked.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add an ambiguity report and blocked-state validator only. Do not add module model code or semantics.

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
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Verify tests fail: `not applicable; blocker-state validator should pass when M0014 is correctly blocked`
- Ordinary tests: `docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Adversarial tests: `docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0014-001-module-package-visibility-model-blocker.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0014-module-package-visibility-model-blocked.sh && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0013-expression-statement-pattern-ast-shell.sh && docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0014-module-package-visibility-model-blocked.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/ambiguities/M0014-module-package-visibility-model.md`
  - `docs/tasks/M0014-001-module-package-visibility-model-blocker.md`
  - `docs/milestones/M0014-module-package-and-visibility-model.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement module model code.
- Do not introduce module identity, package mapping, default visibility, or `internal` semantics.
- Do not add name resolution.

## Ambiguities And Dependencies

- M0014 is blocked by `docs/ambiguities/M0014-module-package-visibility-model.md`.
- The next safe task is for main-task semantic design to draft a non-authoritative module, package, namespace, and visibility model proposal.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0014 module/package/visibility model blocker task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0014-module-package-visibility-model-blocked.sh to enforce blocked state.
2026-07-10 main_task=Language-Lawyer phase=implementation result=pass notes=Filed M0014 ambiguity report without adding module model implementation.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=docs/tests/m0014-module-package-visibility-model-blocked.sh passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0014-001-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0014-001-review.md approves blocked scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Focused M0014-M0002 validation command passed.
```

## Handoff

- Next main task: `main-task semantic design`
- Reason: `Draft module, package, namespace, and visibility semantics proposal without accepting it as source of truth.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/ambiguities/M0014-module-package-visibility-model.md`
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
