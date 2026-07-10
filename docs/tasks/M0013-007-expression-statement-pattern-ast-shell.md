# Task: M0013-007 Add expression statement and pattern AST shell

## Task Metadata

- Task ID: `M0013-007`
- Milestone: `M0013`
- Milestone File: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0013-007-expression-statement-pattern-ast-shell`

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

Add syntax-only AST node kinds and arena constructors for ADR-0024 expression, statement, block, and pattern grammar forms without implementing parser behavior.

## Motivation

The M0013 parser needs AST targets before it can consume the accepted body fixtures. This task keeps the AST surface syntax-only, preserving later separation between parsing, semantic typing, flow analysis, ownership analysis, borrowing analysis, and lowering.

## Scope

- Add AST node kinds for ADR-0024 expression syntax forms.
- Add AST node kinds for ADR-0024 statement and block syntax forms.
- Add AST node kinds for ADR-0024 pattern syntax forms.
- Add `AstArena` constructors for those node kinds.
- Add focused Rust AST tests proving IDs, kinds, and spans are preserved.
- Add a documentation validator for the AST shell.
- Update stale AST validators that previously required body AST nodes to remain absent.

## Out Of Scope

- Parser implementation.
- Parser APIs for expression, statement, block, or pattern syntax.
- Executable parser tests for body syntax.
- Type checking, flow typing, exhaustiveness checking, ownership analysis, borrowing analysis, coroutine analysis, unsafe analysis, name resolution, or lowering.
- Child relationships, resolved names, binding modes, value categories, move/copy behavior, or diagnostic emission.
- `when`, `match`, loop, coroutine, unsafe block, indexing, lambda, receiver, destructuring, or other deferred syntax.
- Changing ADR-0024.

## Required Inputs

- Milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Spec sections:
  - `ADR-0024: Expression Statement And Pattern Syntax`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Existing files:
  - `crates/newlang/src/ast.rs`
  - `crates/newlang/tests/ast.rs`
  - `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - AST arena can create each ADR-0024 expression, statement, block, and pattern syntax node and preserve kind, span, and insertion order.
- Negative tests:
  - Parser source still has no body parser APIs.
- Diagnostic tests:
  - AST shell stays syntax-only and does not define diagnostic behavior.
- Adversarial tests:
  - AST node names do not encode type checking, flow typing, ownership semantics, borrowing semantics, coroutine semantics, unsafe semantics, exhaustiveness, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0013-expression-statement-pattern-ast-shell.sh`
  - focused Rust AST test additions in `crates/newlang/tests/ast.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `crates/newlang/src/ast.rs` does not yet define ADR-0024 body AST node kinds or arena constructors.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add syntax-only `AstNodeKind` variants and `AstArena` constructor methods for ADR-0024 forms. Do not attach child relationships, expression values, binding modes, resolved names, semantic type IDs, ownership state, or parser integration in this task.

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

- Generate tests: `create docs/tests/m0013-expression-statement-pattern-ast-shell.sh and update crates/newlang/tests/ast.rs`
- Verify tests fail: `docs/tests/m0013-expression-statement-pattern-ast-shell.sh`
- Ordinary tests: `cargo test --workspace --all-targets ast -- --nocapture && docs/tests/m0013-expression-statement-pattern-ast-shell.sh`
- Adversarial tests: `docs/tests/m0013-expression-statement-pattern-ast-shell.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0013-007-expression-statement-pattern-ast-shell.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0013-expression-statement-pattern-ast-shell.sh && docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh && docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh && docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh && docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0013-expression-statement-pattern-ast-shell.sh`
  - `crates/newlang/tests/ast.rs`
- Implementation files:
  - `crates/newlang/src/ast.rs`
- Documentation or checklist files:
  - `docs/ast/data-model.md`
  - `docs/tasks/M0013-007-expression-statement-pattern-ast-shell.md`
  - existing validators that formerly rejected body AST nodes

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`.
- Do not add expression, statement, block, or pattern parser APIs to `crates/newlang/src/parser.rs`.
- Do not add executable parser tests for body syntax.
- Do not add semantic type IDs, symbols, binding modes, ownership state, borrow state, flow facts, HIR, MIR, or backend behavior.

## Ambiguities And Dependencies

- AST shell nodes represent syntax forms only.
- Child relationships and typed AST structures remain deferred until parser implementation and later frontend representation milestones.
- Pattern nodes do not define binding modes, move behavior, borrowing behavior, smart casts, or exhaustiveness.
- `if` nodes do not define value typing or flow typing.
- Unsafe block syntax, coroutine syntax, loop syntax, indexing, lambda syntax, and match syntax remain deferred.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0013 body AST shell task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0013-expression-statement-pattern-ast-shell.sh and Rust AST test additions before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0013-expression-statement-pattern-ast-shell.sh failed before implementation because body AST node kinds were missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added syntax-only ADR-0024 AST node kinds and arena constructors without parser or semantic changes.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets ast -- --nocapture passed; M0013 AST-shell and affected historical validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0013-007-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0013-007-review.md approves syntax-only AST shell scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Full M0013-M0002 validation command passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add syntax-only ADR-0024 AST shell nodes before parser implementation.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
