# Task: M0013-001 Record expression statement and pattern syntax blocker

## Task Metadata

- Task ID: `M0013-001`
- Milestone: `M0013`
- Milestone File: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0013-001-expression-statement-pattern-syntax-blocker`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0007-error-handling.md`
  - `docs/adr/ADR-0008-structured-concurrency-semantics.md`
  - `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Record that M0013 parser implementation is blocked until expression, statement, and pattern syntax has accepted source-of-truth authority.

## Motivation

M0013 depends on concrete body syntax, operator precedence, pattern grammar, block grammar, unsafe syntax, coroutine syntax, and diagnostics. Current accepted ADRs define semantic goals but do not define grammar. Guessing from Kotlin, Rust, Go, or existing parser behavior would violate the project rule that main tasks may not invent language semantics.

## Scope

- Confirm the existing ambiguity report blocks M0013.
- Confirm the grammar authority ledger still classifies expression, statement, pattern, coroutine, and unsafe syntax as ambiguous.
- Add a validator that prevents expression, statement, pattern, coroutine, and unsafe parser implementation or fixtures before accepted grammar exists.
- Mark the M0013 milestone checklist item for ambiguous syntax being blocked.

## Out Of Scope

- Drafting expression, statement, or pattern syntax.
- Creating parser fixtures for concrete expression, statement, or pattern forms.
- Adding AST nodes for expressions, statements, or patterns.
- Implementing parser behavior.
- Changing `docs/SPEC.md` or accepted ADRs.
- Resolving the ambiguity.

## Required Inputs

- Milestone: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Ambiguity report: `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- Ledger: `docs/syntax/grammar-authority-ledger.md`
- Spec sections:
  - `ADR-0007: Error Handling`
  - `ADR-0008: Structured Concurrency Semantics`
  - `ADR-0009: Async Suspension And Borrowing`
  - `ADR-0011: Flow Typing And Smart Casts`
  - `ADR-0012: Pattern Matching And Algebraic Data`
  - `ADR-0018: Unsafe, FFI, And Trust Boundaries`
  - `ADR-0021: Lexical Grammar`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Validator proves the ambiguity report exists and is open.
- Negative tests:
  - Validator fails if expression, statement, pattern, coroutine, or unsafe parser APIs or fixtures appear while syntax is ambiguous.
- Diagnostic tests:
  - Not applicable; parser diagnostics cannot be created before grammar authority.
- Adversarial tests:
  - Validator rejects attempts to infer syntax from existing parser behavior or fixture paths.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Expected pre-implementation result: `pass`
- Failure reason expected before implementation:
  - Not applicable; this blocker task records the current correct blocked state.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a blocker-state validator and task metadata only. Do not create syntax, fixtures, AST nodes, parser APIs, or implementation.

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

- Generate tests: `create docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Verify tests fail: `not applicable; blocker-state validator should pass when M0013 is correctly blocked`
- Ordinary tests: `docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0008-grammar-authority-ledger.sh`
- Adversarial tests: `docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0013-001-expression-statement-pattern-syntax-blocker.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0013-expression-statement-pattern-parser-blocked.sh && docs/tests/m0012-type-generic-parser-implementation.sh && docs/tests/m0012-type-ast-shell.sh && docs/tests/m0012-type-generic-parser-fixtures.sh && docs/tests/m0012-type-generic-syntax-accepted.sh && docs/tests/m0012-type-generic-syntax-concrete-draft.sh && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0013-expression-statement-pattern-parser-blocked.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0013-001-expression-statement-pattern-syntax-blocker.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.
- Do not add expression, statement, pattern, coroutine, or unsafe parser APIs.
- Do not add expression, statement, or pattern AST nodes.
- Do not add concrete expression, statement, or pattern fixtures.

## Ambiguities And Dependencies

- M0013 remains blocked by `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`.
- The next safe task is for main-task semantic design to draft a non-authoritative syntax proposal.

## Execution Log

Append entries as the task progresses.

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0013 expression/statement/pattern syntax blocker task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0013-expression-statement-pattern-parser-blocked.sh to enforce blocked state.
2026-07-10 main_task=Language-Lawyer phase=implementation result=pass notes=Recorded M0013 blocker without adding syntax or parser implementation.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=M0013 blocker validator and M0008 ledger validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0013-001-soundness.md after ordinary-tests evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0013-001-review.md approves blocked scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Full M0013-M0002 validation command passed.
```

## Handoff

- Next main task: `main-task semantic design`
- Reason: `Draft expression, statement, and pattern syntax proposal without accepting it as source of truth.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
