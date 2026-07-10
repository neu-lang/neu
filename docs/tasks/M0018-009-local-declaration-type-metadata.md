# Task: M0018-009 Record Local Declaration Type Metadata

## Task Metadata

- Task ID: `M0018-009`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-009-local-declaration-type-metadata`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Record parser metadata connecting local variable declaration statement nodes to their explicit type annotation node and initializer expression node when present.

## Motivation

ADR-0027 includes local declarations with known explicit annotation types and assignment/declaration mismatch diagnostics. Existing parser metadata records local binding names and type/name references separately, but does not connect a variable declaration statement to its annotation or initializer. M0018 needs that relationship before declaration checking can be implemented safely.

## Scope

- Add parser metadata for local `val` and `var` declaration statements.
- Preserve the declaration statement node id.
- Preserve the local binding kind and name span through existing local binding metadata.
- Record optional explicit annotation type node id.
- Record optional initializer expression node id.
- Preserve source order for local declaration metadata.

## Out Of Scope

- Type resolution for annotation nodes.
- Literal, name, grouped, or assignment checking changes.
- Type mismatch diagnostics.
- Assignment compatibility.
- New parser syntax.
- Direct calls or function type application.
- Ownership, borrow checking, HIR, MIR, or backend behavior.

## Required Inputs

- Existing parser variable declaration statement parsing.
- Existing type AST nodes from M0012.
- Existing expression AST nodes from M0013 and literal metadata from M0018-008.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Parser records local declaration metadata for annotated initialized declarations.
  - Parser records local declaration metadata for unannotated initialized declarations.
  - Parser records local declaration metadata for annotated declarations without initializers.
  - Metadata references `AstNodeKind::VariableDeclarationStatement`.
  - Annotation metadata references the parsed type node.
  - Initializer metadata references the parsed expression node.
  - Metadata preserves source order.
- Negative tests:
  - Malformed declarations do not produce local declaration metadata.
  - Other statement kinds do not produce local declaration metadata.
- Diagnostic tests:
  - Existing parser diagnostics remain unchanged for malformed variable declarations.
- Adversarial tests:
  - Metadata does not perform type checking, assignment compatibility, conversions, ownership, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/parser.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `ParseOutput::local_declarations` and local declaration metadata records do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Track the `AstNodeId` returned by `parse_type` and `parse_expression` for variable declarations, then emit a parser metadata record after the declaration statement node is created. Avoid changing AST shape or type-checking behavior.

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
- [x] No compiler behavior beyond parser local declaration metadata is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/parser.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-009-local-declaration-type-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-009-local-declaration-type-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/parser.rs`
- Implementation files:
  - `crates/newlang/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-009-local-declaration-type-metadata.md`
  - `docs/tasks/reviews/M0018-009-review.md`
  - `docs/tasks/soundness/M0018-009-soundness.md`

## Forbidden Changes

- Do not implement type checking.
- Do not add assignment compatibility.
- Do not add type mismatch diagnostics.
- Do not add numeric conversion, width, signedness, layout, or backend behavior.
- Do not add direct call or function type application behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Type resolution of annotation nodes remains a later M0018 task.
- Declaration mismatch diagnostics remain a later M0018 task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 local declaration metadata task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added parser tests for local declaration annotation and initializer metadata before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed because `ParseOutput::local_declarations` does not exist yet.
- 2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass notes=Added parser local declaration metadata only; `cargo fmt --all --check`, `cargo test --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-009-local-declaration-type-metadata.md` created a passing soundness report; concrete adversarial review found no scope expansion.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-009-local-declaration-type-metadata.md` created review artifact; concrete review approved against SPEC, ADR-0027, and M0018.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=Final CI gate passed after review and soundness reports.
