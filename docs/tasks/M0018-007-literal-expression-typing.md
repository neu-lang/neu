# Task: M0018-007 Add Literal Expression Typing

## Task Metadata

- Task ID: `M0018-007`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0018-007-literal-expression-typing`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Type accepted literal expression nodes to ADR-0027 primitive type-checking identities and record the results in the expression type side table.

## Motivation

M0018 requires approved well-typed fixtures to pass type checking. Literal typing is the first well-typed expression behavior authorized by ADR-0027 and uses the primitive identities and output tables from M0018-006.

## Scope

- Add a literal expression input record keyed by `AstNodeId`.
- Support boolean, accepted integer, accepted string, and null literal categories.
- Type `true` and `false` literals to `Bool`.
- Type accepted integer literals to `Int`.
- Type accepted string literals to `String`.
- Type `null` literals to `Null`.
- Record expression type entries in insertion order.

## Out Of Scope

- Parser changes.
- Literal text parsing or validation beyond accepted lexer categories.
- Numeric overflow, width, signedness, or layout.
- String escape interpretation or runtime representation.
- Name expression typing.
- Grouped expression typing.
- Assignment compatibility.
- Type mismatch diagnostics.
- Direct calls or function type application.
- Ownership, borrow checking, HIR, MIR, or backend behavior.

## Required Inputs

- `PrimitiveType` and primitive `TypeRecord`s from M0018-006.
- `TypeCheckReport` expression type table from M0018-006.
- Literal expression AST node identities from existing parser/AST stages.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Boolean literal inputs produce `Bool` expression types.
  - Integer literal inputs produce `Int` expression types.
  - String literal inputs produce `String` expression types.
  - Null literal inputs produce `Null` expression types.
  - Expression type records preserve literal input order.
- Negative tests:
  - Literal typing does not add declaration signatures or assignment checks.
  - Literal typing does not infer missing literal inputs from unrelated AST nodes.
- Diagnostic tests:
  - Existing ambiguous blocker diagnostics continue to work.
- Adversarial tests:
  - Literal typing does not introduce numeric width, signedness, layout, conversions, assignment compatibility, calls, ownership, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Literal input records and literal typing entry points do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add the smallest literal typing API that accepts already-classified literal records and writes expression type side-table entries. Keep parser-to-type-check integration for a later task so this slice does not invent AST payload semantics.

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
- [x] No compiler behavior beyond accepted literal expression typing is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-007-literal-expression-typing.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-007-literal-expression-typing.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-007-literal-expression-typing.md`
  - `docs/tasks/reviews/M0018-007-review.md`
  - `docs/tasks/soundness/M0018-007-soundness.md`

## Forbidden Changes

- Do not modify parser behavior.
- Do not add assignment compatibility.
- Do not add type mismatch diagnostics.
- Do not add numeric conversion, width, signedness, layout, or backend behavior.
- Do not add direct call or function type application behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Parser integration depends on a later task that maps parser literal nodes into literal input records.
- Assignment compatibility and type mismatch diagnostics remain later M0018 tasks.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0018 literal expression typing task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Added literal expression typing tests before implementation.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed because `type_literal_expressions`, `LiteralExpressionInput`, and `LiteralKind` do not exist yet.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=Added classified literal input records and literal expression typing to ADR-0027 primitive identities only; `cargo fmt --all --check`, `cargo test --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-007-literal-expression-typing.md` created a passing soundness report; concrete adversarial review found no scope expansion.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-007-literal-expression-typing.md` created review artifact; concrete review approved against SPEC, ADR-0027, and M0018.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Final CI gate passed after review and soundness reports.
