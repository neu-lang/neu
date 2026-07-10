# Task: M0016-006 Add name resolution data model

## Task Metadata

- Task ID: `M0016-006`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-006-name-resolution-data-model`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Add the minimal data model needed to record resolved name references and M0016 resolution diagnostics without implementing lookup.

## Motivation

Accepted ADR-0026 requires later implementation to emit stable resolved symbols and structured diagnostics. A small data model gives later lookup tasks a stable target while keeping semantic lookup behavior out of this slice.

## Scope

- Add `crates/newlang/src/name_resolution.rs`.
- Export the module from `crates/newlang/src/lib.rs`.
- Add `ResolutionTable`, `ResolvedName`, `ResolutionDiagnostic`, and `ResolutionDiagnosticKind`.
- Add tests for stable resolved-name insertion, lookup by AST reference, unknown lookup, diagnostic records, and ADR-0026 diagnostic variants.
- Update M0016 docs validators to allow the data model while still forbidding lookup implementation.

## Out Of Scope

- Implementing lexical scope construction.
- Implementing lookup.
- Implementing duplicate detection.
- Implementing unresolved-name analysis.
- Implementing parser integration.
- Adding resolution fixtures.
- Activating imports, cross-module lookup, member lookup, overloads, extensions, or type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Existing symbol infrastructure: `crates/newlang/src/symbol.rs`
- Existing AST IDs: `crates/newlang/src/ast.rs`
- Existing source spans: `crates/newlang/src/source.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Resolved name records preserve reference node, symbol ID, and insertion order.
  - Lookup by reference AST node returns the resolved symbol.
  - Resolution diagnostics preserve kind and primary span.
- Negative tests:
  - Unknown reference lookups return `None`.
  - The implementation does not expose lookup, scope stack, import resolver, or visibility enforcement behavior.
- Diagnostic tests:
  - All accepted ADR-0026 diagnostic variants are represented.
- Adversarial tests:
  - Data model cannot silently choose among duplicate resolution entries.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `newlang::name_resolution` module does not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Create only storage and diagnostic records. Reject duplicate resolved-name inserts by preserving the existing record and returning both existing and attempted records.

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

- Generate tests: `create crates/newlang/tests/name_resolution.rs && create docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p newlang --test name_resolution`
- Ordinary tests: `cargo test -p newlang --test name_resolution && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-006-name-resolution-data-model.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-006-name-resolution-data-model.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`
  - `docs/tests/m0016-name-resolution-blocked.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
  - `crates/newlang/src/lib.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-006-name-resolution-data-model.md`

## Forbidden Changes

- Do not implement lookup.
- Do not add scope stack behavior.
- Do not add import resolver behavior.
- Do not add visibility enforcement.
- Do not modify accepted ADR-0026.
- Do not add parser integration.

## Ambiguities And Dependencies

- Lookup semantics are accepted by ADR-0026 but intentionally left for a later implementation task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 name resolution data model task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Created Rust and docs validators before adding name_resolution module.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because newlang::name_resolution did not exist.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added name_resolution data model records, duplicate-preserving insert behavior, and ADR-0026 diagnostic variants without lookup behavior.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution and M0016 data-model/accepted/concrete/review/proposal/authority validators passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-006-soundness.md after ordinary tests were recorded.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-006-review.md approved data-model-only implementation scope pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/concrete/review/proposal/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add M0016 data model after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/ast.rs`
  - `crates/newlang/src/symbol.rs`
