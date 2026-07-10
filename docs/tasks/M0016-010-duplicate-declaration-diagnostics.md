# Task: M0016-010 Add duplicate declaration diagnostics

## Task Metadata

- Task ID: `M0016-010`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-010-duplicate-declaration-diagnostics`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Emit `duplicate_name` resolution diagnostics when declaration-index construction encounters duplicate accepted top-level declaration keys.

## Motivation

ADR-0026 requires same-module same-package duplicate top-level declarations with the same declaration kind to produce `duplicate_name`. The builder already preserves duplicate insertion evidence; this task turns that evidence into structured diagnostics without adding lookup.

## Scope

- Extend `DeclarationIndexBuild` with resolution diagnostics.
- Emit `ResolutionDiagnosticKind::DuplicateName` for duplicate declaration keys.
- Use the later attempted declaration name span as the diagnostic primary span.
- Preserve declaration index and insert behavior.
- Add tests for duplicate diagnostic kind/span and non-duplicate package separation.

## Out Of Scope

- Unresolved-name diagnostics.
- Ambiguous-name diagnostics.
- Lookup.
- Local binding diagnostics.
- Related-location diagnostics.
- Parser changes.
- Accepted ADR changes.

## Required Inputs

- `build_declaration_index` from `crates/newlang/src/name_resolution.rs`.
- Parser declaration metadata from `crates/newlang/src/parser.rs`.
- ADR-0026 duplicate-name diagnostic policy.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Duplicate declaration builder output includes `DuplicateName` diagnostic.
  - Duplicate diagnostic primary span is the later declaration name span.
  - Non-duplicate same-name declarations in different packages do not produce diagnostics.
- Negative tests:
  - Builder still does not implement lookup.
  - Builder does not emit unresolved or ambiguous diagnostics.
- Adversarial tests:
  - Duplicate diagnostics must not replace the original declaration in the index.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `DeclarationIndexBuild` does not expose diagnostics.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Collect duplicate diagnostics while building the declaration index from parser metadata. Keep the existing insertion results unchanged.

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

## Execution Commands

- Generate tests: `update crates/newlang/tests/name_resolution.rs && update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p newlang --test name_resolution`
- Ordinary tests: `cargo test -p newlang --test name_resolution && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-010-duplicate-declaration-diagnostics.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-010-duplicate-declaration-diagnostics.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-010-duplicate-declaration-diagnostics.md`

## Forbidden Changes

- Do not implement lookup.
- Do not emit unresolved-name diagnostics.
- Do not emit ambiguous-name diagnostics.
- Do not add related-location diagnostics.
- Do not change parser behavior.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Related declaration locations for diagnostics remain a future diagnostics enhancement.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 duplicate declaration diagnostics task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution and docs validators before adding builder diagnostics.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because DeclarationIndexBuild did not expose diagnostics.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added duplicate_name diagnostics for duplicate declaration keys using the attempted declaration name span.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-010-soundness.md after ordinary tests were recorded.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-010-review.md approved duplicate-diagnostics scope pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add duplicate declaration diagnostics after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
