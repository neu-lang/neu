# Task: M0016-022 Record parser type-name reference metadata

## Task Metadata

- Task ID: `M0016-022`
- Milestone: `M0016`
- Milestone File: `docs/milestones/M0016-name-resolution-pass.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0016-022-parser-type-name-reference-metadata`

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

Record parser metadata for accepted `NamedType` AST nodes so later M0016 tasks can bind type-name references.

## Motivation

ADR-0026 includes type name nodes in accepted declaration, local binding, and explicit type annotation positions. The parser already creates `NamedType` nodes, but it does not expose reference metadata for the name portion. Name resolution cannot bind type names until that metadata exists.

## Scope

- Add parser output metadata for `NamedType` references.
- Preserve the `NamedType` AST node id.
- Preserve the exact qualified-name text and source span, excluding generic argument text.
- Record type-name references in parser encounter order.
- Include named types in function return types, local binding annotations, generic arguments, grouped types, nullable types, and function types.

## Out Of Scope

- Binding type-name references.
- Splitting package-qualified type names into package and declaration components.
- Package-qualified lookup.
- Expression name-reference behavior.
- Import path or alias names.
- Package declaration names.
- Capability-bound binding semantics.
- Function parameter binding.
- Pattern binding.

## Required Inputs

- `docs/adr/ADR-0026-name-resolution-policy.md`
- `crates/newlang/src/parser.rs`
- `crates/newlang/tests/parser.rs`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Named types in return types and local annotations record type-name metadata.
  - Generic argument, nullable, grouped, and function type names are recorded in parser encounter order.
  - Metadata preserves the `NamedType` AST node id, exact qualified-name text, and source span.
- Negative tests:
  - Package and import declarations are not recorded as type-name references.
  - Expression identifier references are not recorded as type-name references.
  - Malformed missing type names do not create type-name references.
- Adversarial tests:
  - Metadata does not bind type names.
  - Metadata does not activate package-qualified lookup, imports, members, or cross-module lookup.

## Test-First Gate

- Test files to update before implementation:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Parser type-name reference metadata APIs do not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add `ParsedTypeNameReference` to parser output and populate it when `parse_named_type` creates a `NamedType` AST node.

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
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0016-022-parser-type-name-reference-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0016-022-parser-type-name-reference-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh && docs/tests/m0016-name-resolution-blocked.sh && docs/tests/m0015-name-table-infrastructure.sh && docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Implementation files:
  - `crates/newlang/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0016-022-parser-type-name-reference-metadata.md`

## Forbidden Changes

- Do not bind type names.
- Do not add package-qualified lookup.
- Do not activate imports.
- Do not add cross-module lookup.
- Do not modify accepted ADR-0026.
- Do not weaken or delete existing M0016 tests.

## Ambiguities And Dependencies

- Package-qualified type-name binding remains a later task.
- Capability-bound resolution remains deferred until a task explicitly scopes it.
- Function parameter binding remains excluded by ADR-0026.

## Execution Log

```text
2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created M0016 parser type-name reference metadata task.
2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Updated parser tests and M0016 data-model validator before adding ParsedTypeNameReference.
2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=cargo test -p newlang --test parser failed before implementation because ParseOutput.type_name_references was missing.
2026-07-10 agent=Implementer phase=implementation result=pass notes=Added ParsedTypeNameReference metadata for NamedType nodes while preserving source encounter order across generic arguments.
2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=cargo test -p newlang --test parser, M0016 data-model validator, and M0016 accepted-state validator passed.
2026-07-10 agent=Adversarial-Engineer phase=adversarial-tests result=pass notes=docs/scripts/adversarial-check.sh created a soundness report after ordinary tests; concrete adversarial review found metadata-only behavior with no type binding.
2026-07-10 agent=Reviewer phase=review result=pass notes=docs/scripts/review-task.sh created a review after adversarial checks; concrete review approved scope pending final CI.
2026-07-10 agent=Build-Engineer phase=ci result=pass notes=cargo fmt, cargo clippy, cargo test, M0016 data-model/accepted/authority validators, M0015 validator, and M0002 validator passed.
```

## Handoff

- Next Agent: `Task Decomposer`
- Reason: `Select the next M0016 task after parser type-name reference metadata.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `crates/newlang/src/parser.rs`
