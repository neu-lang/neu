# Task: M0018-010 Resolve Primitive Local Declaration Signatures

## Task Metadata

- Task ID: `M0018-010`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0018-010-primitive-local-declaration-signatures`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Resolve explicitly annotated local declarations whose annotation is an ADR-0027 primitive type name and record declaration signatures in the type-check report.

## Motivation

M0018 declaration initializer checks require known declaration target types. M0018-009 connected local declarations to annotation nodes; this task turns the accepted primitive annotation subset into declaration signature side-table entries without adding mismatch checking yet.

## Scope

- Accept parser local declaration metadata and parser type-name metadata.
- Resolve annotation names `Bool`, `Int`, `String`, `Unit`, and `Null` to primitive type-checking identities.
- Record declaration signature entries keyed by local declaration statement node.
- Preserve parser local declaration order for signature records.
- Leave unannotated and non-primitive annotations without synthesized signatures.

## Out Of Scope

- Initializer expression checks.
- Assignment compatibility.
- Type mismatch diagnostics.
- Nullable annotation resolution.
- Nominal type resolution.
- Generic solving.
- New parser syntax.
- Direct calls or function type application.
- Ownership, borrow checking, HIR, MIR, or backend behavior.

## Required Inputs

- Parser local declaration metadata from M0018-009.
- Parser type-name metadata from earlier parser milestones.
- Primitive type identities from M0018-006.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Primitive annotations produce declaration signatures.
  - `Bool`, `Int`, `String`, `Unit`, and `Null` map to the matching primitive type identities.
  - Declaration signatures preserve local declaration order.
- Negative tests:
  - Unannotated declarations do not get synthesized signatures.
  - Non-primitive annotations do not get synthesized signatures.
  - Initializer expression types and assignment checks are not added by this task.
- Diagnostic tests:
  - Existing ambiguous blocker diagnostics continue to work.
- Adversarial tests:
  - Primitive annotation resolution does not implement assignment compatibility, mismatch diagnostics, nominal lookup, generic solving, ownership, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Primitive local declaration signature entry point does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a small type-check entry point that consumes parser local declaration metadata plus parser type-name metadata, creates the standard primitive type arena, and records declaration signatures for annotations that exactly match accepted primitive names.

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
- [x] No compiler behavior beyond primitive local declaration signature recording is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-010-primitive-local-declaration-signatures.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-010-primitive-local-declaration-signatures.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-010-primitive-local-declaration-signatures.md`
  - `docs/tasks/reviews/M0018-010-review.md`
  - `docs/tasks/soundness/M0018-010-soundness.md`

## Forbidden Changes

- Do not implement initializer or assignment checking.
- Do not add type mismatch diagnostics.
- Do not resolve nominal or generic annotations.
- Do not add nullable annotation resolution.
- Do not add direct call or function type application behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Nullable annotation resolution remains a later M0018 task.
- Type mismatch diagnostics remain a later M0018 task.
- Nominal type annotation resolution remains outside this task unless accepted by later source of truth.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0018 primitive local declaration signature task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Added parser-driven primitive local declaration signature tests before implementation.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed because `type_primitive_local_declarations` does not exist yet.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=Added primitive local declaration signature resolver only; `cargo fmt --all --check`, `cargo test --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-010-primitive-local-declaration-signatures.md` created a passing soundness report; concrete adversarial review found no scope expansion.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-010-primitive-local-declaration-signatures.md` created review artifact; concrete review approved against SPEC, ADR-0027, and M0018.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Final CI gate passed after review and soundness reports.
