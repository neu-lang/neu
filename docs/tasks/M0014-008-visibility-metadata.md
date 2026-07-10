# Task: M0014-008 Add visibility metadata representation

## Task Metadata

- Task ID: `M0014-008`
- Milestone: `M0014`
- Milestone File: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0014-008-visibility-metadata`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add ADR-0025 visibility metadata representation to module metadata without parser extraction or name resolution.

## Motivation

M0014 requires visibility metadata for declarations with parsed visibility scope. ADR-0025 defines the categories `public`, `internal`, and `private`, default visibility `internal`, and explicit/defaulted metadata. The compiler needs this representation before a later task can extract it from parser output.

## Scope

- Add visibility category representation for `public`, `internal`, and `private`.
- Add visibility origin representation for explicit vs defaulted metadata.
- Add declaration visibility metadata keyed by AST declaration node ID.
- Store visibility metadata in `ModuleMetadata`.
- Add tests and a docs validator.
- Mark M0014 visibility metadata represented.

## Out Of Scope

- Parsing visibility metadata from source text.
- Modifying parser output.
- Name resolution or access checking.
- Cross-module dependency lookup.
- Package and import visibility metadata.
- Protected, friend, sealed, extension, or protocol-conformance visibility.
- Diagnostics beyond the accepted metadata enum names already represented.

## Required Inputs

- `docs/adr/ADR-0022-declaration-syntax.md`
- `docs/adr/ADR-0025-module-package-visibility-model.md`
- `crates/newlang/src/ast.rs`
- `crates/newlang/src/module.rs`
- `crates/newlang/tests/module.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Explicit `public`, `internal`, and `private` metadata can be represented.
  - Omitted visibility is represented as defaulted `internal`.
  - Module metadata preserves declaration visibility records.
- Negative tests:
  - Package and import declarations are not modeled as visibility metadata in tests.
- Diagnostic tests:
  - `UnsupportedVisibilityCategory` and `DuplicateVisibilityMetadata` diagnostic kinds exist.
- Adversarial tests:
  - Visibility metadata must not implement access checking, dependency lookup, name resolution, or additional visibility categories.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0014-visibility-metadata.sh`
  - visibility metadata test additions in `crates/newlang/tests/module.rs`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `VisibilityCategory`, `VisibilityOrigin`, and declaration visibility metadata do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Extend `crates/newlang/src/module.rs` with visibility metadata types and a constructor that accepts package and visibility metadata together. Keep metadata explicit and independent of parser extraction.

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

- Generate tests: `create docs/tests/m0014-visibility-metadata.sh and update crates/newlang/tests/module.rs`
- Verify tests fail: `docs/tests/m0014-visibility-metadata.sh`
- Ordinary tests: `cargo test --workspace --all-targets module -- --nocapture && docs/tests/m0014-visibility-metadata.sh && docs/tests/m0014-package-namespace-metadata.sh && docs/tests/m0014-module-identity-model.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0014-008-visibility-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0014-008-visibility-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0014-visibility-metadata.sh && docs/tests/m0014-package-namespace-metadata.sh && docs/tests/m0014-module-identity-model.sh && docs/tests/m0014-module-package-visibility-model-accepted.sh && docs/tests/m0014-module-package-visibility-model-concrete-draft.sh && docs/tests/m0014-module-package-visibility-model-review.sh && docs/tests/m0014-module-package-visibility-model-proposal.sh && docs/tests/m0014-module-package-visibility-model-blocked.sh && docs/tests/m0013-expression-statement-pattern-parser-implementation.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0014-visibility-metadata.sh`
  - `crates/newlang/tests/module.rs`
- Implementation files:
  - `crates/newlang/src/module.rs`
- Documentation or checklist files:
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
  - `docs/tasks/M0014-008-visibility-metadata.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify accepted ADRs.
- Do not weaken or delete failing tests without main-task review approval.
- Do not modify parser behavior in this task.
- Do not implement name resolution, access checking, dependency lookup, manifests, target triples, or target-pack behavior.
- Do not add visibility categories beyond `public`, `internal`, and `private`.
- Do not attach visibility metadata to package or import declarations.

## Ambiguities And Dependencies

- Parser extraction of visibility metadata remains a later task.
- Access checking belongs to M0016 name resolution and later semantic analysis.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0014 visibility metadata representation task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0014-visibility-metadata.sh and Rust visibility metadata tests before implementation.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=docs/tests/m0014-visibility-metadata.sh failed before implementation because VisibilityCategory was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added visibility category, origin, declaration metadata, module storage, and diagnostic variants without parser extraction or access checking.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test --workspace --all-targets module -- --nocapture plus M0014 visibility, package namespace, and module identity validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0014-008-soundness.md after ordinary-test evidence.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0014-008-review.md approves visibility metadata scope.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && M0014-M0002 validation scripts passed.
```

## Handoff

- Next main task: `main-task test work`
- Reason: `Add failing tests for visibility metadata representation.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
