# Task: M0012-001 Record type and generic syntax parser blocker

## Task Metadata

- Task ID: `M0012-001`
- Milestone: `M0012`
- Milestone File: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Status: `blocked`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0012-001-type-generic-syntax-blocker`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Record that M0012 type and generic syntax parser implementation is blocked until type, nullable, generic, function-type, and capability-bound syntax has accepted source-of-truth authority.

## Motivation

M0012 requires parsing approved type syntax and generic parameter syntax. The grammar authority ledger classifies all M0012 syntax constructs as ambiguous, and `docs/ambiguities/M0008-type-generic-syntax.md` already records the missing authority. Implementing parser behavior now would invent language syntax.

## Scope

- Record the M0012 blocker.
- Add validation that type and generic parser implementation remains absent while ambiguity is open.
- Point to the required type and generic syntax resolution path.
- Preserve completed M0011 parser behavior.

## Out Of Scope

- Type parser implementation.
- Type syntax fixtures.
- Generic syntax fixtures.
- Accepting type or generic grammar.
- Modifying `docs/SPEC.md`.
- Modifying accepted ADRs.
- Adding type AST nodes.

## Required Inputs

- Milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Spec sections:
  - `ADR-0006: Nullability And Absence`
  - `ADR-0010: Type System Shape`
  - `ADR-0016: Generics And Parametric Polymorphism`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Existing files:
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0012-type-generic-parser-blocked.sh` verifies the M0012 blocker is recorded.
- Negative tests:
  - Validation fails if type parser code, type AST nodes, or type/generic fixtures are introduced while the ambiguity is open.
- Diagnostic tests:
  - not applicable
- Adversarial tests:
  - Confirm no concrete type, nullable, generic, function-type, or capability-bound syntax is accepted.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0012-type-generic-parser-blocked.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - M0012 blocker task is not yet recorded as blocked.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add blocker validation only. Do not implement parser code or create syntax fixtures.

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

- Generate tests: `create docs/tests/m0012-type-generic-parser-blocked.sh`
- Verify tests fail: `docs/tests/m0012-type-generic-parser-blocked.sh`
- Ordinary tests: `docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0008-grammar-authority-ledger.sh`
- Adversarial tests: `docs/tests/m0012-type-generic-parser-blocked.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0012-001-type-generic-syntax-blocker.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0012-type-generic-parser-blocked.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0012-001-type-generic-syntax-blocker.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.
- Do not add type AST nodes.
- Do not add type or generic syntax fixtures.
- Do not add `parse_type`, `parse_generic`, or capability-bound parser behavior.

## Ambiguities And Dependencies

- Blocking ambiguity: `docs/ambiguities/M0008-type-generic-syntax.md`
- Required next step: main-task semantic design drafts type and generic syntax ADR or `docs/SPEC.md` revision.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0012 type and generic syntax blocker task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created M0012 blocker validator.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0012-type-generic-parser-blocked.sh failed before blocker status was recorded as blocked.
2026-07-10 main_task=Language-Lawyer phase=implementation result=pass notes=Recorded existing M0008 type/generic syntax ambiguity as blocking M0012 parser implementation.
2026-07-10 main_task=Language-Lawyer phase=ordinary-tests result=pass notes=docs/tests/m0012-type-generic-parser-blocked.sh passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0012-001-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=blocked notes=docs/tasks/reviews/M0012-001-review.md blocks implementation pending type/generic syntax ambiguity resolution.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0012-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task semantic design`
- Reason: `Draft type and generic syntax source-of-truth proposal before M0012 parser implementation.`
- Required Context:
  - This task file
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
