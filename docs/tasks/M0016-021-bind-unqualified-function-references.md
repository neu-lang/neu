# Task: M0016-021 Bind unqualified function references

## Task Metadata

- Task ID: `M0016-021`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-021-bind-unqualified-function-references`

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

Bind parser-recorded simple identifier expression references by checking visible locals first, then same-module current-package top-level function declarations.

## Motivation

ADR-0026 requires unqualified simple identifier expression lookup to search local lexical scopes outward and then the current source file's package namespace in the current module. M0016-020 implemented the local-only portion. This task adds the current-package top-level function fallback while keeping type-name, package-qualified, import, member, and cross-module lookup deferred.

## Scope

- Add a function-reference binding result type.
- Add a function that accepts module metadata, parser-recorded name references, local scopes, local bindings, and a declaration index.
- Check local bindings first using existing local lookup.
- If no local binding is visible, check top-level function declarations in the reference source file's package namespace.
- Insert successful local or top-level function bindings into `ResolutionTable`.
- Produce `UnresolvedName` when neither lookup tier finds a candidate.
- Preserve parser reference node ids and name spans.

## Out Of Scope

- Top-level type-name fallback.
- Package-qualified expression lookup.
- Type annotation name lookup.
- Import lookup.
- Cross-module lookup.
- Member, method, constructor, overload, extension, or type-directed lookup.
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
  - Same-package top-level function references bind after local lookup misses.
  - Local bindings still bind before top-level function fallback.
- Negative tests:
  - Top-level functions in a different package do not satisfy unqualified lookup.
  - Missing names produce `UnresolvedName`.
- Adversarial tests:
  - The task does not activate imports or cross-module lookup.
  - The task does not treat type declarations as expression function fallback.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Unqualified function reference binding APIs do not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `FunctionReferenceBind` and `bind_unqualified_function_references`, reusing existing local lookup, top-level lookup, module package metadata, and `ResolutionTable` APIs.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-021-bind-unqualified-function-references.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-021-bind-unqualified-function-references.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-021-bind-unqualified-function-references.md`

## Forbidden Changes

- Do not implement type-name lookup.
- Do not implement package-qualified lookup.
- Do not implement imports.
- Do not implement cross-module lookup.
- Do not implement member or overload lookup.
- Do not add full module resolver orchestration.
- Do not modify accepted ADR-0026.
- Do not weaken or delete existing M0016 tests.

## Ambiguities And Dependencies

- Type-name lookup remains a later task because M0016 has not yet connected parser type-name metadata to name resolution.
- Package-qualified expression lookup remains a later task.
- `ResolutionTable` records reference-to-symbol mappings, not declaration node identities, so local shadowing is covered behaviorally by diagnostics and lookup order rather than by distinct same-text symbol ids.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 unqualified function reference binding task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding unqualified function binder API.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because bind_unqualified_function_references was missing.
2026-07-10 agent=Implementer phase=implementation result=pass notes=Added FunctionReferenceBind and bind_unqualified_function_references with local-first then same-package function fallback.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests; concrete adversarial review found no import, cross-module, member, type, or broader resolver behavior.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/scripts/review-task.sh created a review after adversarial checks; concrete review approved scope pending final CI.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Task Decomposer`
- Reason: `Select the next M0016 task after unqualified function reference binding.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
