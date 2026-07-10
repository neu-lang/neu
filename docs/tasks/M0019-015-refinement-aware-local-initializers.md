# Task: M0019-015 Add Refinement-Aware Local Initializer Checking

## Task Metadata

- Task ID: `M0019-015`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-015-refinement-aware-local-initializers`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0029-immutable-local-const-keyword.md`
  - `docs/adr/ADR-0030-local-initializer-nullable-diagnostic.md`
- Project Rules: `main task rules`
- main task Prompts: `main task rules`, `main task rules`, `main task rules`, `main task rules`, `main task rules`, `main task rules`, `main task rules`, `main task rules`

## Goal

Check annotated local `const` declaration initializers using valid per-use flow refinements, allowing a refined `T?` initializer to satisfy `T` while diagnosing an unrefined nullable initializer.

## Authority Extract

- `docs/SPEC.md`, “ADR-0027: Type Checking Core”: exact assignment compatibility; declaration annotation mismatches use the initializer span.
- `docs/SPEC.md`, “ADR-0028: Nullability And Flow Typing”: immutable-local refinements are per-use views; active branch refinements permit `T?` as `T`, while unrefined uses report `invalid_nullable_use`.
- `docs/SPEC.md`, “ADR-0029: Immutable Local `const` Keyword”: `const` is the immutable-local spelling and retains existing initializer semantics.
- `docs/adr/ADR-0030-local-initializer-nullable-diagnostic.md`, “Decision” and “Consequences And Limits”: only an exact bare resolved `T?` name initializer for annotated `T` uses `nullable_assignment_without_refinement`; `Null -> T` and unrelated mismatches remain `type_mismatch`.
- Validation: focused type-check test, task validator, adversarial check, review-task check, formatting, clippy, and workspace tests.

## Motivation

M0019-013 applies the accepted nullable-use rule to assignments, but local declarations must consume the same provenance-validated per-use view. ADR-0029 requires the initializer to use the migrated `const` spelling and existing immutable-local category.

## Scope

- Add local-initializer checking that consumes the exact initializer expression’s validated refined type.
- Preserve declaration signatures and original expression types as `T?`.
- Reuse accepted assignment compatibility and provenance, identity, consistency, and branch containment checks from M0019-013.
- Add focused positive, negative, diagnostic, adversarial tests, and a documentation validator before implementation.

## Out Of Scope

- Any lexer/parser/declaration-spelling migration; depends on M0019-014.
- Grouped-expression propagation, mutation invalidation, new mutable/exclusive-borrow eligibility, or flow orchestration.
- Changes to resolution, type representation, accepted semantics, diagnostic rendering, examples, or build configuration.

## Required Tests

Tests must be created before implementation.

- Positive: refined nullable local used as a `T` initializer inside its valid non-null branch; exact initialization, `Null -> T?`, and `T -> T?` remain accepted; original nullable signature/type entries remain unchanged.
- Negative: unrefined, after-branch, sibling-branch, shadowing, wrong-binding, inconsistent, duplicate, forged, and out-of-region refinements do not pass.
- Diagnostic: unrefined `T? -> T` reports `InvalidNullableUse` with `NullableAssignmentWithoutRefinement`, initializer span, expected `T`, and actual `T?`; `Null -> T` and unrelated nullable mismatches remain ordinary `TypeMismatch`.
- Adversarial: one initializer cannot consume another use’s refinement and no invalid provenance can satisfy the target.

## Test-First Gate

- Test files to create before implementation: `crates/compiler/tests/type_check.rs` and `docs/tests/m0019-refinement-aware-local-initializers.sh`.
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation: the local-initializer checker does not yet consume validated refined expression types.
- main-task review approval required to modify/delete failing tests: `yes`

## Dependencies And Blockers

- Revalidated dependency: M0019-014 is complete (commit `4d2a3ae`), including the ADR-0029 `const` migration gates.
- Resolved diagnostic mapping: accepted ADR-0030 (commit `2128abc`) requires `invalid_nullable_use` with stable rule identifier `nullable_assignment_without_refinement` for an exact bare resolved `T?` initializer assigned to annotated `T`. The primary span is the initializer; expected is `T`, actual is `T?`, with ADR-0028 recovery and suggestion policy. `Null -> T` and unrelated mismatches remain ordinary `type_mismatch`.

