# Task: M0018-012 Type Resolved Local Name Expressions

## Task Metadata

- Task ID: `M0018-012`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0018-012-resolved-local-name-expression-typing`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Type name expressions whose M0016 resolution points to a local binding with a known primitive type supplied to the type checker.

## Motivation

ADR-0027 includes name expressions whose resolution points to a binding with a known type. Existing M0018 work can type primitive declaration annotations and literal initializers, but name expressions still cannot receive expression type entries. This blocks non-literal initializer checks such as `val next: Int = count;`.

## Scope

- Add a type-check input record mapping resolved symbols to known type IDs.
- Consume M0016 `ResolutionTable` entries for name expressions.
- Record expression type entries for resolved name expressions with known symbols.
- Preserve resolution table order.
- Leave unresolved or unknown-typed names without synthesized expression types.

## Out Of Scope

- Changing name resolution.
- Inferring binding types from source order.
- Declaration initializer checks using name expressions.
- Assignment statements.
- Type mismatch diagnostics beyond existing behavior.
- Nominal lookup, generic solving, member lookup, calls, ownership, borrow checking, HIR, MIR, or backend behavior.

## Required Inputs

- M0016 `ResolutionTable`.
- Known type IDs from earlier M0018 primitive annotation/literal tasks.
- Existing expression type side table from M0018-006.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Resolved name expressions with known symbol types record expression type entries.
  - Multiple references to the same typed symbol receive the same type ID.
  - Expression type records preserve resolution table order.
- Negative tests:
  - Resolved names without known symbol types do not get synthesized expression types.
  - Missing resolution entries do not get synthesized expression types.
  - No declaration signatures or assignment checks are added.
- Diagnostic tests:
  - Existing ambiguous and type mismatch diagnostics continue to work.
- Adversarial tests:
  - Name expression typing does not implement lookup, inference, assignment statements, member lookup, calls, ownership, borrow checking, HIR, MIR, or backend behavior.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Resolved name expression typing entry point and known-symbol type input records do not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a minimal `KnownSymbolType` input and a function that reads `ResolutionTable::resolved_names()` in order, looks up each symbol in the known type list, and records expression type entries for matches only.

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
- [x] No compiler behavior beyond resolved name expression typing from explicit known symbol types is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-012-resolved-local-name-expression-typing.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-012-resolved-local-name-expression-typing.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-012-resolved-local-name-expression-typing.md`
  - `docs/tasks/reviews/M0018-012-review.md`
  - `docs/tasks/soundness/M0018-012-soundness.md`

## Forbidden Changes

- Do not change name-resolution binding semantics.
- Do not infer missing symbol types.
- Do not implement declaration initializer checks using name expressions in this task.
- Do not add member lookup, calls, ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Mapping declaration signatures into known symbol type inputs remains a later M0018 integration task.
- Name-based declaration initializer checks remain a later M0018 task.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0018 resolved local name expression typing task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Added resolved name expression typing tests before implementation.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed because `KnownSymbolType` and `type_resolved_name_expressions` do not exist yet.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=Added explicit known-symbol resolved name expression typing only; `cargo fmt --all --check`, `cargo test --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-012-resolved-local-name-expression-typing.md` created a passing soundness report; concrete adversarial review found no scope expansion.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=`docs/scripts/review-task.sh docs/tasks/M0018-012-resolved-local-name-expression-typing.md` created review artifact; concrete review approved against SPEC, ADR-0027, ADR-0026, and M0018.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Final CI gate passed after review and soundness reports.
