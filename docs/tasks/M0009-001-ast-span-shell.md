# Task: M0009-001 Create Syntax-Independent AST Span Shell

## Task Metadata

- Task ID: `M0009-001`
- Milestone: `M0009`
- Milestone File: `docs/milestones/M0009-ast-data-model.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0009-001-ast-span-shell`

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

Create the syntax-independent AST data-model shell and span preservation rules allowed by the M0008 grammar authority ledger.

## Motivation

M0009 needs an AST target before parser implementation, but M0008 shows concrete declaration, type, expression, statement, and pattern grammar is ambiguous. The safe step is to establish node identity, source-file root nodes, and span retention without inventing concrete syntax nodes.

## Scope

- Add AST module with node IDs, source-file root nodes, and source spans.
- Add unit tests for AST construction and span retention.
- Add AST model documentation recording deferred concrete syntax nodes.
- Add validation that parser, concrete AST syntax nodes, HIR, and MIR remain out of scope.
- Mark M0009 complete only if the model satisfies the current ledger constraints.

## Out Of Scope

- Parser implementation.
- Concrete declaration, type, expression, statement, or pattern AST nodes.
- Name resolution.
- Type checking.
- HIR or MIR.
- Semantic ownership or borrow analysis.

## Required Inputs

- Milestone: `docs/milestones/M0009-ast-data-model.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Existing files:
  - `docs/syntax/grammar-authority-ledger.md`
  - `crates/newlang/src/source.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - AST source-file root preserves node ID and source span.
  - AST arena returns nodes by stable ID.
- Negative tests:
  - Concrete syntax node APIs are absent.
- Diagnostic tests:
  - Not applicable.
- Adversarial tests:
  - Confirm no parser code is added.
  - Confirm no concrete syntax AST nodes are added while grammar is ambiguous.
  - Confirm AST does not encode semantic analysis concepts.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/ast.rs`
  - `docs/tests/m0009-ast-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `newlang::ast` does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `crates/newlang/src/ast.rs`, export it from `lib.rs`, and document the AST shell in `docs/ast/data-model.md`.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing AST shell.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0009 milestone checklist is updated.

## Execution Commands

- Generate tests: `create crates/newlang/tests/ast.rs && create docs/tests/m0009-ast-data-model.sh`
- Verify tests fail: `cargo test --workspace --all-targets ast`
- Ordinary tests: `cargo test --workspace --all-targets && docs/tests/m0009-ast-data-model.sh`
- Adversarial tests: `docs/tests/m0009-ast-data-model.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0009-001-ast-span-shell.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/ast.rs`
  - `docs/tests/m0009-ast-data-model.sh`
- Implementation files:
  - `crates/newlang/src/lib.rs`
  - `crates/newlang/src/ast.rs`
- Documentation or checklist files:
  - `docs/tasks/M0009-001-ast-span-shell.md`
  - `docs/ast/data-model.md`
  - `docs/milestones/M0009-ast-data-model.md`

## Forbidden Changes

- Do not implement parser code.
- Do not add concrete declaration, type, expression, statement, or pattern nodes.
- Do not add HIR or MIR.
- Do not add name resolution, type checking, ownership, borrow, or coroutine analysis.
- Do not modify accepted syntax authority.

## Ambiguities And Dependencies

- Concrete syntax nodes are blocked by M0008 ambiguity reports.
- M0011-M0013 require future accepted syntax authority before concrete parser work.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0009 AST span shell task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created AST tests and M0009 validator before adding ast module.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Rust tests failed as expected because newlang::ast did not exist.
2026-07-09 main_task=main-task implementation phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets and M0009 AST validator passed after adding syntax-independent AST shell.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms no parser code, no concrete syntax nodes, and no semantic analysis concepts were added.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved AST shell against M0008 grammar constraints and M0009 acceptance criteria.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Implement syntax-independent AST span shell.`
- Required Context:
  - This task file
  - `docs/syntax/grammar-authority-ledger.md`
  - `crates/newlang/src/source.rs`