## Implementation Plan

Reuse the validated exact-expression compatibility path established by M0019-013, applying the refined view only to the initializer while preserving the declaration’s original nullable data. Keep ordinary mismatch behavior and accepted diagnostics unchanged outside the specified nullable-use case.

## Diagnostics, Build, And Reviews

- main-task diagnostics check confirms the identifier ambiguity is resolved before tests and checks spans and wording.
- main-task adversarial check checks provenance, identity, branch containment, and cross-use isolation.
- main-task review and main-task specification check verify scope and ADR-0028/0029 compliance.
- main-task build check verifies formatting, lint, workspace tests, task validator, and final CI.

## Acceptance Criteria

- [x] M0019-014 migration gates pass and task is revalidated.
- [x] Diagnostic-identifier ambiguity is resolved by accepted authority before tests.
- [x] Tests are written and fail before implementation.
- [x] Refined initializer succeeds without mutating original nullable records.
- [x] Negative, diagnostic, and adversarial tests pass.
- [x] Required reviews and CI pass.

## Execution Commands

- Generate tests: `cargo test -p compiler --test type_check m0019_refinement_aware_local_initializer`
- Verify tests fail: `cargo test -p compiler --test type_check m0019_refinement_aware_local_initializer` (expected pre-implementation failure)
- Ordinary tests: `cargo test -p compiler --test type_check m0019_refinement_aware_local_initializer && sh docs/tests/m0019-refinement-aware-local-initializers.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-015-refinement-aware-local-initializers.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-015-refinement-aware-local-initializers.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-refinement-aware-local-initializers.sh`

## Files Expected To Change

- Test files: `crates/compiler/tests/type_check.rs`, `docs/tests/m0019-refinement-aware-local-initializers.sh`.
- Implementation files: local-initializer/type-checking implementation only.
- Documentation: this task’s execution/review artifacts only, if required by the workflow.

## Forbidden Changes

- Do not edit `docs/SPEC.md`, `docs/adr/`, milestones, examples, or build files.
- Do not expand beyond the exact ADR-0030 initializer case or change accepted semantics.
- Do not weaken or delete failing tests without main-task review approval.

## Revalidation Log

- `main-task task planning` — after `4d2a3ae` and `2128abc`, dependency and diagnostic blocker cleared; authority extract and test-first commands refreshed; handoff to `main-task test work`.

## Execution Log

- 2026-07-10 main_task=main-task test work phase=test-first result=fail evidence=`cargo test -p compiler --test type_check m0019_refinement_aware_local_initializer` fails only because `type_m0019_local_declaration_initializers` is not implemented; `sh docs/tests/m0019-refinement-aware-local-initializers.sh` fails on the same missing API. next=main-task implementation
- 2026-07-10 main_task=Task-Decomposer phase=ordinary-tests result=pass evidence=`cargo test -p compiler --test type_check m0019_refinement_aware_local_initializer` passed (7); `sh docs/tests/m0019-refinement-aware-local-initializers.sh` passed; `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `git diff --check`, and `cargo test --workspace --all-targets` passed (214 tests). next=main-task review
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial result=pass evidence=`docs/scripts/adversarial-check.sh docs/tasks/M0019-015-refinement-aware-local-initializers.md` passed; cross-use and invalid-provenance checks hold. next=main-task review
- 2026-07-10 main_task=main-task review phase=final-review result=approve evidence=Diagnostics, Test, Spec Compliance, and Adversarial reviews approved; no findings. next=Build-Engineer
- 2026-07-10 main_task=Build-Engineer phase=final-ci result=pass evidence=CI passed 5/5: formatting, clippy, workspace tests (214), task validator, and diff check. next=Roadmap-Planner

This remains one M0019 milestone task for one problem: consuming an exact initializer use’s accepted refinement while preserving original nullable data.

## Handoff

- Next main task: `Roadmap-Planner`
- Reason: M0019-015 evidence is complete; continue with the next accepted M0019 work item according to milestone sequencing.
- Required Context: the accepted M0019 roadmap/task records and their stated Authority Extracts.
