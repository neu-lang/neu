# Task: M0018-016 Parser Assignment Statement Metadata

## Task Metadata

- Task ID: `M0018-016`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-016-parser-assignment-statement-metadata`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Expose parser metadata for assignment statements so M0018 can later check exact assignment compatibility when both sides have known types.

## Motivation

ADR-0027 includes assignment statements where both sides have known types. The parser currently creates assignment statement AST nodes, but it does not expose a side table connecting each statement to its target expression and value expression.

## Scope

- Add parser output metadata for accepted assignment statements.
- Record the assignment statement AST node.
- Record the target expression AST node.
- Record the value expression AST node.
- Preserve source order.
- Exclude malformed assignment statements from metadata.

## Out Of Scope

- Type checking assignment statements.
- Resolving assignment targets.
- Restricting assignment targets semantically.
- Changing expression grammar or parser recovery.
- Adding AST child storage.
- Typing unsupported expressions.
- Ownership, borrow, HIR, MIR, or backend behavior.

## Required Inputs

- Parser assignment statement AST nodes.
- Existing expression node metadata lookup helpers.
- ADR-0027 assignment compatibility inclusion.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Simple assignment statements record statement, target, and value expression nodes.
  - Multiple assignment statements preserve source order.
  - Target and value nodes are the actual parsed expression nodes.
- Negative tests:
  - Malformed assignments do not synthesize assignment metadata.
  - Expression statements and local declarations do not synthesize assignment metadata.
- Adversarial tests:
  - Metadata addition does not type assignments, infer types, resolve names, or add target legality rules.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/compiler/tests/parser.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `ParseOutput` has no assignment statement metadata field yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a `ParsedAssignmentStatement` parser metadata record and populate it only when an assignment statement parses successfully and both target and value expression nodes can be identified from their spans.

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
- [x] No compiler behavior beyond parser assignment statement metadata is introduced.

## Execution Commands

- Generate tests: edit `crates/compiler/tests/parser.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-016-parser-assignment-statement-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-016-parser-assignment-statement-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/parser.rs`
- Implementation files:
  - `crates/compiler/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-016-parser-assignment-statement-metadata.md`
  - `docs/tasks/reviews/M0018-016-review.md`
  - `docs/tasks/soundness/M0018-016-soundness.md`

## Forbidden Changes

- Do not type assignment statements in this task.
- Do not add assignment target legality rules.
- Do not resolve names or infer types.
- Do not change expression grammar or recovery.
- Do not add AST child storage.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing parser or type-checking tests.

## Ambiguities And Dependencies

- Consuming assignment statement metadata in the type checker remains a later M0018 task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 parser assignment statement metadata task.
- 2026-07-10 main_task=main-task test work phase=test-first result=pass notes=`cargo test --workspace --all-targets` failed before implementation because `ParseOutput` had no `assignment_statements` field.
- 2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added parser assignment statement metadata with statement, target expression, and value expression nodes.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 153 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-016-parser-assignment-statement-metadata.md` created a passing soundness report after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved parser assignment metadata scope, spec compliance, and maintainability.
- 2026-07-10 main_task=main-task implementation phase=ci result=pass notes=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets`; `sh docs/tests/m0018-type-checking-core-accepted.sh`; `sh docs/tests/m0002-workspace-ci.sh`.
