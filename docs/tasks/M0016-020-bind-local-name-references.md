# Task: M0016-020 Bind local name references

## Task Metadata

- Task ID: `M0016-020`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-020-bind-local-name-references`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Project Rules: `docs/AGENTS.md`
- Agent Prompts:
  - `.codex/agents/test-engineer.md`
  - `.codex/agents/implementer.md`
  - `.codex/agents/reviewer.md`
  - `.codex/agents/adversarial-engineer.md`

## Goal

Bind parser-recorded simple identifier expression references to visible local bindings.

## Motivation

M0016 has parser metadata for simple identifier references and a local lookup API, but it does not yet produce resolution-table entries from those references. This task connects those two accepted pieces without adding top-level fallback, import resolution, member resolution, or full resolver orchestration.

## Scope

- Add a local-only reference binding result type.
- Add a function that accepts parser-recorded name references, local scopes, and local bindings.
- For each reference, find the innermost containing block scope.
- Use existing local lookup visibility and scope-walk rules.
- Insert successful local bindings into `ResolutionTable`.
- Preserve unresolved-name diagnostics from local lookup.
- Report unresolved-name diagnostics for references without a containing local block scope.

## Out Of Scope

- Top-level declaration fallback.
- Import resolution.
- Cross-module lookup.
- Member resolution.
- Type-name resolution.
- Package-qualified name resolution.
- Full-file or full-module resolver orchestration.
- Changing the parser reference metadata policy.
- Changing `ResolutionTable` to distinguish shadowed same-text symbols.

## Required Inputs

- `docs/adr/ADR-0026-name-resolution-policy.md`
- `crates/newlang/src/parser.rs`
- `crates/newlang/src/name_resolution.rs`
- `crates/newlang/tests/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Visible local references produce `ResolutionTable` entries.
  - Binder uses parser `ParsedNameReference` spans and node ids.
- Negative tests:
  - References before a local binding remain unresolved.
  - Top-level names are not used as fallback in this local-only binder.
- Adversarial tests:
  - The binder does not add import, member, cross-module, or full resolver orchestration.
  - The binder preserves local lookup's existing visibility rule.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Local reference binding APIs do not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a `LocalReferenceBind` result and `bind_local_name_references` function that use existing parser metadata, scope tree, scoped local binding index, local lookup, and resolution table APIs.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-020-bind-local-name-references.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-020-bind-local-name-references.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/name_resolution.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-020-bind-local-name-references.md`

## Forbidden Changes

- Do not implement top-level fallback.
- Do not implement imports.
- Do not implement member resolution.
- Do not add full module or file resolver orchestration.
- Do not modify accepted ADR-0026.
- Do not weaken or delete existing M0016 tests.

## Ambiguities And Dependencies

- Top-level fallback ordering remains a later task.
- Type-name and package-qualified reference metadata remain later tasks.
- `ResolutionTable` currently records reference-to-symbol mappings, so distinguishing same-text shadowed locals by binding node remains outside this task.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 local reference binding task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Updated name-resolution tests and M0016 data-model validator before adding local binder API.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=cargo test -p newlang --test name_resolution failed before implementation because bind_local_name_references was missing.
2026-07-10 agent=Implementer phase=implementation result=pass notes=Added LocalReferenceBind and bind_local_name_references using existing local lookup and ResolutionTable APIs.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=cargo test -p newlang --test name_resolution, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests; concrete adversarial review found no broader resolver behavior.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/scripts/review-task.sh created a review after adversarial checks; concrete review approved scope pending final CI.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Task Decomposer`
- Reason: `Select the next M0016 task after local-only reference binding.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/name_resolution.rs`
