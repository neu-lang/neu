# Task: M0016-019 Record parser name-reference metadata

## Task Metadata

- Task ID: `M0016-019`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner main task: `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0016-019-parser-name-reference-metadata`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Record parser metadata for simple identifier expression name references.

## Motivation

M0016 local lookup can resolve a supplied name and scope, but the parser does not yet expose name-reference metadata from `NameExpression` nodes. ADR-0026 includes simple identifier expressions as resolvable nodes and excludes member, import, package declaration, and generated names.

## Scope

- Add parser output metadata for simple identifier expression references.
- Preserve the `NameExpression` AST node id.
- Preserve exact reference text and source span.
- Keep metadata in parser encounter order.

## Out Of Scope

- Local lookup orchestration.
- Top-level fallback.
- Qualified package-name reference metadata.
- Type-name reference metadata.
- Member access names.
- Import path or alias names.
- Package declaration names.
- Function parameter or pattern bindings.

## Required Inputs

- Accepted ADR: `docs/adr/ADR-0026-name-resolution-policy.md`
- Parser `NameExpression` support from `crates/compiler/src/parser.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Simple identifier expressions record name-reference metadata.
  - Metadata preserves AST node id, name text, and source span.
  - References are recorded in parser encounter order.
- Negative tests:
  - Member names after `.` are not recorded as separate name references.
  - Import/package declaration names are not recorded as name references.
- Adversarial tests:
  - Metadata does not perform name lookup.
  - Metadata does not resolve imports, members, overloads, or type-directed candidates.

## Test-First Gate

- Test files to update before implementation:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Parser name-reference metadata APIs do not exist.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `ParsedNameReference` and populate it when `parse_name_expression` creates a `NameExpression` AST node.

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

- Generate tests: `update crates/compiler/tests/parser.rs && update docs/tests/m0016-name-resolution-data-model.sh`
- Verify tests fail: `cargo test -p compiler --test parser`
- Ordinary tests: `cargo test -p compiler --test parser && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-019-parser-name-reference-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-019-parser-name-reference-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/compiler/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-019-parser-name-reference-metadata.md`

## Forbidden Changes

- Do not implement lookup orchestration.
- Do not add top-level fallback.
- Do not record member names as references.
- Do not modify accepted ADR-0026.

## Ambiguities And Dependencies

- Type-name and package-qualified reference metadata remain later tasks.
- Combining reference metadata with lookup remains a later M0016 task.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer phase=create-task result=pass notes=Created M0016 parser name-reference metadata task.
2026-07-10 main_task=main-task test work phase=generate-tests result=pass notes=Updated parser tests and M0016 data-model validator before adding ParsedNameReference.
2026-07-10 main_task=main-task test work phase=verify-tests-fail result=pass notes=cargo test -p compiler --test parser failed before implementation because ParseOutput.name_references was missing.
2026-07-10 main_task=main-task implementation phase=implementation result=pass notes=Added ParsedNameReference metadata for simple identifier NameExpression nodes.
2026-07-10 main_task=main-task test work phase=ordinary-tests result=pass notes=cargo test -p compiler --test parser, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests were recorded; concrete adversarial review found no lookup or excluded reference metadata.
2026-07-10 main_task=main-task review phase=review result=pass notes=docs/tasks/reviews/M0016-019-review.md approved parser name-reference metadata pending final CI gate.
2026-07-10 main_task=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next main task: `main-task implementation`
- Reason: `Add parser name-reference metadata after tests fail.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/compiler/src/parser.rs`
