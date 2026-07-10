# Task: M0007-008 Implement ADR-0021 Lexer

## Task Metadata

- Task ID: `M0007-008`
- Milestone: `M0007`
- Milestone File: `docs/milestones/M0007-lexer-implementation.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0007-008-lexer-implementation`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Implement the smallest lexer module that satisfies ADR-0021 and the M0007 lexer fixtures.

## Motivation

M0007 now has accepted lexical authority and concrete fixtures. The compiler needs a lexer that emits tokens and lexical diagnostics with source spans for later parser work.

## Scope

- Add lexer API and token/diagnostic kinds for ADR-0021.
- Return token and diagnostic source spans.
- Support whitespace, comments, identifiers, keywords, integer literals, string literals, operators, delimiters, and lexical errors from ADR-0021.
- Add executable tests based on M0007 fixtures.
- Update docs validators that previously rejected `crates/newlang/src/lexer.rs`.

## Out Of Scope

- Parser behavior.
- AST/HIR/MIR.
- Literal type checking or integer overflow diagnostics.
- Unicode identifiers.
- Raw strings, interpolation, char literals, numeric suffix support.
- Error recovery beyond ADR-0021 lexical categories.

## Required Inputs

- Milestone: `docs/milestones/M0007-lexer-implementation.md`
- Spec sections:
  - `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Existing files:
  - `tests/fixtures/lexer/*.fixture.toml`
  - `crates/newlang/src/source.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Lexer emits expected token kinds for keywords, identifiers, literals, comments, operators, and delimiters.
- Negative tests:
  - Lexer reports ADR-0021 lexical diagnostic kinds for unsupported Unicode identifiers, invalid escapes, malformed integer literals, unterminated comments, unterminated strings, and unknown characters.
- Diagnostic tests:
  - Diagnostics include `ByteSpan`s whose text matches the offending source text.
- Adversarial tests:
  - Confirm integer overflow is not a lexer error.
  - Confirm comments are skipped.
  - Confirm longest operator matching.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/lexer.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `newlang::lexer` does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `crates/newlang/src/lexer.rs` and export it from `lib.rs`. Keep token and diagnostic data structures in the lexer module until later architecture requires a separate token module.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing lexer change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0007 milestone checklist is updated.

## Execution Commands

- Generate tests: `create crates/newlang/tests/lexer.rs`
- Verify tests fail: `cargo test --workspace --all-targets lexer`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/tests/m0007-lexer-implementation.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0007-008-lexer-implementation.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0007-lexer-implementation.sh && docs/tests/m0007-lexer-fixtures.sh && docs/tests/m0007-lexical-grammar-accepted.sh && docs/tests/m0007-blocker-status-sync.sh && docs/tests/m0007-language-designer-review.sh && docs/tests/m0007-lexical-grammar-review.sh && docs/tests/m0007-lexical-grammar-proposal.sh && docs/tests/m0007-lexer-blocked.sh && docs/tests/m0006-token-model-fixtures.sh && docs/tests/m0005-source-spans.sh && docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/lexer.rs`
  - `docs/tests/m0007-lexer-implementation.sh`
  - existing docs validators that formerly rejected `crates/newlang/src/lexer.rs`
- Implementation files:
  - `crates/newlang/src/lib.rs`
  - `crates/newlang/src/source.rs`
  - `crates/newlang/src/lexer.rs`
- Documentation or checklist files:
  - `docs/tasks/M0007-008-lexer-implementation.md`
  - `docs/milestones/M0007-lexer-implementation.md`

## Forbidden Changes

- Do not add parser, AST, HIR, MIR, or backend code.
- Do not add a garbage collector, allocator, runtime, or code generation.
- Do not weaken or delete tests to pass CI.
- Do not implement deferred lexical features.
- Do not treat integer overflow as a lexer error.

## Ambiguities And Dependencies

- Parser grammar remains out of scope.
- Token naming is an implementation API and may be refined by later parser milestones.
- Unicode identifiers remain deferred by ADR-0021.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0007 lexer implementation task.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created crates/newlang/tests/lexer.rs before adding lexer implementation.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Rust tests failed as expected because newlang::lexer did not exist.
2026-07-09 main_task=main-task implementation phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets and M0007 lexer implementation validator passed after adding lexer module.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Adversarial report confirms no parser behavior, no unsafe code, no overflow lexer diagnostic, and Unicode identifiers remain rejected.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved lexer implementation against ADR-0021 and M0007 fixtures.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Full CI-equivalent gate passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add executable lexer tests before implementation.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `tests/fixtures/lexer/*.fixture.toml`
