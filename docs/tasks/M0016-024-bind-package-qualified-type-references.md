# Task: M0016-024 Bind package-qualified type references

## Task Metadata

- Task ID: `M0016-024`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-024-bind-package-qualified-type-references`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Bind parser-recorded package-qualified type-name references to same-module top-level type declarations in the explicitly named package namespace.

## Motivation

ADR-0026 includes package-qualified names in accepted type positions and states that package-qualified lookup uses the explicitly named package namespace in the current module only. M0016-022 records the full qualified type-name text, and M0016-023 binds unqualified type names. This task adds the package-qualified type-name slice without activating imports or cross-module lookup.

## Scope

- Add a package-qualified type-reference binding result type.
- Add a function that accepts module metadata, parser-recorded type-name references, and a declaration index.
- Bind only references containing at least one dot.
- Split qualified type names at the final dot into package namespace and declaration name.
- Look up top-level type declarations in the current module and explicitly named package namespace.
- Insert successful bindings into `ResolutionTable`.
- Produce `UnresolvedName` when the current-module package-qualified type lookup finds no candidate.

## Out Of Scope

- Unqualified type-name lookup.
- Package-qualified expression lookup.
- Import lookup.
- Cross-module lookup.
- Member, constructor, overload, extension, or type-directed lookup.
- Separate value/type namespace design.
- Full-file or full-module resolver orchestration.
- Changing duplicate-name behavior.
- Changing visibility enforcement.

## Required Inputs

- `docs/SPEC.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `crates/newlang/src/name_resolution.rs`
- `crates/newlang/tests/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Package-qualified type references bind to same-module declarations in the explicitly named package.
  - Nested package paths split at the final dot.
- Negative tests:
  - Unqualified type names are ignored by this package-qualified binder.
  - Function declarations are not used as package-qualified type fallback.
  - Missing package-qualified type names produce `UnresolvedName`.
- Adversarial tests:
  - The task does not activate imports or cross-module lookup.
  - The task does not treat member expressions as type references.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Package-qualified type reference binding APIs do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `PackageQualifiedTypeReferenceBind` and `bind_package_qualified_type_references`, reusing parser type-name metadata, module package namespaces, top-level declaration lookup, and `ResolutionTable`.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-024-bind-package-qualified-type-references.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-024-bind-package-qualified-type-references.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-024-bind-package-qualified-type-references.md`

## Forbidden Changes

- Do not implement package-qualified expression lookup.
- Do not implement imports.
- Do not implement cross-module lookup.
- Do not implement member or overload lookup.
- Do not add full module resolver orchestration.
- Do not modify accepted ADR-0026.
- Do not weaken or delete existing M0016 tests.

## Ambiguities And Dependencies

- Package-qualified expression lookup remains blocked on parser metadata that distinguishes package-qualified expressions from member access.
- Cross-module qualified paths remain unsupported by ADR-0026.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 package-qualified type reference binding task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding package-qualified type binder API.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because bind_package_qualified_type_references was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added PackageQualifiedTypeReferenceBind and bind_package_qualified_type_references for dotted type-name metadata.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests; concrete adversarial review found no import, cross-module, member, expression, or broader resolver behavior.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/scripts/review-task.sh created a review after adversarial checks; concrete review approved scope pending final CI.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task task planning`
- Reason: `Select the next M0016 task after package-qualified type reference binding.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
