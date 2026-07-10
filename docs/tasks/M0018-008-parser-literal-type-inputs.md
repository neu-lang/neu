# Task: M0018-008 Map Parser Literal Nodes To Type Inputs

## Task Metadata

- Task ID: `M0018-008`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0018-008-parser-literal-type-inputs`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Record parser literal expression metadata and feed it into M0018 literal typing without changing parser syntax or adding unsupported type-checking behavior.

## Motivation

M0018-007 added literal typing for already-classified literal inputs. To make well-typed literal fixtures meaningful, the parser must preserve literal category metadata for literal AST nodes so the type checker can produce expression type side-table entries from real parsed programs.

## Scope

- Add parser metadata records for literal expression AST nodes.
- Preserve literal category for `true`, `false`, accepted integer literals, accepted string literals, and `null`.
- Preserve literal token span on the metadata record.
- Add a type-check entry point that accepts parser literal metadata and records literal expression types.
- Keep expression type entries in parser encounter order.

## Out Of Scope

- New parser syntax.
- Literal validation beyond lexer-accepted token kinds.
- Numeric overflow, width, signedness, layout, or backend semantics.
- Name expression typing.
- Grouped expression typing.
- Assignment compatibility.
- Type mismatch diagnostics.
- Direct calls or function type application.
- Ownership, borrow checking, HIR, MIR, or backend behavior.

## Required Inputs

- Parser literal expression AST nodes from ADR-0024 syntax.
- `LiteralExpressionInput` and `type_literal_expressions` from M0018-007.
- Primitive type-checking identities from M0018-006.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Parser records literal expression metadata for bool, integer, string, and null literals.
  - Metadata references `AstNodeKind::LiteralExpression` nodes.
  - Metadata preserves source span text.
  - Type checker maps parser literal metadata to ADR-0027 primitive expression types.
- Negative tests:
  - Parser does not record non-literal expressions as literal metadata.
  - Type checker does not synthesize entries for missing parser literal metadata.
- Diagnostic tests:
  - Existing ambiguous blocker diagnostics continue to work.
- Adversarial tests:
  - Parser mapping does not introduce assignment compatibility, conversions, calls, ownership, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/parser.rs`
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Parser literal expression metadata and parser-metadata type-check entry point do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend parser output with a literal metadata vector and map accepted literal token kinds into the existing `LiteralKind` model. Add a small type-check wrapper that converts parser metadata into literal expression inputs and reuses M0018-007 literal typing.

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
- [x] No compiler behavior beyond parser literal metadata and literal expression typing input mapping is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/parser.rs` and `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-008-parser-literal-type-inputs.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-008-parser-literal-type-inputs.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/parser.rs`
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-008-parser-literal-type-inputs.md`
  - `docs/tasks/reviews/M0018-008-review.md`
  - `docs/tasks/soundness/M0018-008-soundness.md`

## Forbidden Changes

- Do not add new syntax.
- Do not add assignment compatibility.
- Do not add type mismatch diagnostics.
- Do not add numeric conversion, width, signedness, layout, or backend behavior.
- Do not add direct call or function type application behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Assignment compatibility and type mismatch diagnostics remain later M0018 tasks.
- Name expression and grouped expression typing remain later M0018 tasks.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0018 parser literal metadata to type-check input task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Added parser literal metadata and parser-metadata literal typing tests before implementation.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed because `ParsedLiteralKind`, `ParseOutput::literal_expressions`, and `type_parser_literals` do not exist yet.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=Added parser literal metadata and parser-metadata literal typing wrapper only; `cargo fmt --all --check`, `cargo test --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-008-parser-literal-type-inputs.md` created a passing soundness report; concrete adversarial review found no scope expansion.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-008-parser-literal-type-inputs.md` created review artifact; concrete review approved against SPEC, ADR-0027, and M0018.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Final CI gate passed after review and soundness reports.
