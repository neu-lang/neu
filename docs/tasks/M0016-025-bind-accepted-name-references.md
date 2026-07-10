# Task: M0016-025 Bind accepted name references

## Task Metadata

- Task ID: `M0016-025`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-025-bind-accepted-name-references`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/test-engineer.md`
  - `.codex/agents/implementer.md`
  - `.codex/agents/reviewer.md`
  - `.codex/agents/adversarial-engineer.md`

## Goal

Provide a single accepted-subset binding entry point that combines the already implemented M0016 simple expression, unqualified type, and package-qualified type reference binders.

## Motivation

M0016 now has separate binders for simple identifier expression references, unqualified type references, and package-qualified type references. A later frontend stage needs one result containing all accepted resolved names and diagnostics. This task composes the accepted pieces without implementing package-qualified expression lookup, import lookup, cross-module lookup, member lookup, or full module orchestration.

## Scope

- Add an accepted reference binding result type.
- Add a function that accepts parser-recorded simple expression references, parser-recorded type-name references, local scopes, local bindings, and top-level declarations.
- Bind simple expression references with the existing unqualified function-reference binder.
- Partition type-name references into unqualified and package-qualified references.
- Bind unqualified type names with the existing unqualified type-reference binder.
- Bind package-qualified type names with the existing package-qualified type-reference binder.
- Merge successful bindings into one `ResolutionTable`.
- Merge diagnostics in deterministic binding-stage order.

## Out Of Scope

- Package-qualified expression lookup.
- Import lookup.
- Cross-module lookup.
- Member, method, constructor, overload, extension, or type-directed lookup.
- Building declaration indices.
- Building local scope trees.
- Building local binding indices.
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
  - The combined binder records simple expression, unqualified type, and package-qualified type bindings in one table.
  - Package-qualified type references are not also processed as unqualified type references.
- Negative tests:
  - Unresolved expression and type references both produce diagnostics.
  - Package-qualified expression/member syntax remains outside the combined binder.
- Adversarial tests:
  - The task does not activate imports or cross-module lookup.
  - The task does not build indices or add full module resolver orchestration.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Accepted name-reference binding APIs do not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `AcceptedNameReferenceBind` and `bind_accepted_name_references`, reusing existing binding APIs and merging their `ResolutionTable` outputs and diagnostics.

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

## Execution Commands

- Generate tests: `update crates/newlang/tests/name_resolution.rs && update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p newlang --test name_resolution`
- Ordinary tests: `cargo test -p newlang --test name_resolution && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-025-bind-accepted-name-references.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-025-bind-accepted-name-references.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-025-bind-accepted-name-references.md`

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
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 accepted name-reference binding task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding accepted reference binder API.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because bind_accepted_name_references was missing.
2026-07-10 agent=Implementer phase=implementation result=pass notes=Added AcceptedNameReferenceBind and bind_accepted_name_references composing existing accepted M0016 binders.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests; concrete adversarial review found no import, cross-module, member, expression-qualified, or broader resolver behavior.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/scripts/review-task.sh created a review after adversarial checks; concrete review approved scope pending final CI.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Task Decomposer`
- Reason: `Select the next M0016 task after accepted name-reference binding.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
