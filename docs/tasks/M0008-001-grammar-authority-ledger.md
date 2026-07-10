# Task: M0008-001 Create Grammar Authority Ledger

## Task Metadata

- Task ID: `M0008-001`
- Milestone: `M0008`
- Milestone File: `docs/milestones/M0008-grammar-authority-and-syntax-ambiguity-ledger.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0008-001-grammar-authority-ledger`

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

Create a grammar authority ledger that classifies planned parser constructs as specified, ambiguous, or deferred before parser implementation begins.

## Motivation

M0008 exists to prevent parser work from assuming Kotlin grammar or turning lexer tokens into de facto syntax. Parser milestones need a durable ledger that says which syntax has source-of-truth authority and which syntax requires ambiguity reports.

## Scope

- Add `docs/syntax/grammar-authority-ledger.md`.
- Classify planned parser constructs for M0011, M0012, and M0013.
- Record ambiguity reports for syntax that lacks accepted grammar authority.
- Define the parser unblock list for constructs that may safely proceed.
- Validate that no parser code or parser fixtures are added.

## Out Of Scope

- Parser implementation.
- Parser fixtures for ambiguous constructs.
- Modifying `docs/SPEC.md`.
- Creating new language syntax.
- Accepting new syntax ADRs.
- Changing lexer behavior.

## Required Inputs

- Milestone: `docs/milestones/M0008-grammar-authority-and-syntax-ambiguity-ledger.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Roadmap:
  - `docs/milestones/M0011-declaration-parser.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0008-grammar-authority-ledger.sh` verifies the ledger exists and classifies parser constructs.
- Negative tests:
  - The validation script must fail before the ledger and ambiguity reports are added.
- Diagnostic tests:
  - Not applicable.
- Adversarial tests:
  - Confirm no parser code is introduced.
  - Confirm no parser fixtures are added for ambiguous constructs.
  - Confirm ambiguous constructs have owner and blocking milestone.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0008-grammar-authority-ledger.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `docs/syntax/grammar-authority-ledger.md` does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a syntax ledger plus ambiguity reports only. Classify syntax by existing authority without inventing grammar.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing planning artifact.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone checklist is updated if the whole milestone is satisfied.

## Execution Commands

- Generate tests: `create docs/tests/m0008-grammar-authority-ledger.sh`
- Verify tests fail: `docs/tests/m0008-grammar-authority-ledger.sh`
- Ordinary tests: `docs/tests/m0008-grammar-authority-ledger.sh`
- Adversarial tests: `docs/tests/m0008-grammar-authority-ledger.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0008-001-grammar-authority-ledger.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0008-grammar-authority-ledger.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0008-001-grammar-authority-ledger.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
  - `docs/milestones/M0008-grammar-authority-and-syntax-ambiguity-ledger.md`

## Forbidden Changes

- Do not implement parser code.
- Do not add parser fixtures.
- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs.
- Do not define new grammar.
- Do not assume Kotlin syntax beyond accepted source-of-truth text.

## Ambiguities And Dependencies

- Declaration grammar is ambiguous.
- Type and generic syntax grammar is ambiguous.
- Expression, statement, and pattern grammar is ambiguous.
- M0011, M0012, and M0013 remain blocked until their relevant syntax authority is accepted.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0008 grammar authority ledger task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0008-grammar-authority-ledger.sh before adding ledger and ambiguity reports.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/syntax/grammar-authority-ledger.md.
2026-07-09 main_task=Language-Lawyer phase=ordinary-tests result=pass notes=M0008 grammar authority ledger validation passed after adding ledger and ambiguity reports.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms no parser code or parser fixtures were added and ambiguous constructs remain blocked.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved M0008 ledger and ambiguity reports as scoped planning artifacts.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next main task: `main-task language review`
- Reason: `Classify parser syntax authority without inventing grammar.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/`
  - `docs/milestones/M0011-declaration-parser.md`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
  - `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
