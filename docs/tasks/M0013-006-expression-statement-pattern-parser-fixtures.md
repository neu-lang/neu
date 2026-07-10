# Task: M0013-006 Create expression statement and pattern parser fixtures

## Task Metadata

- Task ID: `M0013-006`
- Milestone: `M0013`
- Milestone File: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Status: `complete`
- Owner main task: `main-task test work`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0013-006-expression-statement-pattern-parser-fixtures`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add ADR-0024-backed expression, statement, and pattern parser fixture files without implementing parser behavior.

## Motivation

M0013 now has accepted expression, statement, and pattern syntax authority. Parser implementation should be driven by concrete positive, negative, diagnostic, and adversarial fixtures that cite ADR-0024 rather than inferred Kotlin behavior, Rust behavior, or existing compiler behavior.

## Scope

- Add expression parser fixture files under `tests/fixtures/parser/expressions/`.
- Add statement parser fixture files under `tests/fixtures/parser/statements/`.
- Add pattern parser fixture files under `tests/fixtures/parser/patterns/`.
- Cover ADR-0024 literals, names, grouping, operators, calls, member access, `if`, blocks, local declarations, assignment statements, returns, expression statements, and accepted pattern forms.
- Add a documentation validator for the fixture corpus.
- Update the M0013 milestone checklist for completed fixture coverage.

## Out Of Scope

- Parser implementation.
- Rust tests that execute a parser.
- Expression, statement, or pattern AST nodes.
- Type checking, flow typing, exhaustiveness checking, ownership analysis, borrow analysis, coroutine analysis, or unsafe checking.
- `when`, `match`, loop, coroutine, unsafe block, indexing, lambda, receiver, destructuring, or other deferred syntax.
- Changing ADR-0024 expression, statement, or pattern semantics.

## Required Inputs

- Milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Spec sections:
  - `ADR-0024: Expression Statement And Pattern Syntax`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Existing files:
  - `docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Fixture corpus includes valid ADR-0024 expression, statement, block, and pattern examples.
- Negative tests:
  - Fixture corpus includes malformed binary expressions, calls, member access, blocks, variable declarations, assignments, returns, patterns, and unsupported deferred forms.
- Diagnostic tests:
  - Fixture corpus includes every ADR-0024 parser diagnostic category relevant to expressions, statements, blocks, conditionals, and patterns.
- Adversarial tests:
  - Fixture corpus does not encode type checking, ownership, borrow checking, flow typing, exhaustiveness, coroutine, unsafe, match, loop, or backend semantics.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Expression, statement, and pattern parser fixture files do not exist yet.
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

- Generate tests: `create docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
- Verify tests fail: `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
- Ordinary tests: `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh && docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh`
- Adversarial tests: `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0013-006-expression-statement-pattern-parser-fixtures.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh && docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh && docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh && docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `tests/fixtures/parser/expressions/positive.fixture.toml`
  - `tests/fixtures/parser/expressions/negative.fixture.toml`
  - `tests/fixtures/parser/expressions/diagnostics.fixture.toml`
  - `tests/fixtures/parser/statements/positive.fixture.toml`
  - `tests/fixtures/parser/statements/negative.fixture.toml`
  - `tests/fixtures/parser/statements/diagnostics.fixture.toml`
  - `tests/fixtures/parser/patterns/positive.fixture.toml`
  - `tests/fixtures/parser/patterns/negative.fixture.toml`
  - `tests/fixtures/parser/patterns/diagnostics.fixture.toml`
  - `docs/tasks/M0013-006-expression-statement-pattern-parser-fixtures.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`.
- Do not add expression, statement, or pattern parser APIs to `crates/compiler/src/parser.rs`.
- Do not add expression, statement, or pattern AST nodes to `crates/compiler/src/ast.rs`.
- Do not add executable Rust parser tests.

## Ambiguities And Dependencies

- Fixture examples may use only ADR-0024 expression, statement, block, and pattern syntax.
- Pattern fixtures must not imply accepted `when` or `match` syntax.
- `if` fixtures are parser syntax only; value typing and smart-cast behavior are deferred.
- Unsafe block syntax, coroutine syntax, loop syntax, indexing, lambda syntax, and match syntax remain deferred.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0013 expression, statement, and pattern parser fixture task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh before adding fixture files.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh failed before fixtures were added because tests/fixtures/parser/expressions/positive.fixture.toml was missing.
2026-07-10 main_task=main-task test work phase=implementation result=pass notes=Added positive, negative, and diagnostic ADR-0024 expression, statement, and pattern parser fixtures without parser code.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh and docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0013-006-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0013-006-review.md approves ADR-0024 fixture scope and parser implementation deferral.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Full M0013-M0002 validation command passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add ADR-0024 fixture corpus before parser implementation.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
