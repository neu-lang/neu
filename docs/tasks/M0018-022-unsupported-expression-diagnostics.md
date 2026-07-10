# Task: M0018-022 Unsupported Expression Diagnostics

## Task Metadata

- Task ID: `M0018-022`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task diagnostics check`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-022-unsupported-expression-diagnostics`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Emit ADR-0027 `unsupported_type_rule` diagnostics for parsed expression forms excluded from M0018 type checking.

## Motivation

ADR-0027 requires excluded expression forms to report `unsupported_type_rule` with stable identifiers rather than being guessed or silently accepted. M0018-020 added the diagnostic representation, but no helper reports unsupported expression AST nodes yet.

## Scope

- Consume existing AST nodes.
- Report unsupported diagnostics for `CallExpression`, `MemberExpression`, `BinaryExpression`, `UnaryExpression`, and value-producing `IfExpression` nodes.
- Preserve typed output side-table behavior: unsupported nodes receive diagnostics and no successful expression type entries.
- Avoid changing parser metadata.

## Out Of Scope

- Type checking unsupported expression forms.
- Distinguishing direct function declaration calls from structural function type application by callee type.
- Emitting diagnostics for type forms, declarations, statements, patterns, ownership, borrow, HIR, MIR, or backend behavior.
- Running name resolution or inference.
- Changing examples, because this task changes diagnostics only.

## Required Inputs

- `AstArena` node list.
- ADR-0027 excluded expression forms and stable rule identifiers.
- Existing `TypeCheckDiagnostic` model.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Parsed call expressions report `direct_call_deferred`.
  - Parsed member expressions report `member_expression_deferred`.
  - Parsed binary expressions report `binary_expression_deferred`.
  - Parsed `if` expressions report `if_value_deferred`.
- Negative tests:
  - Literal, name, grouped, block, declaration, and statement nodes do not receive unsupported expression diagnostics.
  - Unsupported diagnostics do not produce expression types, declaration signatures, or assignment checks.
- Adversarial tests:
  - The helper does not infer, resolve, type, lower, or execute unsupported expressions.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/compiler/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Unsupported expression diagnostic helper does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a helper that scans the existing AST arena and records unsupported type-rule diagnostics for ADR-0027 excluded expression node kinds.

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
- [x] No compiler behavior beyond unsupported expression diagnostic emission is introduced.
- [x] Examples update is explicitly skipped because no language-level source forms changed.

## Execution Commands

- Generate tests: edit `crates/compiler/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-022-unsupported-expression-diagnostics.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-022-unsupported-expression-diagnostics.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/type_check.rs`
- Implementation files:
  - `crates/compiler/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-022-unsupported-expression-diagnostics.md`
  - `docs/tasks/reviews/M0018-022-review.md`
  - `docs/tasks/soundness/M0018-022-soundness.md`

## Forbidden Changes

- Do not type unsupported expression forms.
- Do not alter parser output.
- Do not infer types or run name resolution.
- Do not add HIR, MIR, backend, ownership, or borrow behavior.
- Do not weaken or delete existing tests.

## Ambiguities And Dependencies

- `function_type_application_deferred` requires callee type authority and remains out of scope for this task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 unsupported expression diagnostics task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added unsupported expression diagnostic tests before implementation for call, member, binary, unary, and if expression nodes.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed before implementation with unresolved import `type_unsupported_m0018_expressions`.
- 2026-07-10 main_task=Diagnostics-Engineer phase=implementation result=pass notes=Added AST-scanning helper that reports ADR-0027 unsupported expression diagnostics without producing successful type entries.
- 2026-07-10 main_task=Diagnostics-Engineer phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 169 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-022-unsupported-expression-diagnostics.md` passed after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved against `docs/SPEC.md`, ADR-0027, ADR-0024, and `docs/milestones/M0018-type-checking-core.md`.
- 2026-07-10 main_task=Examples-Curator phase=examples result=skip notes=No example update required because this task changes diagnostics for already parsed source forms, not language-level source forms.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
