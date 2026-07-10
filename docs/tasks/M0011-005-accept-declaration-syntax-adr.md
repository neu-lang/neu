# Task: M0011-005 Accept ADR-0022 declaration syntax

## Task Metadata

- Task ID: `M0011-005`
- Milestone: `M0011`
- Milestone File: `docs/milestones/M0011-declaration-parser.md`
- Status: `complete`
- Owner main task: `main task`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0011-005-<slug>`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/proposals/ADR-0022-declaration-syntax.md`
  - `docs/adr/proposals/reviews/ADR-0022-chief-architect-decision.md`
- Project Rules: `main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Accept ADR-0022 as declaration syntax source of truth, resolve the M0011 declaration syntax ambiguity, and unblock declaration parser fixture and implementation tasks without implementing parser behavior.

## Motivation

M0011 cannot safely parse concrete declaration syntax while package, import, visibility, and declaration forms are only described in a draft proposal. The concrete draft and reviews now provide enough reviewed material for the main task to either accept or reject a source-of-truth update.

## Scope

- Add accepted `docs/adr/ADR-0022-declaration-syntax.md`.
- Add an ADR-0022 summary to `docs/SPEC.md`.
- Resolve `docs/ambiguities/M0008-declaration-syntax.md`.
- Update `docs/syntax/grammar-authority-ledger.md` for the declaration constructs covered by ADR-0022.
- Update the main task decision artifact to record final approval.
- Update M0011 validation scripts so they distinguish historical draft/review evidence from accepted source of truth.

## Out Of Scope

- Parser implementation.
- Parser fixtures for concrete declarations.
- AST declaration node expansion.
- Type, generic, expression, statement, pattern, ownership, or name-resolution syntax.
- New language semantics beyond the reviewed ADR-0022 declaration grammar.

## Required Inputs

- Milestone: `docs/milestones/M0011-declaration-parser.md`
- Spec sections:
  - `ADR-0015: Diagnostics As Semantics`
  - `ADR-0017: Modules, Visibility, And API Evolution`
  - `ADR-0021: Lexical Grammar`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Existing files:
  - `docs/adr/proposals/ADR-0022-declaration-syntax.md`
  - `docs/adr/proposals/reviews/ADR-0022-chief-architect-decision.md`
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Accepted ADR-0022 exists, is marked accepted, and contains concrete declaration grammar sections.
  - `docs/SPEC.md`, the ambiguity report, main task decision, and grammar authority ledger all point to accepted ADR-0022.
- Negative tests:
  - Parser implementation and parser fixture paths remain absent during this source-of-truth acceptance task.
- Diagnostic tests:
  - ADR-0022 defines declaration diagnostic categories with primary span, recovery action, and safe suggestion policy.
- Adversarial tests:
  - Type/generic/expression/statement/pattern syntax remain ambiguous or deferred rather than being accidentally accepted.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0011-declaration-syntax-accepted.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted `docs/adr/ADR-0022-declaration-syntax.md` does not exist yet and the ambiguity remains open.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create an accepted ADR from the reviewed concrete draft, update only the source-of-truth and governance documents needed to record acceptance, and leave parser implementation for the next task.

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

- Generate tests: `create docs/tests/m0011-declaration-syntax-accepted.sh`
- Verify tests fail: `docs/tests/m0011-declaration-syntax-accepted.sh`
- Ordinary tests: `docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0011-declaration-syntax-concrete-draft.sh && docs/tests/m0011-declaration-syntax-review.sh && docs/tests/m0011-declaration-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0011-declaration-syntax-accepted.sh`
- Review: `docs/scripts/review-task.sh <task-file>`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0011-declaration-syntax-concrete-draft.sh && docs/tests/m0011-declaration-syntax-review.sh && docs/tests/m0011-declaration-parser-blocked.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0011-declaration-syntax-accepted.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/SPEC.md`
  - `docs/ambiguities/M0008-declaration-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/adr/proposals/reviews/ADR-0022-chief-architect-decision.md`
  - `docs/tests/m0011-declaration-syntax-concrete-draft.sh`
  - `docs/tests/m0011-declaration-syntax-review.sh`
  - `docs/tests/m0011-declaration-parser-blocked.sh`

## Forbidden Changes

- Do not modify compiler source files.
- Do not add parser fixtures.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- Type and generic syntax remain unresolved for M0012.
- Expression, statement, pattern, coroutine, and unsafe block syntax remain unresolved for M0013.
- Function parameter and return type contents remain placeholders until type syntax is accepted.
- Function bodies remain declaration bodies or semicolon placeholders until statement and expression syntax are accepted.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=<main task> phase=<phase> result=<result> notes=<notes>
2026-07-09 main_task=Chief-Architect phase=task-created result=pass notes=Created acceptance task for ADR-0022 without parser implementation.
2026-07-09 main_task=main-task test work phase=pre-implementation-test result=fail notes=docs/tests/m0011-declaration-syntax-accepted.sh failed because docs/adr/ADR-0022-declaration-syntax.md was missing.
2026-07-09 main_task=Chief-Architect phase=implementation result=pass notes=Accepted ADR-0022, updated SPEC, resolved ambiguity, updated ledger and M0011 validators.
2026-07-09 main_task=Build-Engineer phase=ordinary-tests result=pass notes=Focused M0011 source-of-truth validators reached only task-completion guard before task status update; no source-of-truth mismatches remained.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0011-005-soundness.md after ordinary-tests evidence.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0011-M0002 validation scripts passed.
2026-07-09 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0011-005-review.md approves scope, spec compliance, and milestone alignment.
```

## Handoff

- Next main task: `main task`
- Reason: `Final ADR acceptance and source-of-truth alignment.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - Relevant ADRs
  - Milestone file
