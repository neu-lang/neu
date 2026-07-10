# Task: M0018-013 Derive Known Local Symbol Types

## Task Metadata

- Task ID: `M0018-013`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0018-013-known-local-symbol-types`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Derive explicit `KnownSymbolType` inputs from resolved local binding records whose declaration statement has a known declaration signature.

## Motivation

M0018-012 types resolved name expressions from explicit known symbol type inputs. M0018-010 records declaration signatures for primitive local declarations. This task bridges those accepted tables so later tasks can type name initializers from actual local declaration signatures without inventing inference.

## Scope

- Consume M0016 local binding records.
- Consume M0018 declaration signature side-table records.
- Emit `KnownSymbolType` records for local bindings whose declaration node has a signature.
- Preserve local binding order.
- Skip local bindings without declaration signatures.

## Out Of Scope

- Running name resolution.
- Inferring declaration signatures.
- Typing name expressions.
- Declaration initializer checks using name expressions.
- Assignment statements.
- Type mismatch diagnostics beyond existing behavior.
- Nominal lookup, generic solving, member lookup, calls, ownership, borrow checking, HIR, MIR, or backend behavior.

## Required Inputs

- M0016 `LocalBinding` records.
- M0018 declaration signatures.
- Existing `KnownSymbolType` type-check input records.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Local bindings with matching declaration signatures produce known symbol type inputs.
  - Multiple typed bindings preserve local binding order.
  - The emitted symbol IDs come from the local binding keys.
- Negative tests:
  - Local bindings without declaration signatures are skipped.
  - Declaration signatures without local bindings do not synthesize symbol types.
  - No expression types, assignment checks, or diagnostics are added.
- Diagnostic tests:
  - Existing type mismatch diagnostics continue to work.
- Adversarial tests:
  - Known symbol derivation does not infer types, run lookup, type name expressions, check assignments, or add backend/safety behavior.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Known local symbol derivation entry point does not exist yet.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a pure helper that scans local bindings in order, looks up each binding node in a supplied declaration signature slice, and emits `KnownSymbolType` records for matches.

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
- [x] No compiler behavior beyond known local symbol type derivation is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-013-known-local-symbol-types.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-013-known-local-symbol-types.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-013-known-local-symbol-types.md`
  - `docs/tasks/reviews/M0018-013-review.md`
  - `docs/tasks/soundness/M0018-013-soundness.md`

## Forbidden Changes

- Do not change name-resolution semantics.
- Do not infer missing declaration signatures.
- Do not type name expressions in this task.
- Do not implement declaration initializer checks using names in this task.
- Do not add member lookup, calls, ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Full integration with parser and name-resolution output remains a later M0018 task.
- Name-based declaration initializer checks remain a later M0018 task.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0018 known local symbol type derivation task.
- 2026-07-10 agent=Test-Engineer phase=test-first result=pass notes=`cargo test --workspace --all-targets` failed before implementation with unresolved import `newlang::type_check::known_local_symbol_types`.
- 2026-07-10 agent=Implementer phase=implementation result=pass notes=Added pure known local symbol type derivation from local bindings and declaration signatures.
- 2026-07-10 agent=Implementer phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 146 tests.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-013-known-local-symbol-types.md` created a passing soundness report after ordinary tests.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=Review approved scope, spec compliance, and maintainability.
- 2026-07-10 agent=Implementer phase=ci result=pass notes=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets`; `sh docs/tests/m0018-type-checking-core-accepted.sh`; `sh docs/tests/m0002-workspace-ci.sh`.
