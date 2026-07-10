# Task: M0021-008 Function Parameter Binding Index

## Task Metadata

- Task ID: `M0021-008`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Map ADR-0034 typed parameters into their function body’s lexical scope as
immutable local bindings.

## Authority Extract

- `docs/SPEC.md`, “ADR-0034: Bootstrap Enum Subject Typing”.
- `docs/adr/ADR-0034-bootstrap-enum-subject-typing.md`, “Decision”.
- `crates/compiler/src/parser.rs`: `ParsedFunctionParameter`.
- `crates/compiler/src/name_resolution.rs`: local scopes and binding indexes.

## Scope

- Build source-ordered immutable binding records for parser-accepted typed
  parameters in their owning function body scope.

## Out Of Scope

- Type annotation resolution, subject validation, duplicate merging with local
  declarations, and all match diagnostics.

## Required Tests Before Implementation

- A typed parameter is indexed as immutable in its function body scope.
- Empty parameter metadata produces no bindings.

## Acceptance Criteria

- [x] Tests fail before the parameter binding API exists.
- [x] Parameter records preserve parser node identity and body scope.
- [x] No type or match semantics are added.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=scope binding isolated from enum resolution. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=parameter binding-index API was absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=parameters index as immutable bindings in their owning body scope. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused name-resolution test, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=only immutable body bindings are created; no semantic safety boundary changes. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=ADR-0034 binding scope and full validation verified. handoff=none
- 2026-07-11 agent=Main phase=test-first result=fail evidence=parameter binding-index API was absent. handoff=Implementer
