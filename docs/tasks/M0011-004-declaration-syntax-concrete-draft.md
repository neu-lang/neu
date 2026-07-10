# Task: M0011-004 Revise Declaration Syntax Proposal With Concrete Grammar

## Task Metadata

- Task ID: `M0011-004`
- Milestone: `M0011`
- Milestone File: `docs/milestones/M0011-declaration-parser.md`
- Status: `complete`
- Owner main task: `main-task semantic design`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0011-004-declaration-syntax-concrete-draft`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Revise draft ADR-0022 so it contains concrete declaration grammar and diagnostic decisions required for a future main task acceptance decision.

## Motivation

ADR-0022 reviews requested revisions before acceptance. The current proposal identifies required content but does not define concrete grammar. This task fills in that draft content while keeping ADR-0022 non-authoritative.

## Scope

- Revise `docs/adr/proposals/ADR-0022-declaration-syntax.md` with concrete draft declaration grammar.
- Include package/import ordering, visibility rules, declaration headers, member rules or deferrals, recovery boundaries, and diagnostics.
- Keep ADR-0022 draft-only.
- Keep declaration syntax ambiguity open.
- Keep parser implementation blocked.

## Out Of Scope

- Accepting ADR-0022.
- Modifying `docs/SPEC.md`.
- Moving ADR-0022 into accepted `docs/adr/`.
- Closing `docs/ambiguities/M0008-declaration-syntax.md`.
- Implementing parser code.
- Adding parser fixtures.
- Adding declaration AST nodes.

## Required Inputs

- Milestone: `docs/milestones/M0011-declaration-parser.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Existing files:
  - `docs/adr/proposals/ADR-0022-declaration-syntax.md`
  - `docs/adr/proposals/reviews/ADR-0022-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0022-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0022-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0022-chief-architect-decision.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0011-declaration-syntax-concrete-draft.sh` verifies concrete draft grammar sections exist.
- Negative tests:
  - The validation script must fail before ADR-0022 is revised.
- Diagnostic tests:
  - Draft must define declaration diagnostic categories, primary spans, and recovery actions.
- Adversarial tests:
  - Confirm ADR-0022 remains non-authoritative.
  - Confirm declaration syntax ambiguity remains open.
  - Confirm no parser code or parser fixtures are added.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0011-declaration-syntax-concrete-draft.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0022 lacks concrete draft grammar sections.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Patch ADR-0022 only. Do not update accepted source of truth or parser implementation.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing proposal revision.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0011 milestone checklist is not marked complete because parser implementation remains blocked.

## Execution Commands

- Generate tests: `create docs/tests/m0011-declaration-syntax-concrete-draft.sh`
- Verify tests fail: `docs/tests/m0011-declaration-syntax-concrete-draft.sh`
- Ordinary tests: `docs/tests/m0011-declaration-syntax-concrete-draft.sh && docs/tests/m0011-declaration-syntax-review.sh`
- Adversarial tests: `docs/tests/m0011-declaration-syntax-concrete-draft.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0011-004-declaration-syntax-concrete-draft.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0011-declaration-syntax-concrete-draft.sh && docs/tests/m0011-declaration-syntax-review.sh && docs/tests/m0011-declaration-syntax-proposal.sh && docs/tests/m0011-declaration-parser-blocked.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0011-declaration-syntax-concrete-draft.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0011-004-declaration-syntax-concrete-draft.md`
  - `docs/adr/proposals/ADR-0022-declaration-syntax.md`

## Forbidden Changes

- Do not accept ADR-0022.
- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADR files.
- Do not close declaration syntax ambiguity.
- Do not implement parser code.
- Do not add parser fixtures.
- Do not add declaration AST nodes.

## Ambiguities And Dependencies

- Blocking ambiguity remains `docs/ambiguities/M0008-declaration-syntax.md`.
- Revised draft must be reviewed before acceptance.

## Execution Log

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created ADR-0022 concrete draft revision task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created concrete draft validator before revising ADR-0022.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation failed as expected because ADR-0022 lacked concrete draft grammar sections.
2026-07-09 main_task=Language-Designer phase=ordinary-tests result=pass notes=Concrete draft validation passed after revising ADR-0022 with grammar and diagnostics.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms concrete draft remains non-authoritative and no parser code or fixtures were added.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved concrete draft revision as scoped blocker-resolution progress.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next main task: `main-task semantic design`
- Reason: `Revise ADR-0022 with concrete draft grammar while keeping it non-authoritative.`
- Required Context:
  - This task file
  - `docs/adr/proposals/ADR-0022-declaration-syntax.md`
  - `docs/adr/proposals/reviews/ADR-0022-*.md`
