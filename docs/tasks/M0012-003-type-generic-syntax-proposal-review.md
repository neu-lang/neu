# Task: M0012-003 Review ADR-0023 type and generic syntax proposal

## Task Metadata

- Task ID: `M0012-003`
- Milestone: `M0012`
- Milestone File: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Status: `complete`
- Owner Agent: `Language Lawyer | Adversarial Engineer | Diagnostics Engineer | Simplicity Guardian | Chief Architect`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0012-003-type-generic-syntax-proposal-review`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/adversarial-engineer.md`
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/simplicity-guardian.md`
  - `.codex/agents/chief-architect.md`

## Goal

Review the ADR-0023 type and generic syntax proposal and record concrete revision requirements before any acceptance or parser implementation.

## Motivation

The draft ADR-0023 proposal identifies the right decision surface, but M0012 cannot proceed until reviewers determine whether it is concrete enough to become source of truth. Review artifacts make the acceptance blockers explicit and prevent accidental parser implementation from a draft.

## Scope

- Add Language Lawyer review for ADR-0023.
- Add Adversarial Engineer review for capability-bound and soundness risks.
- Add Diagnostics Engineer review for type syntax diagnostic obligations.
- Add Simplicity Guardian review for grammar complexity.
- Add Chief Architect decision artifact recording whether the proposal is accepted or needs revision.
- Keep ADR-0023 draft and M0012 ambiguity non-authoritative and open.

## Out Of Scope

- Accepting ADR-0023.
- Revising ADR-0023 into concrete accepted grammar.
- Updating `docs/SPEC.md`.
- Adding type parser implementation, type AST nodes, or fixtures.
- Closing `docs/ambiguities/M0008-type-generic-syntax.md`.

## Required Inputs

- Milestone: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Spec sections:
  - `ADR-0006: Nullability And Absence`
  - `ADR-0010: Type System Shape`
  - `ADR-0015: Diagnostics As Semantics`
  - `ADR-0016: Generics And Parametric Polymorphism`
- ADRs:
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Existing files:
  - `docs/ambiguities/M0008-type-generic-syntax.md`
  - `docs/syntax/grammar-authority-ledger.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Review artifacts exist for Language Lawyer, Adversarial Engineer, Diagnostics Engineer, Simplicity Guardian, and Chief Architect.
- Negative tests:
  - Chief Architect decision remains pending and proposal remains non-authoritative.
  - Accepted ADR-0023, parser type APIs, type AST nodes, and type/generic fixtures remain absent.
- Diagnostic tests:
  - Diagnostics review lists required diagnostic fields and categories before acceptance.
- Adversarial tests:
  - Adversarial review identifies capability-bound, variance, nullable, and function-type risks.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0012-type-generic-syntax-review.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - ADR-0023 review files do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create review documents only. Do not modify the proposal grammar or accept it as source of truth in this task.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0012 remains blocked pending accepted syntax authority.

## Execution Commands

- Generate tests: `create docs/tests/m0012-type-generic-syntax-review.sh`
- Verify tests fail: `docs/tests/m0012-type-generic-syntax-review.sh`
- Ordinary tests: `docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh`
- Adversarial tests: `docs/tests/m0012-type-generic-syntax-review.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0012-003-type-generic-syntax-proposal-review.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0012-type-generic-syntax-review.sh && docs/tests/m0012-type-generic-syntax-proposal.sh && docs/tests/m0012-type-generic-parser-blocked.sh && docs/tests/m0011-declaration-parser-implementation.sh && docs/tests/m0011-declaration-ast-shell.sh && docs/tests/m0011-declaration-parser-fixtures.sh && docs/tests/m0011-declaration-syntax-accepted.sh && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0012-type-generic-syntax-review.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/adr/proposals/reviews/ADR-0023-language-lawyer-review.md`
  - `docs/adr/proposals/reviews/ADR-0023-adversarial-review.md`
  - `docs/adr/proposals/reviews/ADR-0023-diagnostics-review.md`
  - `docs/adr/proposals/reviews/ADR-0023-simplicity-review.md`
  - `docs/adr/proposals/reviews/ADR-0023-chief-architect-decision.md`
  - `docs/tasks/M0012-003-type-generic-syntax-proposal-review.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs under `docs/adr/`.
- Do not accept ADR-0023.
- Do not close `docs/ambiguities/M0008-type-generic-syntax.md`.
- Do not add type parser APIs, type AST nodes, or type/generic fixtures.

## Ambiguities And Dependencies

- ADR-0023 remains draft-only until revised and accepted.
- M0012 remains blocked by `docs/ambiguities/M0008-type-generic-syntax.md`.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created ADR-0023 proposal review task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Created review validator before adding review files.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=docs/tests/m0012-type-generic-syntax-review.sh failed because ADR-0023 review files were missing.
2026-07-10 agent=Language-Lawyer phase=implementation result=pass notes=Added Language Lawyer, Adversarial, Diagnostics, Simplicity, and Chief Architect review artifacts requesting revision before acceptance.
2026-07-10 agent=Language-Lawyer phase=ordinary-tests result=pass notes=docs/tests/m0012-type-generic-syntax-review.sh, proposal validator, and blocker validator passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0012-003-soundness.md after ordinary-test evidence.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0012-003-review.md approves review artifacts and preserves M0012 blocker.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0012-M0002 validation scripts passed.
```

## Handoff

- Next Agent: `Language Designer`
- Reason: `Revise ADR-0023 into concrete grammar after review findings.`
- Required Context:
  - This task file
  - ADR-0023 review files
  - `docs/adr/proposals/ADR-0023-type-and-generic-syntax.md`
  - `docs/ambiguities/M0008-type-generic-syntax.md`
