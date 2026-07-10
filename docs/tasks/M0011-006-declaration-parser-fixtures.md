# Task: M0011-006 Add ADR-0022 declaration parser fixtures

## Task Metadata

- Task ID: `M0011-006`
- Milestone: `M0011`
- Milestone File: `docs/milestones/M0011-declaration-parser.md`
- Status: `complete`
- Owner main task: `main-task test work`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0011-006-<slug>`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Project Rules: `main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add ADR-0022-backed declaration parser fixture files without implementing parser behavior.

## Motivation

M0011 now has accepted declaration syntax authority. Parser implementation should be driven by concrete positive, negative, and diagnostic fixtures that cite ADR-0022 rather than draft proposal text or inferred Kotlin behavior.

## Scope

- Add parser declaration fixture files under `tests/fixtures/parser/declarations/`.
- Cover package and import order, visibility modifiers, function declarations, struct declarations, enum declarations, interface declarations, nested declaration bodies, and ADR-0022 diagnostic categories.
- Add a documentation validator for the fixture corpus.
- Update earlier M0011 validators so parser fixtures are no longer treated as out of scope.

## Out Of Scope

- Parser implementation.
- Rust tests that execute a parser.
- AST declaration node expansion.
- Type, generic, expression, statement, or pattern parsing.
- Changing ADR-0022 declaration semantics.

## Required Inputs

- Milestone: `docs/milestones/M0011-declaration-parser.md`
- Spec sections:
  - `ADR-0022: Declaration Syntax`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Existing files:
  - `docs/tests/m0011-declaration-syntax-accepted.sh`
  - `docs/milestones/M0011-declaration-parser.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Fixture corpus includes valid package/import/declaration examples accepted by ADR-0022.
- Negative tests:
  - Fixture corpus includes deferred or invalid syntax examples that must not be accepted as declarations.
- Diagnostic tests:
  - Fixture corpus includes expected ADR-0022 declaration diagnostic categories and recovery names.
- Adversarial tests:
  - Fixture corpus does not encode type, generic, expression, statement, or pattern grammar.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0011-declaration-parser-fixtures.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Declaration parser fixture files do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add fixture metadata and examples only. Do not add parser source, parser APIs, or executable parser tests until a later implementation task.

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

Commands may be `blocked: <reason>` until the project has the relevant harness.

- Generate tests: `create docs/tests/m0011-declaration-parser-fixtures.sh`
- Verify tests fail: `docs/tests/m0011-declaration-parser-fixtures.sh`
- Ordinary tests: `docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh`
- Adversarial tests: `docs/tests/m0011-declaration-parser-fixtures.sh`
- Review: `docs/scripts/review-task.sh <task-file>`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0011-declaration-syntax-concrete-draft.sh && docs/tests/m0011-declaration-syntax-review.sh && docs/tests/m0011-declaration-syntax-proposal.sh && docs/tests/m0011-declaration-parser-blocked.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0011-declaration-parser-fixtures.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `tests/fixtures/parser/declarations/positive.fixture.toml`
  - `tests/fixtures/parser/declarations/negative.fixture.toml`
  - `tests/fixtures/parser/declarations/diagnostics.fixture.toml`
  - `docs/tasks/M0011-006-declaration-parser-fixtures.md`
  - existing M0011 validators that formerly rejected parser fixture paths

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.
- Do not add `crates/compiler/src/parser.rs`.
- Do not add executable Rust parser tests.

## Ambiguities And Dependencies

- Fixture examples may use only ADR-0022 declaration shells.
- Type placeholders must remain opaque and may not define type grammar.
- Function bodies may contain only declaration-body or semicolon placeholders.
- Expression, statement, pattern, coroutine, and unsafe syntax remain out of scope.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=<main task> phase=<phase> result=<result> notes=<notes>
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0011 declaration parser fixture task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0011-declaration-parser-fixtures.sh before adding fixture files.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0011-declaration-parser-fixtures.sh failed before fixtures were added because positive.fixture.toml was missing.
2026-07-09 main_task=main-task test work phase=implementation result=pass notes=Added positive, negative, and diagnostic ADR-0022 declaration parser fixtures without parser code.
2026-07-09 main_task=main-task test work phase=ordinary-tests result=pass notes=docs/tests/m0011-declaration-parser-fixtures.sh plus M0011 source-of-truth guards passed.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0011-006-soundness.md after ordinary-tests evidence.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0011-M0002 validation scripts passed.
2026-07-09 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0011-006-review.md approves ADR-0022 fixture scope and parser implementation deferral.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add ADR-0022 fixture corpus before parser implementation.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - Relevant ADRs
  - Milestone file
