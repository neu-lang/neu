# Task: M0010-001 Define Parser Recovery Architecture

## Task Metadata

- Task ID: `M0010-001`
- Milestone: `M0010`
- Milestone File: `docs/milestones/M0010-parser-recovery-architecture.md`
- Status: `complete`
- Owner Agent: `Diagnostics Engineer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-09`
- Branch: `task/M0010-001-parser-recovery-architecture`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/diagnostics-engineer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/reviewer.md`

## Goal

Define parser recovery architecture, syntax diagnostic shape, and synthetic parser diagnostic fixture conventions without parsing ambiguous language syntax.

## Motivation

M0010 must establish parser recovery and diagnostic behavior before concrete parser milestones. M0008 shows declaration, type, expression, statement, and pattern grammar remains ambiguous, so this task must define recovery infrastructure rules without accepting concrete syntax.

## Scope

- Add parser recovery architecture documentation.
- Add syntax diagnostic fixture format documentation.
- Add synthetic parser diagnostic fixture and golden snapshot.
- Add validation that synthetic parser diagnostics include primary spans and source-of-truth citations.
- Keep concrete parser implementation and ambiguous syntax fixtures out of scope.

## Out Of Scope

- Full parser implementation.
- Parser code.
- Concrete grammar fixtures for declarations, types, expressions, statements, or patterns.
- Semantic diagnostics.
- AST node expansion.
- Modifying `docs/SPEC.md` or accepted ADRs.

## Required Inputs

- Milestone: `docs/milestones/M0010-parser-recovery-architecture.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Existing files:
  - `docs/diagnostics.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/ast/data-model.md`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0010-parser-recovery-architecture.sh` verifies recovery docs, fixture format, and synthetic diagnostics.
- Negative tests:
  - The validation script must fail before recovery docs and synthetic fixture artifacts exist.
- Diagnostic tests:
  - Synthetic parser diagnostic golden must include severity, message, primary span, recovery action, and source-of-truth citation.
- Adversarial tests:
  - Confirm no parser code is added.
  - Confirm no parser fixture encodes ambiguous concrete syntax.
  - Confirm recovery architecture does not promise semantic diagnostics.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0010-parser-recovery-architecture.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `docs/parser/recovery.md` does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add documentation and synthetic fixture artifacts only. Use synthetic token names and spans to define diagnostic shape without parsing user syntax.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing architecture artifact.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0010 milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0010-parser-recovery-architecture.sh`
- Verify tests fail: `docs/tests/m0010-parser-recovery-architecture.sh`
- Ordinary tests: `docs/tests/m0010-parser-recovery-architecture.sh`
- Adversarial tests: `docs/tests/m0010-parser-recovery-architecture.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0010-001-parser-recovery-architecture.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0010-parser-recovery-architecture.sh && docs/tests/m0009-ast-data-model.sh && docs/tests/m0008-grammar-authority-ledger.sh && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0010-parser-recovery-architecture.sh`
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0010-001-parser-recovery-architecture.md`
  - `docs/parser/recovery.md`
  - `docs/parser/syntax-diagnostic-fixtures.md`
  - `tests/fixtures/diagnostics/M0010-synthetic-parser-error.fixture.toml`
  - `tests/golden/diagnostics/M0010-synthetic-parser-error.diagnostic.toml`
  - `docs/milestones/M0010-parser-recovery-architecture.md`

## Forbidden Changes

- Do not add `crates/newlang/src/parser.rs`.
- Do not add concrete parser fixtures.
- Do not add concrete syntax AST nodes.
- Do not modify accepted ADRs or `docs/SPEC.md`.
- Do not define parser grammar.

## Ambiguities And Dependencies

- Concrete declaration, type, expression, statement, and pattern syntax remain blocked by M0008 ambiguity reports.
- Future parser implementation must cite accepted syntax authority before parsing concrete constructs.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 agent=Task-Decomposer phase=create-task result=pass notes=Created M0010 parser recovery architecture task.
2026-07-09 agent=Test-Engineer phase=generate-tests result=pass notes=Created M0010 validator before adding recovery docs and synthetic diagnostics.
2026-07-09 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Validation failed as expected: missing docs/parser/recovery.md.
2026-07-09 agent=Diagnostics-Engineer phase=ordinary-tests result=pass notes=M0010 parser recovery architecture validation passed after adding recovery docs and synthetic diagnostics.
2026-07-09 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms no concrete grammar fixture, parser code, or semantic diagnostics were added.
2026-07-09 agent=Reviewer phase=review result=pass notes=Review approved parser recovery architecture and synthetic diagnostic shape.
2026-07-09 agent=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next Agent: `Diagnostics Engineer`
- Reason: `Define parser recovery and diagnostic shape without parsing ambiguous syntax.`
- Required Context:
  - This task file
  - `docs/diagnostics.md`
  - `docs/syntax/grammar-authority-ledger.md`
  - `docs/ast/data-model.md`
