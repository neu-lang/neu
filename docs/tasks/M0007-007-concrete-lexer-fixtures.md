# Task: M0007-007 Add Concrete Lexer Fixtures

## Task Metadata

- Task ID: `M0007-007`
- Milestone: `M0007`
- Milestone File: `docs/milestones/M0007-lexer-implementation.md`
- Status: `complete`
- Owner Agent: `Test Engineer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-09`
- Branch: `task/M0007-007-concrete-lexer-fixtures`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/test-engineer.md`
  - `.codex/agents/language-lawyer.md`
  - `.codex/agents/reviewer.md`

## Goal

Add concrete lexer fixture files backed by accepted ADR-0021 so the lexer implementation task can be test-first.

## Motivation

M0007 requires lexer fixtures before implementation. ADR-0021 now defines accepted lexical grammar, so the project can add positive and negative fixture data without guessing language semantics.

## Scope

- Add concrete lexer fixture TOML files under `tests/fixtures/lexer/`.
- Cover keywords, identifiers, literals, comments, operators, delimiters, and lexical errors from ADR-0021.
- Update token-model planning status from blocked to specified for ADR-0021-backed categories.
- Update earlier M0007 validators so concrete fixtures are no longer treated as out of scope.

## Out Of Scope

- Implementing lexer code.
- Adding Rust tests that execute a lexer.
- Parser grammar, precedence, or recovery behavior.
- Token enum or API design.
- Changing accepted lexical semantics.

## Required Inputs

- Milestone: `docs/milestones/M0007-lexer-implementation.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Existing files:
  - `docs/lexer/token-model.md`
  - `tests/fixtures/lexer/M0006-inert.fixture.toml`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0007-lexer-fixtures.sh` verifies concrete ADR-0021-backed lexer fixtures exist.
- Negative tests:
  - The validation script must fail before fixture files are added.
- Diagnostic tests:
  - Error fixtures must include lexical diagnostic categories from ADR-0021.
- Adversarial tests:
  - Confirm no lexer implementation is introduced.
  - Confirm fixtures cite ADR-0021 rather than Kotlin precedent.
  - Confirm parser behavior is not encoded in lexer fixtures.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0007-lexer-fixtures.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Concrete lexer fixture files do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add fixture metadata and expected token/diagnostic examples only. Do not add compiler code or executable lexer tests until the next M0007 implementation task.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing fixture addition.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Lexer implementation remains a separate task.

## Execution Commands

- Generate tests: `create docs/tests/m0007-lexer-fixtures.sh`
- Verify tests fail: `docs/tests/m0007-lexer-fixtures.sh`
- Ordinary tests: `docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh`
- Adversarial tests: `docs/tests/m0007-lexer-fixtures.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0007-007-concrete-lexer-fixtures.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0007-lexer-fixtures.sh`
  - existing M0007 validators that formerly rejected concrete lexer fixtures
- Implementation files:
  - none
- Documentation or checklist files:
  - `docs/tasks/M0007-007-concrete-lexer-fixtures.md`
  - `docs/lexer/token-model.md`
  - `tests/fixtures/lexer/keywords.fixture.toml`
  - `tests/fixtures/lexer/identifiers.fixture.toml`
  - `tests/fixtures/lexer/literals.fixture.toml`
  - `tests/fixtures/lexer/comments.fixture.toml`
  - `tests/fixtures/lexer/operators-delimiters.fixture.toml`
  - `tests/fixtures/lexer/errors.fixture.toml`

## Forbidden Changes

- Do not implement lexer code.
- Do not add `crates/newlang/src/lexer.rs`.
- Do not add `crates/newlang/src/token.rs`.
- Do not add parser, AST, HIR, MIR, or backend code.
- Do not encode parser precedence or expression grammar.
- Do not cite Kotlin as source of truth.

## Ambiguities And Dependencies

- Fixture data depends on accepted ADR-0021.
- Executable lexer tests depend on a future lexer API task.
- Parser grammar remains outside this task.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 agent=Task-Decomposer phase=create-task result=pass notes=Created M0007 concrete lexer fixture task.
2026-07-09 agent=Test-Engineer phase=generate-tests result=pass notes=Created docs/tests/m0007-lexer-fixtures.sh before adding concrete fixture files.
2026-07-09 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Validation failed as expected before token model and concrete fixture files were added.
2026-07-09 agent=Test-Engineer phase=ordinary-tests result=pass notes=M0007 concrete lexer fixture validation passed after adding ADR-0021-backed fixtures.
2026-07-09 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=Validation confirms fixtures cite ADR-0021 and no lexer or parser implementation was added.
2026-07-09 agent=Reviewer phase=review result=pass notes=Review approved fixture coverage for ADR-0021 token categories and lexical diagnostics.
2026-07-09 agent=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next Agent: `Test Engineer`
- Reason: `Add concrete lexer fixtures before lexer implementation.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/lexer/token-model.md`
