# Task: M0016-008 Add parser declaration name metadata

## Task Metadata

- Task ID: `M0016-008`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-008-parser-declaration-name-metadata`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Record parser metadata for accepted top-level declaration names so later M0016 tasks can populate the declaration index from parser output.

## Motivation

The declaration index now has an ADR-0026 key model, but parser output exposes only declaration node spans. Later collection needs the declaration AST node, name span, name text, and declaration kind without reparsing source text.

## Scope

- Add parser output metadata for top-level function and type declaration names.
- Record declaration AST node ID, declaration kind, identifier text, and identifier span.
- Exclude nested/member declaration names from this metadata.
- Preserve parser behavior and existing AST node ordering.
- Add tests for function, struct, enum, interface, and nested declaration exclusions.

## Out Of Scope

- Populating `DeclarationIndex`.
- Interning declaration names.
- Implementing lookup.
- Implementing local binding metadata.
- Implementing duplicate diagnostics.
- Changing accepted ADRs.
- Adding fixtures.

## Required Inputs

- Parser: `crates/newlang/src/parser.rs`
- AST: `crates/newlang/src/ast.rs`
- Name-resolution declaration kind: `crates/newlang/src/name_resolution.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Parser output records top-level function names.
  - Parser output records top-level type declaration names for struct, enum, and interface.
  - Name metadata preserves declaration node ID, name text, name span, and declaration kind.
- Negative tests:
  - Nested declaration names inside declaration bodies are not recorded.
  - Missing declaration names do not create metadata.
- Adversarial tests:
  - Parser metadata must not populate declaration index or perform lookup.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - `ParseOutput` does not expose declaration name metadata.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a small parser metadata record and populate it only when a top-level function, struct, enum, or interface declaration node is accepted.

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

- Generate tests: `update crates/newlang/tests/parser.rs && update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p newlang --test parser`
- Ordinary tests: `cargo test -p newlang --test parser && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-008-parser-declaration-name-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-008-parser-declaration-name-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-008-parser-declaration-name-metadata.md`

## Forbidden Changes

- Do not populate `DeclarationIndex`.
- Do not implement lookup.
- Do not add local binding metadata.
- Do not add duplicate diagnostics.
- Do not change accepted ADR-0026.

## Ambiguities And Dependencies

- Local binding metadata remains a later task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 parser declaration name metadata task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated parser and docs validators before adding declaration name metadata.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p newlang --test parser failed before implementation because ParseOutput did not expose declaration_names.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added ParsedDeclarationName metadata for top-level function and type declarations without declaration-index population or lookup.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p newlang --test parser, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created docs/tasks/soundness/M0016-008-soundness.md after ordinary tests were recorded.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-008-review.md approved parser-metadata-only scope pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add parser declaration name metadata after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/parser.rs`
