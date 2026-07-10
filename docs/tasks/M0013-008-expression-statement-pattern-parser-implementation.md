# Task: M0013-008 Implement expression statement and pattern parser

## Task Metadata

- Task ID: `M0013-008`
- Milestone: `M0013`
- Milestone File: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0013-008-expression-statement-pattern-parser-implementation`

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
  - `main task rules`

## Goal

Implement parser support for ADR-0024 body blocks, statements, expressions, and reusable pattern syntax without adding semantic analysis.

## Motivation

M0013 has accepted syntax, fixtures, and AST shell nodes. The compiler now needs parser behavior that turns executable function bodies into syntax-only AST nodes and source-spanned diagnostics.

## Scope

- Parse function block bodies using ADR-0024 body grammar.
- Parse `val` and `var` local declaration statements.
- Parse assignment, return, and expression statements.
- Parse trailing block expressions.
- Parse expression literals, names, grouping, unary operators, binary operators by ADR-0024 precedence, calls, member access, and `if` expressions.
- Add parser diagnostics for ADR-0024 expression, statement, block, conditional, unsafe, and coroutine/deferred forms.
- Add reusable pattern parser support for ADR-0024 pattern forms without accepting `when` or `match` syntax.
- Add focused Rust parser tests and a documentation validator.

## Out Of Scope

- Type checking.
- Flow typing or smart casts.
- Exhaustiveness checking.
- Ownership or borrow analysis.
- Coroutine semantics.
- Unsafe semantics.
- Name resolution, HIR, MIR, lowering, optimization, backend behavior, or runtime behavior.
- Accepting `when`, `match`, loop, coroutine, unsafe block, indexing, lambda, receiver, or destructuring syntax.
- Public parser APIs for standalone expression, statement, or pattern parsing.

## Required Inputs

- Milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Spec sections:
  - `ADR-0024: Expression Statement And Pattern Syntax`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Existing files:
  - `crates/compiler/src/parser.rs`
  - `crates/compiler/src/ast.rs`
  - `crates/compiler/tests/parser.rs`
  - `tests/fixtures/parser/expressions/*.fixture.toml`
  - `tests/fixtures/parser/statements/*.fixture.toml`
  - `tests/fixtures/parser/patterns/*.fixture.toml`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Function body with local bindings, assignment, expression statement, return, call, member access, and binary expressions parses to ADR-0024 AST nodes.
  - Function body with trailing expression parses to `Block` and expression AST nodes.
  - `if` expression with block branches parses to `IfExpression` and nested `Block` nodes.
- Negative tests:
  - Assignment used as an expression reports unsupported expression syntax.
  - Deferred loop, unsafe, coroutine-like, match/when, indexing, and lambda-like forms do not parse as accepted body syntax.
- Diagnostic tests:
  - Malformed variable declarations, assignments, returns, calls, member access, blocks, and conditionals report source-spanned ADR-0024 diagnostics.
- Adversarial tests:
  - Parser output remains syntax-only and does not encode type checking, ownership, borrowing, flow typing, coroutine, unsafe, HIR, MIR, or backend semantics.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0013-expression-statement-pattern-parser-implementation.sh`
  - focused Rust parser test additions in `crates/compiler/tests/parser.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `crates/compiler/src/parser.rs` does not yet define ADR-0024 parser diagnostics or parse body syntax into AST nodes.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend the existing parser with syntax-only recursive descent routines for ADR-0024 body syntax. Keep all new parser output in the existing flat `AstArena`; do not add semantic payloads, public parse APIs, HIR/MIR, or type checking.

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

- Generate tests: `create docs/tests/m0013-expression-statement-pattern-parser-implementation.sh and update crates/compiler/tests/parser.rs`
- Verify tests fail: `docs/tests/m0013-expression-statement-pattern-parser-implementation.sh`
- Ordinary tests: `cargo test --workspace --all-targets parser -- --nocapture && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh`
- Adversarial tests: `docs/tests/m0013-expression-statement-pattern-parser-implementation.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0013-008-expression-statement-pattern-parser-implementation.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0013-expression-statement-pattern-ast-shell.sh && docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh && docs/tests/m0013-expression-statement-pattern-syntax-accepted.sh && docs/tests/m0013-expression-statement-pattern-syntax-concrete-draft.sh && docs/tests/m0013-expression-statement-pattern-syntax-review.sh && docs/tests/m0013-expression-statement-pattern-syntax-proposal.sh && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0013-expression-statement-pattern-parser-implementation.sh`
  - `crates/compiler/tests/parser.rs`
- Implementation files:
  - `crates/compiler/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0013-008-expression-statement-pattern-parser-implementation.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
  - existing validators that formerly required parser implementation to remain absent

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`.
- Do not accept `when`, `match`, loop, coroutine, unsafe block, indexing, lambda, receiver, or destructuring syntax.
- Do not add semantic type IDs, symbols, binding modes, ownership state, borrow state, flow facts, HIR, MIR, or backend behavior.

## Ambiguities And Dependencies

- Pattern parser support is reusable internal syntax only until a future accepted match or `when` grammar defines an external parse context.
- Coroutine-like syntax is detected only as unsupported/deferred body syntax because no coroutine keyword is accepted yet.
- Unsafe block syntax remains deferred; `unsafe` must diagnose rather than parse as a block.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0013 body parser implementation task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0013-expression-statement-pattern-parser-implementation.sh and Rust parser test additions before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0013-expression-statement-pattern-parser-implementation.sh failed before implementation because ADR-0024 parser diagnostics were missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Implemented syntax-only ADR-0024 function body parsing, statement parsing, expression parsing, reusable pattern parsing, and parser diagnostics without semantic analysis.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test --workspace --test parser -- --nocapture and docs/tests/m0013-expression-statement-pattern-parser-implementation.sh passed.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Full M0013-M0002 validation command passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0013-008-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0013-008-review.md approves ADR-0024 parser implementation scope.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add failing parser implementation tests before body parser implementation.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
