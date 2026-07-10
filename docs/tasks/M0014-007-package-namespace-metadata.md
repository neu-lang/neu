# Task: M0014-007 Add package namespace metadata

## Task Metadata

- Task ID: `M0014-007`
- Milestone: `M0014`
- Milestone File: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0014-007-package-namespace-metadata`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add ADR-0025 package namespace representation and source-file package metadata to the existing module model.

## Motivation

M0014 requires package or namespace representation before name resolution. ADR-0025 says packages are namespaces inside modules, package declarations use ADR-0022 qualified names, and files without packages belong to the root package represented by the empty package path.

## Scope

- Add a `PackageNamespace` model to `crates/compiler/src/module.rs`.
- Represent the root package as the empty package path.
- Validate package namespace segments using ADR-0021 identifier spelling and dot separators.
- Add per-source-file package metadata to `ModuleMetadata`.
- Preserve the existing module identity and ordered source-file behavior.
- Add tests and a docs validator.

## Out Of Scope

- Parsing package declarations from source text.
- Visibility metadata extraction.
- Name resolution.
- Symbol tables.
- Module dependencies.
- Import resolution.
- Manifest syntax.
- Target-pack artifact compatibility.
- Parser behavior changes.

## Required Inputs

- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0022-declaration-syntax.md`
- `docs/adr/ADR-0025-module-package-visibility-model.md`
- `crates/compiler/src/module.rs`
- `crates/compiler/tests/module.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Empty package namespace is root.
  - Dot-separated package namespace segments are accepted.
  - Module metadata preserves package namespace per source file.
  - Multiple files may share one package namespace.
- Negative tests:
  - Leading dots, trailing dots, repeated dots, non-identifier segments, and Unicode package segments are rejected.
- Diagnostic tests:
  - Invalid package namespace reports `InvalidPackageNamespace`.
- Adversarial tests:
  - Package namespaces must not become module identity, module dependencies, import resolution, name resolution, or package manager behavior.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0014-package-namespace-metadata.sh`
  - package namespace test additions in `crates/compiler/tests/module.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `PackageNamespace` and source-file package metadata do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend `crates/compiler/src/module.rs` with `PackageNamespace` and per-source-file package records. Keep package data explicit; do not parse it from source text or connect it to name resolution.

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

- Generate tests: `create docs/tests/m0014-package-namespace-metadata.sh and update crates/compiler/tests/module.rs`
- Verify tests fail: `docs/tests/m0014-package-namespace-metadata.sh`
- Ordinary tests: `cargo test --workspace --all-targets module -- --nocapture && docs/tests/m0014-package-namespace-metadata.sh && docs/tests/m0014-module-identity-model.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0014-007-package-namespace-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0014-007-package-namespace-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0014-package-namespace-metadata.sh && docs/tests/m0014-module-identity-model.sh && docs/tests/m0014-module-package-visibility-model-accepted.sh && docs/tests/m0014-module-package-visibility-model-concrete-draft.sh && docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0014-package-namespace-metadata.sh`
  - `crates/compiler/tests/module.rs`
- Implementation files:
  - `crates/compiler/src/module.rs`
- Documentation or checklist files:
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
  - `docs/tasks/M0014-007-package-namespace-metadata.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement parser package extraction in this task.
- Do not implement visibility metadata in this task.
- Do not add name resolution, symbol tables, module dependencies, manifests, target triples, or target-pack behavior.
- Do not make package namespace part of module identity.

## Ambiguities And Dependencies

- Package declaration parsing already exists syntactically, but semantic extraction from parser output is intentionally deferred.
- Visibility metadata remains the remaining M0014 implementation slice after this task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0014 package namespace metadata task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0014-package-namespace-metadata.sh and Rust package namespace tests before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0014-package-namespace-metadata.sh failed before implementation because PackageNamespace was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added PackageNamespace and per-source-file package metadata without parser extraction, visibility, dependencies, or name resolution.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets module -- --nocapture plus M0014 package namespace and module identity validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0014-007-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0014-007-review.md approves package namespace metadata scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0014-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add failing tests for package namespace metadata.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
