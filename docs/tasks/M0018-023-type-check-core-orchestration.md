# Task: M0018-023 Type Check Core Orchestration

## Task Metadata

- Task ID: `M0018-023`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0018-023-type-check-core-orchestration`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Compose existing M0018 type-checking pieces into one core report helper for accepted fixtures.

## Motivation

M0018 now has separate helpers for primitive declaration signatures, literal/name/grouped expression typing, local initializer checks, assignment checks, unresolved diagnostics, and unsupported diagnostics. The milestone deliverable is a type checker core with positive and negative fixtures, so implementation agents need one orchestration helper that produces the M0018 side-table report from accepted inputs without running earlier phases.

## Scope

- Consume parser output pieces, an AST arena, already-resolved names, and already-built local bindings.
- Record primitive declaration signatures and unresolved annotation diagnostics.
- Derive known local symbol types from local bindings and declaration signatures.
- Type accepted expressions: literals, resolved names with known types, and grouped expressions.
- Check local declaration initializers and assignment statements using ADR-0027 assignment compatibility.
- Record unsupported expression diagnostics from existing AST nodes.
- Return one `TypeArena` and one `TypeCheckReport`.

## Out Of Scope

- Running parsing, name resolution, module construction, or local binding construction.
- Inferring types.
- Type checking unsupported expressions.
- Function call checking or function type application.
- Ownership, borrow, HIR, MIR, or backend behavior.
- Changing source-language syntax or examples.

## Required Inputs

- `AstArena`
- `ParsedLocalDeclaration` records.
- `ParsedTypeNameReference` records.
- `ParsedLiteralExpression` records.
- `ParsedGroupedExpression` records.
- `ParsedAssignmentStatement` records.
- `ResolutionTable`
- `LocalBinding` records.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - A well-typed accepted fixture produces declaration signatures, expression types, initializer assignment checks, and assignment-statement checks with no diagnostics.
  - Known local symbol types are derived from explicit declaration signatures and used for resolved name expressions.
- Negative tests:
  - Mismatched local initializer and assignment statement values produce `type_mismatch`.
  - Missing annotation authority produces `missing_annotation_type`.
  - Resolved names without known local type metadata produce `missing_resolved_name_type`.
  - Unsupported expression nodes produce `unsupported_type_rule`.
- Adversarial tests:
  - The orchestration helper does not run name resolution, infer types, or type unsupported expressions.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - M0018 orchestration helper does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a helper that creates primitive type identities once, builds declaration signatures, derives known local symbol types, records accepted expression types, checks local initializer and assignment compatibility, records unresolved diagnostics, records unsupported diagnostics, and returns the merged report.

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
- [x] No compiler behavior beyond M0018 orchestration is introduced.
- [x] Examples update is explicitly skipped because no language-level source forms changed.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-023-type-check-core-orchestration.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-023-type-check-core-orchestration.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-023-type-check-core-orchestration.md`
  - `docs/tasks/reviews/M0018-023-review.md`
  - `docs/tasks/soundness/M0018-023-soundness.md`

## Forbidden Changes

- Do not run name resolution or construct local bindings in the type checker.
- Do not infer types.
- Do not type unsupported expression forms.
- Do not modify examples for this task.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Whole-compiler driver integration remains a later milestone task.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0018 type-check core orchestration task.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Added orchestration tests before implementation for well-typed accepted fixtures and mixed mismatch, unresolved, and unsupported diagnostics.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed before implementation with unresolved import `type_m0018_core`.
- 2026-07-10 agent=Implementer phase=implementation result=pass notes=Added `type_m0018_core` orchestration helper that composes existing M0018 report pieces without running parsing or name resolution.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 171 tests.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-023-type-check-core-orchestration.md` passed after ordinary tests.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=Review approved against `docs/SPEC.md`, ADR-0027, ADR-0026, ADR-0024, and `docs/milestones/M0018-type-checking-core.md`.
- 2026-07-10 agent=Examples-Curator phase=examples result=skip notes=No example update required because this task composes existing supported behavior and diagnostics without changing source forms users can write.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
