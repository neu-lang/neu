# Task: M0012-008 Implement type and generic parser

## Task Metadata

- Task ID: `M0012-008`
- Milestone: `M0012`
- Milestone File: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0012-008-type-generic-parser-implementation`

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

Implement parser support for ADR-0023 type and generic syntax in declaration return types and declaration generic parameter lists.

## Motivation

M0012 has accepted syntax authority, fixtures, and AST shell nodes. The next parser increment should consume ADR-0023 type and generic syntax and produce source-spanned AST syntax nodes plus diagnostics for malformed type syntax.

## Scope

- Parse ADR-0023 return type syntax after `:` in function declarations.
- Parse ADR-0023 generic parameter lists after function, struct, enum, and interface names.
- Parse named types, qualified named types, generic arguments, nullable types, function types, grouped types, and capability bounds.
- Emit ADR-0023 type syntax diagnostic kinds with source spans.
- Preserve existing M0011 declaration parser behavior.

## Out Of Scope

- Type parsing in function parameter lists.
- Field, property, constructor, enum variant, expression, statement, pattern, coroutine, or unsafe syntax.
- Type checking, generic constraint solving, name resolution, or capability semantics.
- HIR, MIR, backend, or optimization changes.
- Changing ADR-0023.

## Required Inputs

- Milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Spec sections:
  - `ADR-0023: Type And Generic Syntax`
- ADRs:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Existing files:
  - `crates/compiler/src/parser.rs`
  - `crates/compiler/src/ast.rs`
  - `crates/compiler/tests/parser.rs`
  - `tests/fixtures/parser/types/*.fixture.toml`
  - `tests/fixtures/parser/generics/*.fixture.toml`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Declarations with ADR-0023 return types and generic parameter lists parse to AST nodes.
- Negative tests:
  - Repeated nullable markers, empty generic lists, comma-separated bounds, and malformed function types produce diagnostics.
- Diagnostic tests:
  - Parser exposes ADR-0023 diagnostic kinds for type syntax failures.
- Adversarial tests:
  - Parser still rejects field, property, expression, statement, pattern, coroutine, unsafe, and deferred type syntax.

## Test-First Gate

- Test files to create before implementation:
  - focused Rust parser tests in `crates/compiler/tests/parser.rs`
  - `docs/tests/m0012-type-generic-parser-implementation.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Parser does not yet produce ADR-0023 type/generic AST nodes or diagnostic kinds.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend the existing parser with the smallest recursive-descent type parser needed by ADR-0023. Integrate it only at accepted declaration return type and generic parameter-list positions. Keep AST nodes syntax-only and preserve existing recovery behavior.

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

- Generate tests: `update crates/compiler/tests/parser.rs and create docs/tests/m0012-type-generic-parser-implementation.sh`
- Verify tests fail: `cargo test --workspace --all-targets parses_type_and_generic_syntax -- --nocapture`
- Ordinary tests: `cargo test --workspace --all-targets parser -- --nocapture && docs/tests/m0012-type-generic-parser-implementation.sh`
- Adversarial tests: `docs/tests/m0012-type-generic-parser-implementation.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0012-008-type-generic-parser-implementation.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0012-type-generic-parser-implementation.sh`
- Implementation files:
  - `crates/compiler/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0012-008-type-generic-parser-implementation.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
  - existing validators updated for parser implementation no longer being deferred

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.
- Do not implement type checking, name resolution, constraint solving, or capability semantics.
- Do not parse expressions, statements, patterns, coroutine syntax, unsafe syntax, fields, properties, constructors, or enum variants.

## Ambiguities And Dependencies

- Function parameter contents remain placeholders from ADR-0022; this task does not parse parameter type syntax.
- Capability-bound names are parsed syntactically only.
- Deferred type forms in ADR-0023 remain unsupported and diagnostic-producing.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0012 type/generic parser implementation task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added parser implementation tests and docs validator before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=Focused parser tests failed before implementation because ADR-0023 AST nodes and diagnostics were not produced by parser.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Implemented ADR-0023 type and generic parser support for declaration generic lists and function return types.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=Parser Rust tests and docs/tests/m0012-type-generic-parser-implementation.sh passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0012-008-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0012-008-review.md approves ADR-0023 parser implementation scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0012-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Implement ADR-0023 parser support after tests fail.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
