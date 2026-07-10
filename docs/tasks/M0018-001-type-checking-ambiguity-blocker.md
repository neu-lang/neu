# Task: M0018-001 Block Ambiguous Type Checking Rules

## Task Metadata

- Task ID: `M0018-001`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task language review`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-001-type-checking-ambiguity-blocker`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Record and block ambiguous type checking rules before implementing any expression or declaration checking.

## Motivation

M0018 requires ambiguous typing rules to be recorded and not implemented. The milestone explicitly identifies literal typing and call syntax or function type rules as risks, and M0017 left primitive scalar categories unresolved.

## Scope

- File an ambiguity report for M0018 type checking rules.
- Add a type-checking diagnostic model for ambiguous rules.
- Cover literal typing, primitive scalar categories, assignment compatibility, call resolution, and function type application as blocked rule categories.
- Add tests proving ambiguous rules produce diagnostics and no typed program result.
- Add a validator for this task.
- Mark the M0018 ambiguous-rule checklist item complete if satisfied.

## Out Of Scope

- Implementing type checking.
- Assigning types to literals.
- Assignment compatibility.
- Call resolution.
- Function type application.
- Primitive scalar catalog decisions.
- Flow typing, ownership, borrow checking, HIR, MIR, or backend behavior.

## Required Inputs

- Milestone: `docs/milestones/M0018-type-checking-core.md`
- Type representation from M0017.
- M0018 risk notes.
- ADR-0010 type system shape.
- ADR-0015 diagnostic obligations.
- ADR-0023 type syntax deferrals.
- ADR-0024 expression syntax, which parses bodies but defers type semantics.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Ambiguous type rule diagnostics preserve the rule and AST node.
  - The blocked rule categories cover M0018 and M0017 known risks.
  - Blocking diagnostics are separate from successful type checking output.
- Negative tests:
  - Focused Rust tests fail before implementation because `newlang::type_check` does not exist.
- Diagnostic tests:
  - Unit tests check diagnostic kind, blocked rule, and primary AST node.
- Adversarial tests:
  - Confirm this task does not implement type inference, literal typing, call resolution, assignment compatibility, flow typing, ownership, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `newlang::type_check` does not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a diagnostic-only `type_check` module with `AmbiguousTypeRule`, `TypeCheckDiagnosticKind`, `TypeCheckDiagnostic`, and `TypeCheckReport`. The module must not assign types, resolve calls, or accept/reject expressions as well-typed.

## Acceptance Criteria

- [ ] Task references exactly one milestone.
- [ ] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] M0018 ambiguous-rule checklist is updated.

## Execution Commands

- Generate tests: `create crates/newlang/tests/type_check.rs docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Verify tests fail: `cargo test -p newlang --test type_check`
- Ordinary tests: `cargo test -p newlang --test type_check && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-001-type-checking-ambiguity-blocker.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-001-type-checking-ambiguity-blocker.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/src/lib.rs`
- Documentation or checklist files:
  - `docs/ambiguities/M0018-type-checking-core.md`
  - `docs/milestones/M0018-type-checking-core.md`
  - `docs/tasks/M0018-001-type-checking-ambiguity-blocker.md`
  - `docs/tasks/reviews/M0018-001-review.md`
  - `docs/tasks/soundness/M0018-001-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not implement type checking.
- Do not assign types to literals.
- Do not implement call or assignment checking.
- Do not weaken or delete failing tests without main-task review approval.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- Literal typing and overload rules are unspecified.
- Primitive scalar categories are unresolved.
- Assignment compatibility and call resolution require accepted rules before implementation.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Task references only M0018 and blocks ambiguous type rules before checking implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test -p newlang --test type_check` failed because `newlang::type_check` did not exist.
- 2026-07-10 main_task=Language-Lawyer phase=ordinary-tests result=pass notes=`cargo test -p newlang --test type_check` passed after adding diagnostic-only ambiguous type rule blocking; validator initially failed only because the milestone checklist had not yet been updated.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-001-type-checking-ambiguity-blocker.md` created a passing soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-001-type-checking-ambiguity-blocker.md` created review and concrete review approved after source-of-truth comparison.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`, `sh docs/tests/m0017-unsupported-type-form-blocking.sh`, `sh docs/tests/m0016-name-resolution-data-model.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
