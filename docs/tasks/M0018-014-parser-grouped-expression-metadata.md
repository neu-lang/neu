# Task: M0018-014 Parser Grouped Expression Metadata

## Task Metadata

- Task ID: `M0018-014`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0018-014-parser-grouped-expression-metadata`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Expose parser metadata for grouped expressions so M0018 can later type grouped expressions from the type of their inner expression.

## Motivation

ADR-0027 includes grouped expressions in the M0018 type-checking subset. The parser already creates grouped expression AST nodes, but it does not expose the inner expression relationship as type-checker input metadata.

## Scope

- Add parser output metadata for accepted grouped expressions.
- Record the grouped expression AST node.
- Record the inner expression AST node.
- Preserve source order.
- Exclude malformed grouped expressions from metadata.

## Out Of Scope

- Typing grouped expressions.
- Changing expression grammar.
- Adding AST child storage.
- Typing unsupported expressions.
- Adding assignment, call, member, ownership, borrow, HIR, MIR, or backend behavior.

## Required Inputs

- Parser grouped expression AST nodes.
- Existing expression node metadata lookup helpers.
- ADR-0027 grouped expression inclusion.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Grouped literal expressions record outer and inner expression nodes.
  - Nested grouped expressions preserve source order.
  - Inner expression nodes are the actual parsed inner expression nodes.
- Negative tests:
  - Malformed grouped expressions do not synthesize grouped expression metadata.
  - Existing literal expression and name reference metadata remains unchanged.
- Adversarial tests:
  - Metadata addition does not type grouped expressions or introduce AST child semantics.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/parser.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `ParseOutput` has no grouped expression metadata field yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a `ParsedGroupedExpression` parser metadata record and populate it in `parse_grouped_expression` using the already-created grouped expression node and the latest expression node for the inner expression span.

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
- [x] No compiler behavior beyond parser grouped expression metadata is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/parser.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-014-parser-grouped-expression-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-014-parser-grouped-expression-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/parser.rs`
- Implementation files:
  - `crates/newlang/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-014-parser-grouped-expression-metadata.md`
  - `docs/tasks/reviews/M0018-014-review.md`
  - `docs/tasks/soundness/M0018-014-soundness.md`

## Forbidden Changes

- Do not type grouped expressions in this task.
- Do not add AST child storage.
- Do not change expression grammar or recovery.
- Do not implement unsupported expression typing.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing parser or type-checking tests.

## Ambiguities And Dependencies

- Consuming grouped expression metadata in the type checker remains a later M0018 task.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0018 parser grouped expression metadata task.
- 2026-07-10 agent=Test-Engineer phase=test-first result=pass notes=`cargo test --workspace --all-targets` failed before implementation because `ParseOutput` had no `grouped_expressions` field.
- 2026-07-10 agent=Implementer phase=implementation result=pass notes=Added parser grouped expression metadata with source-order records of grouped and inner expression nodes.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 148 tests.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-014-parser-grouped-expression-metadata.md` created a passing soundness report after ordinary tests.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=Review approved parser metadata scope, spec compliance, and maintainability.
- 2026-07-10 agent=Implementer phase=ci result=pass notes=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets`; `sh docs/tests/m0018-type-checking-core-accepted.sh`; `sh docs/tests/m0002-workspace-ci.sh`.
