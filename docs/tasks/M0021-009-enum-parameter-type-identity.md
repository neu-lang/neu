# Task: M0021-009 Enum Parameter Type Identity

## Task Metadata

- Task ID: `M0021-009`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Resolve an ADR-0034 typed parameter annotation to its same-module/package
bootstrap enum declaration identity.

## Authority Extract

- `docs/SPEC.md`, “ADR-0034: Bootstrap Enum Subject Typing”.
- `docs/adr/ADR-0034-bootstrap-enum-subject-typing.md`, “Decision”.
- `crates/compiler/src/parser.rs`: typed parameter and named-type metadata.
- `crates/compiler/src/name_resolution.rs`: module/package declaration identity.

## Scope

- Record enum identity only when a parameter annotation exactly names a
  same-module/package enum declaration.

## Out Of Scope

- Subject diagnostics, non-enum type diagnostics, name-reference binding,
  match arms, coverage, and generic/nullable/cross-module types.

## Required Tests Before Implementation

- A same-package enum annotation records the exact parameter and enum nodes.
- A non-enum annotation produces no enum identity record.

## Acceptance Criteria

- [x] Tests fail before enum parameter identity API exists.
- [x] Records preserve exact parser and declaration node identities.
- [x] No diagnostics or coverage semantics are added.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=identity record isolated from subject diagnostics. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=enum parameter identity resolver was absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=same-package enum annotations record parameter and enum declaration identities. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused test, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=non-enum annotations produce no enum identity record. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=ADR-0034 record-only scope and all validation verified. handoff=none
