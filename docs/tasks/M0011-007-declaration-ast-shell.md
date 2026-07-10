# Task: M0011-007 Add ADR-0022 declaration AST shell

## Task Metadata

- Task ID: `M0011-007`
- Milestone: `M0011`
- Milestone File: `docs/milestones/M0011-declaration-parser.md`
- Status: `complete`
- Owner main task: `main-task test work`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0011-007-<slug>`

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

Add a syntax-level AST declaration shell for ADR-0022 declaration forms without implementing parser behavior or semantic analysis.

## Motivation

The declaration parser needs stable AST node kinds for package, import, function, struct, enum, interface, and placeholder declaration bodies. Adding these nodes before parser implementation keeps parser work focused on token consumption and recovery.

## Scope

- Add syntax-level AST node kinds for ADR-0022 declaration shells.
- Add AST arena constructors for declaration nodes that preserve spans.
- Add Rust tests for node identity, kind, and span preservation.
- Add a validation script for the declaration AST shell.

## Out Of Scope

- Parser implementation.
- Parser executable tests.
- Name resolution, symbols, type checking, ownership, borrowing, or semantic lowering.
- Type, generic, expression, statement, or pattern AST nodes.
- Storing declaration names or child relationships; this task adds only node kinds and spans.

## Required Inputs

- Milestone: `docs/milestones/M0011-declaration-parser.md`
- Spec sections:
  - `ADR-0022: Declaration Syntax`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Existing files:
  - `crates/compiler/src/ast.rs`
  - `crates/compiler/tests/ast.rs`
  - `docs/ast/data-model.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Rust AST tests can create ADR-0022 declaration shell nodes and verify kind and span preservation.
- Negative tests:
  - Validator rejects parser implementation, HIR, MIR, semantic symbol, and type/expression AST additions.
- Diagnostic tests:
  - not applicable
- Adversarial tests:
  - Confirm declaration AST shell does not encode semantic analysis or deferred grammar.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0011-declaration-ast-shell.sh`
  - `crates/compiler/tests/ast.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `AstNodeKind` does not include ADR-0022 declaration node kinds and `AstArena` has no declaration constructors.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend the existing syntax-independent AST shell with declaration node kinds and arena constructors only. Preserve the flat arena shape from M0009.

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

- Generate tests: `create docs/tests/m0011-declaration-ast-shell.sh and update crates/compiler/tests/ast.rs`
- Verify tests fail: `cargo test --workspace --all-targets ast && docs/tests/m0011-declaration-ast-shell.sh`
- Ordinary tests: `cargo test --workspace --all-targets ast && docs/tests/m0011-declaration-ast-shell.sh`
- Adversarial tests: `docs/tests/m0011-declaration-ast-shell.sh`
- Review: `docs/scripts/review-task.sh <task-file>`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0011-declaration-syntax-concrete-draft.sh && docs/tests/m0011-declaration-syntax-review.sh && docs/tests/m0011-declaration-syntax-proposal.sh && docs/tests/m0011-declaration-parser-blocked.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0011-declaration-ast-shell.sh`
  - `crates/compiler/tests/ast.rs`
- Implementation files:
  - `crates/compiler/src/ast.rs`
- Documentation or checklist files:
  - `docs/ast/data-model.md`
  - `docs/tasks/M0011-007-declaration-ast-shell.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.
- Do not add `crates/compiler/src/parser.rs`.
- Do not add HIR, MIR, symbol, type, expression, statement, or pattern AST nodes.

## Ambiguities And Dependencies

- ADR-0022 defines declaration shells, not child layout, names, parameters, type syntax, or statement bodies.
- This task intentionally keeps child relationships and semantic payloads deferred to parser and later AST tasks.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=<main task> phase=<phase> result=<result> notes=<notes>
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0011 declaration AST shell task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Added failing AST tests and docs/tests/m0011-declaration-ast-shell.sh before AST implementation.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test --workspace --all-targets ast failed because AstNodeKind declaration variants and AstArena constructors were missing.
2026-07-09 main_task=main-task implementation phase=implementation result=pass notes=Added ADR-0022 declaration node kinds and span-preserving AstArena constructors only.
2026-07-09 main_task=main-task implementation phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets ast passed and docs/tests/m0011-declaration-ast-shell.sh reached only task-completion guard before status update.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0011-007-soundness.md after ordinary-tests evidence.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0011-M0002 validation scripts passed.
2026-07-09 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0011-007-review.md approves AST shell scope and parser implementation deferral.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add failing AST shell tests before implementation.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - Relevant ADRs
  - Milestone file
