# Task: M0019-007 Add Parser Flow Metadata

## Task Metadata

- Task ID: `M0019-007`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-007-parser-flow-metadata`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Record parser metadata needed by M0019 flow analysis: binary expression operands/operators and `if` condition/branch nodes.

## Motivation

ADR-0028 null-test recognition needs direct equality comparisons between a local name expression and `null`, and branch region boundaries from `if` expressions. The parser currently creates AST nodes for binary and `if` expressions but does not expose side-table metadata that later flow tasks can consume.

## Scope

- Add parsed binary expression metadata with expression node, left operand, right operand, operator, and span.
- Add parsed `if` expression metadata with expression node, condition, then block, optional else block, and span.
- Expose metadata in `ParseOutput`.
- Add parser tests and a docs validator.

## Out Of Scope

- Recognizing null tests semantically.
- Type checking binary expressions.
- Applying smart casts.
- Nullable misuse diagnostics.
- Flow refinement tracking.
- Changing accepted grammar.
- Updating examples; this is parser metadata only.

## Required Inputs

- Accepted ADR-0028 null-test and branch-region requirements.
- ADR-0024 parser grammar.
- Existing parser side-table patterns for grouped expressions and assignments.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Binary metadata records equality operator, left operand, right operand, expression node, and span.
  - `if` metadata records condition, then block, and else block.
  - `if` metadata records missing else as `None`.
- Negative tests:
  - Parser metadata does not type check binary expressions.
  - Parser metadata does not recognize null tests or apply smart casts.
- Diagnostic tests:
  - Malformed condition behavior remains covered by existing parser diagnostics.
- Adversarial tests:
  - Metadata is syntax-only and does not infer flow facts.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0019-parser-flow-metadata.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Parser flow metadata fields and structs do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Follow existing parser side-table patterns. Record binary expression metadata during binary parse and `if` metadata after condition and branch blocks are parsed.

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
- [x] Examples decision is recorded.

## Execution Commands

- Generate tests: `edit crates/newlang/tests/parser.rs and create docs/tests/m0019-parser-flow-metadata.sh`
- Verify tests fail: `cargo test -p newlang --test parser m0019`
- Ordinary tests: `cargo test -p newlang --test parser && sh docs/tests/m0019-parser-flow-metadata.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-007-parser-flow-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-007-parser-flow-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-parser-flow-metadata.sh && sh docs/tests/m0019-flow-output-data-model.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0019-parser-flow-metadata.sh`
- Implementation files:
  - `crates/newlang/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0019-007-parser-flow-metadata.md`
  - `docs/tasks/reviews/M0019-007-review.md`
  - `docs/tasks/soundness/M0019-007-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not implement null-test recognition.
- Do not type check binary expressions.
- Do not apply smart casts.
- Do not change grammar.

## Ambiguities And Dependencies

- Later M0019 tasks will consume this metadata to recognize null tests and branch regions.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0019 parser flow metadata task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added parser metadata tests and docs validator before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test -p newlang --test parser m0019` failed because ParsedBinaryOperator and parser flow metadata fields did not exist.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=Added syntax-only parser metadata for binary expressions and if condition/branch nodes; `cargo test -p newlang --test parser`, `cargo test -p newlang --test parser m0019`, and `sh docs/tests/m0019-parser-flow-metadata.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0019-007-parser-flow-metadata.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0019-007-parser-flow-metadata.md` created review report; concrete review approved syntax-metadata-only scope.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0019-parser-flow-metadata.sh`, `sh docs/tests/m0019-flow-output-data-model.sh`, M0019 accepted validator chain, and `sh docs/tests/m0002-workspace-ci.sh` passed. Examples skipped because this task adds parser metadata only.

## Handoff

- Next main task: `main-task test work`
- Reason: `Verify tests fail before implementation.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `crates/newlang/src/parser.rs`
