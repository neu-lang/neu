# Task: M0012-006 Create type and generic parser fixtures

## Task Metadata

- Task ID: `M0012-006`
- Milestone: `M0012`
- Milestone File: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Status: `complete`
- Owner main task: `main-task test work`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0012-006-type-generic-parser-fixtures`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add ADR-0023-backed type and generic parser fixture files without implementing parser behavior.

## Motivation

M0012 now has accepted type and generic syntax authority. The parser implementation should be driven by concrete positive, negative, diagnostic, and adversarial fixtures that cite ADR-0023 rather than draft proposal text, inferred Kotlin behavior, or existing compiler behavior.

## Scope

- Add parser type fixture files under `tests/fixtures/parser/types/`.
- Add parser generic fixture files under `tests/fixtures/parser/generics/`.
- Cover named type references, nullable type syntax, generic parameter syntax, generic argument syntax, capability-bound syntax, function type syntax, grouping, recovery boundaries, and ADR-0023 diagnostic categories.
- Add a documentation validator for the fixture corpus.
- Update earlier M0012 validators so type/generic fixture paths are no longer treated as out of scope once fixture creation begins.

## Out Of Scope

- Parser implementation.
- Rust tests that execute a parser.
- Type AST nodes.
- Type checking, generic constraint solving, or capability analysis.
- Expression, statement, pattern, coroutine, unsafe, or deferred type syntax.
- Changing ADR-0023 type or generic semantics.

## Required Inputs

- Milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Spec sections:
  - `ADR-0023: Type And Generic Syntax`
- ADRs:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Existing files:
  - `docs/tests/m0012-type-generic-syntax-accepted.sh`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Fixture corpus includes valid ADR-0023 named, nullable, generic, capability-bound, function, and grouped type examples.
- Negative tests:
  - Fixture corpus includes malformed nullable markers, empty generic lists, comma-separated bounds, malformed function types, and unsupported type forms.
- Diagnostic tests:
  - Fixture corpus includes every ADR-0023 type syntax diagnostic category and recovery boundary family.
- Adversarial tests:
  - Fixture corpus does not encode expression, statement, pattern, coroutine, unsafe, type-checking, generic solving, or capability semantics.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0012-type-generic-parser-fixtures.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Type and generic parser fixture files do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add fixture metadata and examples only. Do not add parser source, AST nodes, parser APIs, executable parser tests, or semantic analysis.

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

- Generate tests: `create docs/tests/m0012-type-generic-parser-fixtures.sh`
- Verify tests fail: `docs/tests/m0012-type-generic-parser-fixtures.sh`
- Ordinary tests: `docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh`
- Adversarial tests: `docs/tests/m0012-type-generic-parser-fixtures.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0012-006-type-generic-parser-fixtures.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0012-type-generic-parser-fixtures.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `tests/fixtures/parser/types/positive.fixture.toml`
  - `tests/fixtures/parser/types/negative.fixture.toml`
  - `tests/fixtures/parser/types/diagnostics.fixture.toml`
  - `tests/fixtures/parser/generics/positive.fixture.toml`
  - `tests/fixtures/parser/generics/negative.fixture.toml`
  - `tests/fixtures/parser/generics/diagnostics.fixture.toml`
  - `docs/tasks/M0012-006-type-generic-parser-fixtures.md`
  - existing M0012 validators that formerly rejected type/generic parser fixture paths

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.
- Do not add type parser APIs to `crates/newlang/src/parser.rs`.
- Do not add type AST nodes to `crates/newlang/src/ast.rs`.
- Do not add executable Rust parser tests.

## Ambiguities And Dependencies

- Fixture examples may use only ADR-0023 type and generic syntax.
- Capability-bound fixture examples must not assign semantic meaning to capability names.
- Expression, statement, pattern, coroutine, unsafe, and deferred type syntax remain out of scope.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0012 type and generic parser fixture task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0012-type-generic-parser-fixtures.sh before adding fixture files.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0012-type-generic-parser-fixtures.sh failed before fixtures were added because tests/fixtures/parser/types/positive.fixture.toml was missing.
2026-07-10 main_task=main-task test work phase=implementation result=pass notes=Added positive, negative, and diagnostic ADR-0023 type and generic parser fixtures without parser code.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=docs/tests/m0012-type-generic-parser-fixtures.sh plus M0012 source-of-truth guards passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0012-006-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0012-006-review.md approves ADR-0023 fixture scope and parser implementation deferral.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0012-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add ADR-0023 fixture corpus before parser implementation.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
