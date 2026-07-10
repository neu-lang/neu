# Task: M0018-021 Unresolved Type Rule Diagnostics

## Task Metadata

- Task ID: `M0018-021`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task diagnostics check`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-021-unresolved-type-rule-diagnostics`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Emit ADR-0027 `unresolved_type_rule` diagnostics for accepted M0018 constructs whose required type authority is missing.

## Motivation

M0018-020 added the diagnostic representation. The checker still silently skips known unresolved cases: local declarations without accepted primitive annotation authority and resolved name expressions whose symbol lacks known type metadata. ADR-0027 requires diagnostics and no successful table entry when accepted constructs cannot be typed.

## Scope

- Emit `missing_annotation_type` for local declarations whose annotation is absent, missing from type metadata, or not an accepted primitive type.
- Emit `missing_resolved_name_type` for resolved name expressions whose symbol has no supplied known type.
- Preserve successful expression types, declaration signatures, assignment checks, and type mismatch behavior for typed constructs.
- Keep existing helper APIs focused on accepted M0018 metadata.

## Out Of Scope

- Emitting unsupported diagnostics for calls, members, binary expressions, unary expressions, value-producing `if`, or block values.
- Running name resolution.
- Deriving known symbol types.
- Inferring missing declaration types.
- Changing parser metadata.
- Ownership, borrow, HIR, MIR, or backend behavior.

## Required Inputs

- `ParsedLocalDeclaration` records.
- `ParsedTypeNameReference` records.
- `ResolutionTable` records.
- `KnownSymbolType` records.
- ADR-0027 unresolved diagnostic rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Local declarations missing accepted annotation authority emit `missing_annotation_type`.
  - Resolved names without known symbol types emit `missing_resolved_name_type`.
  - Typed local declarations and typed resolved names still produce successful output.
- Negative tests:
  - Missing annotation authority does not produce declaration signatures or assignment checks.
  - Missing resolved name type does not produce an expression type.
  - Type mismatch diagnostics remain distinct from unresolved diagnostics.
- Adversarial tests:
  - The helper does not infer annotation types, run name resolution, derive known symbol types, or type unsupported expressions.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/compiler/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Existing helpers skip missing annotation authority and missing resolved-name types without diagnostics.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add unresolved diagnostic recording in the existing accepted M0018 helper paths that already encounter missing annotation authority and missing known symbol types.

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
- [x] No compiler behavior beyond unresolved diagnostic emission for accepted M0018 constructs is introduced.

## Execution Commands

- Generate tests: edit `crates/compiler/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-021-unresolved-type-rule-diagnostics.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-021-unresolved-type-rule-diagnostics.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/type_check.rs`
- Implementation files:
  - `crates/compiler/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-021-unresolved-type-rule-diagnostics.md`
  - `docs/tasks/reviews/M0018-021-review.md`
  - `docs/tasks/soundness/M0018-021-soundness.md`

## Forbidden Changes

- Do not infer types.
- Do not run name resolution or derive known symbol types.
- Do not type unsupported expression forms.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Unsupported expression traversal remains a later task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 unresolved type-rule diagnostic task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated accepted M0018 tests to require `missing_annotation_type` and `missing_resolved_name_type` diagnostics before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed before implementation with four missing-diagnostic assertion failures.
- 2026-07-10 main_task=Diagnostics-Engineer phase=implementation result=pass notes=Recorded unresolved diagnostics for missing primitive annotation authority and resolved symbols without supplied known types.
- 2026-07-10 main_task=Diagnostics-Engineer phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 166 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-021-unresolved-type-rule-diagnostics.md` passed after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved against `docs/SPEC.md`, ADR-0027, ADR-0026, and `docs/milestones/M0018-type-checking-core.md`.
- 2026-07-10 main_task=Examples-Curator phase=examples result=skip notes=No example update required because this task changes diagnostic behavior, not the language-level source forms users can write.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
