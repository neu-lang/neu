# Task: M0016-009 Build declaration index from parser metadata

## Task Metadata

- Task ID: `M0016-009`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-009-build-declaration-index`

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

## Goal

Populate `DeclarationIndex` from parser declaration-name metadata and module package metadata for accepted top-level declarations.

## Motivation

M0016 now has parser-owned declaration-name metadata and a declaration index keyed by module, package namespace, symbol, and declaration kind. The next safe step is connecting those two storage layers without implementing lookup.

## Scope

- Add a builder that consumes `ParsedDeclarationName` records, `ModuleMetadata`, and `SymbolInterner`.
- Intern declaration names and create ADR-0026 declaration keys.
- Use the declaring source file's package namespace from module metadata.
- Preserve insertion order.
- Preserve duplicate insert results for later `duplicate_name` diagnostics.

## Out Of Scope

- Name lookup.
- Local bindings.
- Lexical scope stack.
- Resolution diagnostics emission.
- Import handling.
- Cross-module lookup.
- Parser changes.
- Accepted ADR changes.

## Required Inputs

- Parser declaration metadata from `crates/newlang/src/parser.rs`.
- Module metadata from `crates/newlang/src/module.rs`.
- Declaration index from `crates/newlang/src/name_resolution.rs`.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Declaration index builder populates function and type declarations from parser metadata.
  - Builder uses the source file package namespace from module metadata.
  - Builder interns declaration names in stable order.
- Negative tests:
  - Duplicate declaration keys produce duplicate insert records without replacing existing declarations.
  - Builder does not implement lookup or local binding behavior.
- Adversarial tests:
  - Same declaration name in different packages is not reported as duplicate.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Declaration index builder API does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a builder result containing the populated declaration index and all insertion outcomes. Keep it deterministic and storage-only.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-009-build-declaration-index.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-009-build-declaration-index.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-009-build-declaration-index.md`

## Forbidden Changes

- Do not implement lookup.
- Do not add local binding storage.
- Do not add scope stack behavior.
- Do not add parser changes.
- Do not emit diagnostics.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Missing module/package metadata diagnostics are deferred; this task assumes module metadata contains the relevant source-file package entries.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 declaration index builder task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated name-resolution and docs validators before adding declaration index builder API.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because build_declaration_index was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added build_declaration_index to populate DeclarationIndex from parser metadata, module package metadata, and symbol interning without lookup behavior.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-009-soundness.md after ordinary tests were recorded.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-009-review.md approved declaration-index-builder scope pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add declaration index builder after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
