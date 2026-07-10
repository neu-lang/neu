# Task: M0016-013 Record parser local binding metadata

## Task Metadata

- Task ID: `M0016-013`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-013-parser-local-binding-metadata`

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

## Goal

Record parser metadata for valid local `val` and `var` binding names.

## Motivation

ADR-0026 includes local `val` and `var` statements as M0016 binding positions. M0016-012 added storage for local bindings, but the parser still exposes only top-level declaration names. The next name-resolution slices need parsed local binding names, spans, AST node ids, and binding kind without yet constructing lexical scopes.

## Scope

- Add parser output metadata for valid local `val` and `var` statement bindings.
- Preserve the binding statement AST node id.
- Preserve the exact binding name text and name span.
- Preserve whether the binding came from `val` or `var`.
- Exclude malformed local variable declarations from metadata.

## Out Of Scope

- Lexical scope ids.
- Local binding index construction.
- Local lookup.
- Duplicate local diagnostics.
- Expression name-reference metadata.
- Function parameter bindings.
- Pattern bindings.
- Import, cross-module, member, overload, extension, or type-directed lookup.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Local binding storage from `crates/newlang/src/name_resolution.rs`
- Parser body statement support from `crates/newlang/src/parser.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Valid local `val` and `var` statements record binding metadata in source order.
  - Metadata preserves local binding kind, name text, and name span.
- Negative tests:
  - Malformed local variable declarations do not record binding metadata.
- Adversarial tests:
  - Parser metadata does not assign lexical scope ids.
  - Parser metadata does not build or query the local binding index.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Parser local binding metadata APIs do not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a small parsed local binding metadata record to parser output and fill it only after a variable declaration statement is successfully parsed and inserted into the AST arena.

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

- Generate tests: `update crates/newlang/tests/parser.rs && update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p newlang --test parser`
- Ordinary tests: `cargo test -p newlang --test parser && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-013-parser-local-binding-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-013-parser-local-binding-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-013-parser-local-binding-metadata.md`

## Forbidden Changes

- Do not assign lexical scope ids.
- Do not build `LocalBindingIndex` from parser output.
- Do not implement local lookup.
- Do not add duplicate local diagnostics.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Scope construction and declaration-order visibility remain later M0016 tasks.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 parser local binding metadata task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Updated parser tests and M0016 data-model validator before adding parser local binding metadata APIs.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=cargo test -p newlang --test parser failed before implementation because ParseOutput.local_binding_names was missing.
2026-07-10 agent=Implementer phase=implementation result=pass notes=Added ParsedLocalBindingName and parser output metadata for successfully parsed local val/var statements.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=cargo test -p newlang --test parser, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests were recorded; concrete adversarial review found no scope expansion.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/tasks/reviews/M0016-013-review.md approved parser local binding metadata scope pending final CI gate.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Implementer`
- Reason: `Add parser local binding metadata after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/parser.rs`
