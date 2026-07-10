# Task: M0019-015 Add Refinement-Aware Local Initializer Checking

## Task Metadata

- Task ID: `M0019-015`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `paused`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0019-015-refinement-aware-local-initializers`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0029-immutable-local-const-keyword.md`
- Project Rules: `AGENTS.md`
- Agent Prompts: `.codex/agents/task-decomposer.toml`, `.codex/agents/test-engineer.toml`, `.codex/agents/implementer.toml`, `.codex/agents/diagnostics-engineer.toml`, `.codex/agents/adversarial-engineer.toml`, `.codex/agents/reviewer.toml`, `.codex/agents/spec-compliance-auditor.toml`, `.codex/agents/build-engineer.toml`

## Goal

Check annotated local `const` declaration initializers using valid per-use flow refinements, allowing a refined `T?` initializer to satisfy `T` while diagnosing an unrefined nullable initializer.

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

- Test files to create before implementation: `crates/newlang/tests/type_check.rs` and `docs/tests/m0019-refinement-aware-local-initializers.sh`.
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation: the local-initializer checker does not yet consume validated refined expression types.
- Reviewer approval required to modify/delete failing tests: `yes`

## Dependencies And Blockers

- Hard dependency: M0019-014 must complete its ADR-0029 migration gates and be revalidated by Task Decomposer and Roadmap Planner.
- Blocked ambiguity: the diagnostic identifier for this initializer path must be confirmed against accepted ADR-0028/diagnostic authority before tests encode it. Do not guess or add a new identifier in this task.
- Until both conditions are resolved, this task remains paused and no implementation or tests should proceed.

## Implementation Plan

Reuse the validated exact-expression compatibility path established by M0019-013, applying the refined view only to the initializer while preserving the declaration’s original nullable data. Keep ordinary mismatch behavior and accepted diagnostics unchanged outside the specified nullable-use case.

## Diagnostics, Build, And Reviews

- Diagnostics Engineer confirms the identifier ambiguity is resolved before tests and checks spans and wording.
- Adversarial Engineer checks provenance, identity, branch containment, and cross-use isolation.
- Reviewer and Spec Compliance Auditor verify scope and ADR-0028/0029 compliance.
- Build Engineer verifies formatting, lint, workspace tests, task validator, and final CI.

## Acceptance Criteria

- [ ] M0019-014 migration gates pass and task is revalidated.
- [ ] Diagnostic-identifier ambiguity is resolved by accepted authority before tests.
- [ ] Tests are written and fail before implementation.
- [ ] Refined initializer succeeds without mutating original nullable records.
- [ ] Negative, diagnostic, and adversarial tests pass.
- [ ] Required reviews and CI pass.

## Execution Commands

- Generate tests: `blocked: M0019-014 migration and diagnostic-identifier ambiguity must be resolved`
- Verify tests fail: `blocked: M0019-014 migration and diagnostic-identifier ambiguity must be resolved`
- Ordinary tests: `cargo test -p newlang --test type_check m0019_refinement_aware_local_initializer && sh docs/tests/m0019-refinement-aware-local-initializers.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-015-refinement-aware-local-initializers.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-015-refinement-aware-local-initializers.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-refinement-aware-local-initializers.sh`

## Files Expected To Change

- Test files: `crates/newlang/tests/type_check.rs`, `docs/tests/m0019-refinement-aware-local-initializers.sh`.
- Implementation files: local-initializer/type-checking implementation only.
- Documentation: this task’s execution/review artifacts only, if required by the workflow.

## Forbidden Changes

- Do not edit `docs/SPEC.md`, `docs/adr/`, milestones, examples, or build files.
- Do not proceed while M0019-014 or the diagnostic-identifier ambiguity is unresolved.
- Do not weaken or delete failing tests without reviewer approval.

## Handoff

- Next Agent: `Task Decomposer` then `Test Engineer`
- Reason: revalidate the task after M0019-014 and diagnostic authority unblock it, then create tests first.
- Required Context: this task, M0019-014, `AGENTS.md`, `docs/SPEC.md`, ADR-0027, ADR-0028, ADR-0029, M0019 milestone, and M0019-013.
