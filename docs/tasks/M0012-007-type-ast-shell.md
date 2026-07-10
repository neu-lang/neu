# Task: M0012-007 Add type and generic AST shell

## Task Metadata

- Task ID: `M0012-007`
- Milestone: `M0012`
- Milestone File: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0012-007-type-ast-shell`

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

Add syntax-only AST node kinds and arena constructors for ADR-0023 type and generic grammar forms without implementing parser behavior.

## Motivation

The M0012 parser needs AST targets before it can consume the accepted type and generic fixtures. This task keeps the AST surface syntax-only, preserving later separation between parsing, type representation, type checking, generic solving, and capability analysis.

## Scope

- Add AST node kinds for ADR-0023 named type references, nullable types, generic parameters, generic arguments, capability bounds, function types, grouped types, and type syntax diagnostics grouping.
- Add `AstArena` constructors for those node kinds.
- Add focused Rust AST tests proving IDs, kinds, and spans are preserved.
- Add a documentation validator for the AST shell.
- Update stale M0012 validators that previously required type AST nodes to remain absent.

## Out Of Scope

- Parser implementation.
- Parser APIs for type or generic syntax.
- Executable parser tests for type syntax.
- Type representation, type checking, generic constraint solving, or capability analysis.
- Expression, statement, pattern, coroutine, unsafe, or deferred type syntax.
- Changing ADR-0023.

## Required Inputs

- Milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Spec sections:
  - `ADR-0023: Type And Generic Syntax`
- ADRs:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Existing files:
  - `crates/compiler/src/ast.rs`
  - `crates/compiler/tests/ast.rs`
  - `docs/tests/m0012-type-generic-parser-fixtures.sh`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - AST arena can create each ADR-0023 type/generic syntax node and preserve kind, span, and insertion order.
- Negative tests:
  - Parser source still has no type/generic parser APIs.
- Diagnostic tests:
  - AST shell stays syntax-only and does not define diagnostic behavior.
- Adversarial tests:
  - AST node names do not encode type checking, generic solving, capability semantics, ownership, borrowing, coroutine, or unsafe semantics.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0012-type-ast-shell.sh`
  - focused Rust AST test additions in `crates/compiler/tests/ast.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `crates/compiler/src/ast.rs` does not yet define ADR-0023 type/generic AST node kinds or arena constructors.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add syntax-only `AstNodeKind` variants and `AstArena` constructor methods for ADR-0023 forms. Do not attach child relationships, resolved names, semantic type IDs, capability meanings, or parser integration in this task.

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

- Generate tests: `create docs/tests/m0012-type-ast-shell.sh and update crates/compiler/tests/ast.rs`
- Verify tests fail: `docs/tests/m0012-type-ast-shell.sh`
- Ordinary tests: `cargo test --workspace --all-targets ast -- --nocapture && docs/tests/m0012-type-ast-shell.sh`
- Adversarial tests: `docs/tests/m0012-type-ast-shell.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0012-007-type-ast-shell.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0012-type-ast-shell.sh`
  - `crates/compiler/tests/ast.rs`
- Implementation files:
  - `crates/compiler/src/ast.rs`
- Documentation or checklist files:
  - `docs/tasks/M0012-007-type-ast-shell.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
  - existing M0012 validators that formerly rejected type AST nodes

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.
- Do not add type parser APIs to `crates/compiler/src/parser.rs`.
- Do not add executable parser tests for type syntax.
- Do not add semantic type IDs, symbol resolution, constraint solving, or capability meanings.

## Ambiguities And Dependencies

- AST shell nodes represent syntax forms only.
- Child relationships and typed AST structures remain deferred until parser implementation and later frontend representation milestones.
- Capability-bound AST nodes do not define capability semantics.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0012 type AST shell task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0012-type-ast-shell.sh and Rust AST test additions before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0012-type-ast-shell.sh failed before implementation because type AST node kinds were missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added syntax-only ADR-0023 AST node kinds and arena constructors without parser or semantic type changes.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=Focused Rust AST test and docs/tests/m0012-type-ast-shell.sh passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0012-007-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0012-007-review.md approves syntax-only AST shell scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0012-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add syntax-only ADR-0023 AST shell nodes before parser implementation.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
