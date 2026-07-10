# Task: M0014-006 Add module identity model

## Task Metadata

- Task ID: `M0014-006`
- Milestone: `M0014`
- Milestone File: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0014-006-module-identity-model`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add the ADR-0025 module identity and source-file assignment data model without package extraction, visibility extraction, name resolution, or module dependency behavior.

## Motivation

M0014 now has accepted authority for explicit module names and one-module source-file assignment. The frontend needs a small data model before later tasks can attach package namespaces and visibility metadata.

## Scope

- Add a `module` frontend module.
- Add validated explicit module names using ADR-0021 identifier spelling and dot separators.
- Preserve deterministic module identity as the exact validated module-name string.
- Add a module metadata record containing module name and ordered source file identities.
- Add diagnostics for missing and invalid module identity.
- Add focused Rust tests and a docs validator.
- Update M0014 validators that previously required module implementation files to remain absent.

## Out Of Scope

- Package namespace extraction.
- Visibility metadata extraction.
- Module dependencies.
- Name resolution.
- Symbol tables.
- Manifest syntax.
- Target-pack artifact compatibility.
- Cross-module import lookup.
- Any parser behavior changes.

## Required Inputs

- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0025-module-package-visibility-model.md`
- `docs/SPEC.md`
- `crates/compiler/src/source.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Dot-separated module names using ADR-0021 identifiers are accepted.
  - The deterministic module ID is the exact validated module-name string.
  - Module metadata preserves ordered source file identities.
- Negative tests:
  - Empty module names, leading dots, trailing dots, repeated dots, non-identifier segments, and Unicode identifier segments are rejected.
- Diagnostic tests:
  - Missing module identity reports `MissingModuleIdentity`.
  - Malformed module identity reports `InvalidModuleIdentity`.
- Adversarial tests:
  - Module identity must not depend on host paths, source roots, output paths, package names, manifests, target triples, or dependencies.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0014-module-identity-model.sh`
  - `crates/compiler/tests/module.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `crates/compiler/src/module.rs` and `compiler::module` do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `crates/compiler/src/module.rs` with a small validated `ModuleName`, module metadata record, and module diagnostics. Export it from `crates/compiler/src/lib.rs`. Do not read source file paths or parse package declarations in this task.

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
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0014-module-identity-model.sh and crates/compiler/tests/module.rs`
- Verify tests fail: `docs/tests/m0014-module-identity-model.sh`
- Ordinary tests: `cargo test --workspace --all-targets module -- --nocapture && docs/tests/m0014-module-identity-model.sh && docs/tests/m0014-module-package-visibility-model-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0014-006-module-identity-model.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0014-006-module-identity-model.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0014-module-identity-model.sh && docs/tests/m0014-module-package-visibility-model-accepted.sh && docs/tests/m0014-module-package-visibility-model-concrete-draft.sh && docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0014-module-identity-model.sh`
  - `crates/compiler/tests/module.rs`
- Implementation files:
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/module.rs`
- Documentation or checklist files:
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
  - `docs/tasks/M0014-006-module-identity-model.md`
  - M0014 validators that need to allow the module identity file.

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement package namespaces or visibility metadata in this task.
- Do not add name resolution, symbol tables, module dependencies, manifests, target triples, or target-pack behavior.
- Do not derive module identity from host paths, source roots, output paths, package names, current directories, or source file ordering.

## Ambiguities And Dependencies

- Package namespace and visibility metadata are accepted in ADR-0025 but intentionally left for later M0014 tasks.
- Module dependency metadata remains deferred by ADR-0025.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0014 module identity model task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0014-module-identity-model.sh and Rust module tests before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0014-module-identity-model.sh failed before implementation because crates/compiler/src/module.rs was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added explicit ModuleName validation and ModuleMetadata source-file assignment model without package, visibility, or name-resolution behavior.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets module -- --nocapture plus M0014 module identity and accepted ADR validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0014-006-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0014-006-review.md approves module identity scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0014-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add failing tests for explicit module identity model.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
