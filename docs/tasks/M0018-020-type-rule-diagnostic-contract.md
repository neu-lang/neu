# Task: M0018-020 Type Rule Diagnostic Contract

## Task Metadata

- Task ID: `M0018-020`
- Milestone: `M0018`
- Milestone File: `docs/milestones/M0018-type-checking-core.md`
- Status: `complete`
- Owner main task: `main-task diagnostics check`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0018-020-type-rule-diagnostic-contract`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
- Milestone: `docs/milestones/M0018-type-checking-core.md`

## Goal

Represent the ADR-0027 `unresolved_type_rule` and `unsupported_type_rule` diagnostics with stable rule identifiers.

## Motivation

ADR-0027 requires type checking diagnostics for unresolved and unsupported type rules. Existing M0018 diagnostics cover blockers and type mismatches, but do not yet expose the accepted unresolved and unsupported diagnostic contract needed by later fixtures.

## Scope

- Add diagnostic kind representation for `unresolved_type_rule`.
- Add diagnostic kind representation for `unsupported_type_rule`.
- Add stable rule identifier representation for accepted ADR-0027 unresolved and unsupported rule examples.
- Preserve existing ambiguity and type mismatch diagnostics.
- Add tests proving diagnostic kind, node, rule identifier, and type payload behavior.

## Out Of Scope

- Emitting unresolved or unsupported diagnostics from a traversal pass.
- Type checking calls, members, binary expressions, unary expressions, value-producing `if` expressions, or block values.
- Implementing inference, overload resolution, generic solving, ownership, borrow, HIR, MIR, or backend behavior.
- Changing parser metadata.
- Changing diagnostic rendering or CLI output.

## Required Inputs

- `docs/adr/ADR-0027-type-checking-core.md`
- Existing `TypeCheckDiagnostic` model.
- Existing M0018 type-checking tests.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `unresolved_type_rule` diagnostics preserve the stable rule identifier and node.
  - `unsupported_type_rule` diagnostics preserve the stable rule identifier and node.
  - Existing `type_mismatch` diagnostics continue to preserve expected and actual types.
- Negative tests:
  - Unresolved and unsupported diagnostics do not synthesize expected or actual types.
  - Ambiguous diagnostics remain distinct from unresolved and unsupported diagnostics.
- Adversarial tests:
  - Adding diagnostic representation does not type any unsupported expression or resolve any missing type rule.

## Test-First Gate

- Test files to edit before implementation:
  - `crates/newlang/tests/type_check.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `unresolved_type_rule`, `unsupported_type_rule`, and stable type-rule identifiers do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend the type-check diagnostic data model with accepted ADR-0027 unresolved and unsupported rule identifiers and constructors, keeping existing fields and behavior compatible with earlier M0018 tasks.

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
- [x] No compiler behavior beyond diagnostic representation is introduced.

## Execution Commands

- Generate tests: edit `crates/newlang/tests/type_check.rs`
- Verify tests fail: `cargo test --workspace --all-targets`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0018-020-type-rule-diagnostic-contract.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0018-020-type-rule-diagnostic-contract.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0018-type-checking-core-accepted.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0018-020-type-rule-diagnostic-contract.md`
  - `docs/tasks/reviews/M0018-020-review.md`
  - `docs/tasks/soundness/M0018-020-soundness.md`

## Forbidden Changes

- Do not emit diagnostics from a new traversal pass.
- Do not type unsupported expression forms.
- Do not infer types or resolve missing names.
- Do not add ownership, borrow, HIR, MIR, or backend behavior.
- Do not weaken or delete existing M0018 tests.

## Ambiguities And Dependencies

- Traversal that emits these diagnostics remains a later task.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0018 type-rule diagnostic contract task.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added unresolved and unsupported type-rule diagnostic contract tests before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test --workspace --all-targets` failed before implementation because `TypeRuleDiagnostic`, `unresolved_type_rule`, `unsupported_type_rule`, `UnresolvedTypeRule`, and `UnsupportedTypeRule` did not exist.
- 2026-07-10 main_task=Diagnostics-Engineer phase=implementation result=pass notes=Added stable type-rule diagnostic identifiers plus unresolved and unsupported diagnostic constructors.
- 2026-07-10 main_task=Diagnostics-Engineer phase=ordinary-tests result=pass notes=`cargo test --workspace --all-targets` passed with 166 tests.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=`docs/scripts/adversarial-check.sh docs/tasks/M0018-020-type-rule-diagnostic-contract.md` passed after ordinary tests.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Review approved against `docs/SPEC.md`, ADR-0027, and `docs/milestones/M0018-type-checking-core.md`.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `sh docs/tests/m0018-type-checking-core-accepted.sh`, and `sh docs/tests/m0002-workspace-ci.sh` passed.
