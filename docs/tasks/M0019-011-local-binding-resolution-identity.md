# Task: M0019-011 Preserve Local Binding Resolution Identity

## Task Metadata

- Task ID: `M0019-011`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-011-local-binding-resolution-identity`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Preserve exact local binding identity for each successfully resolved local name use so flow typing can distinguish shadowed bindings.

## Motivation

ADR-0028 requires refinements to follow local binding identity rather than textual names. The existing local reference binder records only a symbol in `ResolutionTable`, which cannot distinguish an outer binding from a nested binding with the same name. Per-use refinement typing must not proceed until exact local binding identity is available.

## Scope

- Add a passive resolved-local record containing a name-expression reference node and its exact `LocalBinding`.
- Make `bind_local_name_references` return resolved-local records alongside its existing symbol resolution table.
- Preserve source/reference order in the resolved-local output.
- Add focused Rust tests and a docs validator.

## Out Of Scope

- Recording `RefinedExpressionType` entries.
- Walking refinement branch regions.
- Nullable-use diagnostics.
- Mutation invalidation.
- Changing local lookup, visibility, or shadowing semantics.
- Changing the general `ResolutionTable` representation.
- Updating examples; this is internal name-resolution metadata only.

## Required Inputs

- M0016 local binding and lexical-scope metadata.
- Existing `bind_local_name_references` behavior.
- ADR-0028 shadowing and local binding identity rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - A resolved local use records the exact `LocalBinding` returned by local lookup.
  - Resolved-local records preserve reference order.
- Negative tests:
  - An unresolved local use does not produce a resolved-local identity record.
- Diagnostic tests:
  - Existing unresolved-name diagnostics remain unchanged.
- Adversarial tests:
  - Same-name uses before and after a nested shadowing declaration retain distinct outer and inner binding identities.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0019-local-binding-resolution-identity.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Resolved local binding identity output does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend the existing local reference binding result with passive records captured directly from successful local lookup results. Keep the existing general symbol resolution table and all lookup semantics unchanged.

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
- [x] Examples decision is recorded.

## Execution Commands

- Generate tests: `edit crates/compiler/tests/name_resolution.rs and create docs/tests/m0019-local-binding-resolution-identity.sh`
- Verify tests fail: `cargo test -p compiler --test name_resolution m0019_local_binding_resolution_identity`
- Ordinary tests: `cargo test -p compiler --test name_resolution m0019_local_binding_resolution_identity && sh docs/tests/m0019-local-binding-resolution-identity.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-011-local-binding-resolution-identity.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-011-local-binding-resolution-identity.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-local-binding-resolution-identity.sh && sh docs/tests/m0019-branch-refinement-records.sh && sh docs/tests/m0019-null-test-eligibility.sh && sh docs/tests/m0019-null-test-recognition.sh && sh docs/tests/m0019-parser-flow-metadata.sh && sh docs/tests/m0019-flow-output-data-model.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0019-local-binding-resolution-identity.sh`
- Implementation files:
  - `crates/compiler/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0019-011-local-binding-resolution-identity.md`
  - `docs/tasks/reviews/M0019-011-review.md`
  - `docs/tasks/soundness/M0019-011-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not change local lookup or shadowing behavior.
- Do not add per-use refined expression types or nullable diagnostics.

## Ambiguities And Dependencies

- A later M0019 task will consume these exact binding identities while recording per-use refined expression types inside active branch regions.

## Execution Log

- 2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created prerequisite task for shadowing-safe local binding identity output.
- 2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Added exact-binding, nested-shadowing, unresolved-use tests and a docs validator before implementation.
- 2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=`cargo test -p compiler --test name_resolution m0019_local_binding_resolution_identity` failed because resolved-local identity output was absent; docs validator failed for the missing record type.
- 2026-07-10 main_task=main-task test work phase=fixture-correction result=pass notes=Replaced unsupported boolean-literal condition text with an accepted name-expression condition; identity assertions and tested behavior were unchanged.
- 2026-07-10 main_task=main-task implementation phase=implement result=pass notes=Added passive resolved-local records captured directly from successful local lookup results without changing lookup or general resolution behavior.
- 2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=Focused M0019 identity tests, the 57-test name-resolution suite, docs validator, and `git diff --check` passed; `cargo fmt --all` normalized the new fixture.
- 2026-07-10 main_task=Adversarial-Engineer phase=adversarial-check result=pass notes=Harness ran after ordinary tests; shadowing, scope exit, unresolved-name, and source-order attacks passed with a concrete soundness report.
- 2026-07-10 main_task=main-task review phase=review result=pass notes=Compared implementation with SPEC lexical scope rules, ADR-0028, and M0019; approved the fixture terminator correction and implementation pending final CI.
- 2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=`cargo fmt --all --check`, workspace clippy with warnings denied, all 191 workspace tests, all listed M0019 validators, and the M0002 workspace CI gate passed.
- 2026-07-10 main_task=Task-Decomposer phase=milestone-checklist result=pass notes=No M0019 completion item changed; binding identity is a prerequisite and does not yet complete nullable checks, smart casts, or mutation invalidation.
- 2026-07-10 main_task=main-task implementation phase=examples-decision result=pass notes=No examples update; this task adds internal name-resolution identity metadata without changing source-language syntax or semantics.

## Handoff

- Next main task: `main-task task planning`
- Reason: `Create the next M0019 task for per-use refined expression type records.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `crates/compiler/src/name_resolution.rs`
