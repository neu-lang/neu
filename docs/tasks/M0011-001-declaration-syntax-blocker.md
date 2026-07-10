# Task: M0011-001 Record Declaration Parser Syntax Blocker

## Task Metadata

- Task ID: `M0011-001`
- Milestone: `M0011`
- Milestone File: `docs/milestones/M0011-declaration-parser.md`
- Status: `blocked`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0011-001-declaration-syntax-blocker`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Record that M0011 declaration parser implementation is blocked until declaration syntax has accepted source-of-truth authority.

## Motivation

M0011 requires parsing approved module-level and declaration syntax. M0008 explicitly classifies package declarations, imports, visibility syntax, function declarations, struct declarations, enum or sealed sum declarations, and interface declarations as ambiguous.

## Scope

- Record the M0011 blocker.
- Add validation that M0011 parser implementation remains absent while declaration syntax ambiguity is open.
- Point to the required declaration syntax resolution path.

## Out Of Scope

- Parser implementation.
- Parser fixtures.
- Accepting declaration grammar.
- Modifying `docs/SPEC.md`.
- Modifying accepted ADRs.
- Adding concrete declaration AST nodes.

## Required Inputs

- Milestone: `docs/milestones/M0011-declaration-parser.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
- Existing files:
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/ambiguities/M0008-declaration-syntax.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0011-declaration-parser-blocked.sh` verifies the declaration parser blocker is recorded.
- Negative tests:
  - The validation fails if parser code or parser fixtures are introduced while the ambiguity is open.
- Diagnostic tests:
  - Not applicable.
- Adversarial tests:
  - Confirm no declaration parser code exists.
  - Confirm no concrete declaration fixtures exist.
  - Confirm the declaration syntax ambiguity remains open.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0011-declaration-parser-blocked.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - M0011 blocker task is not yet recorded as blocked.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add blocker validation only. Do not implement parser code.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing blocker artifact.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone checklist is not marked complete because M0011 remains blocked.

## Execution Commands

- Generate tests: `create docs/tests/m0011-declaration-parser-blocked.sh`
- Verify tests fail: `docs/tests/m0011-declaration-parser-blocked.sh`
- Ordinary tests: `docs/tests/m0011-declaration-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0011-declaration-parser-blocked.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0011-001-declaration-syntax-blocker.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0011-declaration-parser-blocked.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0011-declaration-parser-blocked.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0011-001-declaration-syntax-blocker.md`

## Forbidden Changes

- Do not add `crates/compiler/src/parser.rs`.
- Do not add concrete parser fixtures.
- Do not add concrete declaration AST nodes.
- Do not change declaration syntax authority.

## Ambiguities And Dependencies

- Blocking ambiguity: `docs/ambiguities/M0008-declaration-syntax.md`
- Required next step: main-task semantic design drafts declaration syntax ADR or `docs/SPEC.md` revision.

## Execution Log

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0011 declaration syntax blocker task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created M0011 blocker validator.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation would fail before blocker task status was recorded as blocked.
2026-07-09 main_task=Language-Lawyer phase=ordinary-tests result=pass notes=M0011 blocker validation passed.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms no parser code or concrete declaration fixtures were added.
2026-07-09 main_task=main-task review phase=review result=pass notes=Blocker is valid against M0011 and M0008 declaration syntax ambiguity.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next main task: `main-task semantic design`
- Reason: `Draft declaration syntax source-of-truth proposal before M0011 parser implementation.`
- Required Context:
  - This task file
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/milestones/M0011-declaration-parser.md`
