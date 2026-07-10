# Task: M0011-008 Implement ADR-0022 declaration parser

## Task Metadata

- Task ID: `M0011-008`
- Milestone: `M0011`
- Milestone File: `docs/milestones/M0011-declaration-parser.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0011-008-<slug>`

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

Implement the minimal ADR-0022 declaration parser over the existing lexer and declaration AST shell.

## Motivation

M0011 requires declaration fixtures to parse to AST and invalid declarations to produce diagnostics with spans. ADR-0022, parser fixtures, and declaration AST shell now exist, so parser implementation can proceed without inventing syntax.

## Scope

- Add `crates/compiler/src/parser.rs`.
- Export the parser module.
- Parse ADR-0022 package declarations, import declarations, visibility modifiers, function declarations, struct declarations, enum declarations, interface declarations, and declaration bodies.
- Emit ADR-0022 declaration diagnostics with primary spans.
- Add executable Rust parser tests.
- Add a validator for parser implementation scope.

## Out Of Scope

- Type, generic, expression, statement, pattern, coroutine, or unsafe-block parsing.
- Name resolution, symbols, type checking, ownership, borrowing, HIR, MIR, backend, or lowering.
- Full AST child relationship modeling.
- Parsing parameter declarations or concrete return types.

## Required Inputs

- Milestone: `docs/milestones/M0011-declaration-parser.md`
- Spec sections:
  - `ADR-0022: Declaration Syntax`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Existing files:
  - `crates/compiler/src/lexer.rs`
  - `crates/compiler/src/ast.rs`
  - `tests/fixtures/parser/declarations/*.fixture.toml`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Valid ADR-0022 declaration snippets produce AST declaration node kinds without parser diagnostics.
- Negative tests:
  - Deferred or invalid declaration snippets emit ADR-0022 parser diagnostics.
- Diagnostic tests:
  - Diagnostics expose kind and primary span.
- Adversarial tests:
  - Parser implementation does not parse deferred type, expression, statement, or pattern grammar.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0011-declaration-parser-implementation.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `compiler::parser` does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a small token-stream parser that builds flat AST declaration shell nodes and records declaration diagnostics. Keep type placeholders and function bodies opaque according to ADR-0022.

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

- Generate tests: `create crates/compiler/tests/parser.rs and docs/tests/m0011-declaration-parser-implementation.sh`
- Verify tests fail: `cargo test --workspace --all-targets parser`
- Ordinary tests: `cargo test --workspace --all-targets parser && docs/tests/m0011-declaration-parser-implementation.sh`
- Adversarial tests: `docs/tests/m0011-declaration-parser-implementation.sh`
- Review: `docs/scripts/review-task.sh <task-file>`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0011-declaration-syntax-concrete-draft.sh && docs/tests/m0011-declaration-syntax-review.sh && docs/tests/m0011-declaration-syntax-proposal.sh && docs/tests/m0011-declaration-parser-blocked.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0011-declaration-parser-implementation.sh`
- Implementation files:
  - `crates/compiler/src/parser.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/ast.rs`
- Documentation or checklist files:
  - `docs/tasks/M0011-008-declaration-parser-implementation.md`
  - `docs/milestones/M0011-declaration-parser.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.
- Do not add HIR, MIR, name resolution, type checking, ownership, or lowering behavior.

## Ambiguities And Dependencies

- ADR-0022 allows return type placeholders but does not define concrete type grammar.
- ADR-0022 allows declaration bodies and semicolon function placeholders but does not define statements or expressions.
- Parser output remains a flat AST shell until child relationships are specified.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=<main task> phase=<phase> result=<result> notes=<notes>
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0011 declaration parser implementation task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Added crates/compiler/tests/parser.rs and docs/tests/m0011-declaration-parser-implementation.sh before parser implementation.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test --workspace --all-targets parser failed because compiler::parser was missing.
2026-07-09 main_task=main-task implementation phase=implementation result=pass notes=Added flat ADR-0022 parser over lexer tokens with parser diagnostics and AST declaration shell output.
2026-07-09 main_task=main-task implementation phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets passed and parser implementation validator reached only task-completion guard before status update.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0011-008-soundness.md after ordinary-tests evidence.
2026-07-09 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0011-008-review.md approves pending final CI gate.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0011-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add failing parser tests before implementation.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - Relevant ADRs
  - Milestone file
