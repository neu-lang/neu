# Task: M0021-007 Typed Function Parameter Metadata

## Task Metadata

- Task ID: `M0021-007`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Parse and record ADR-0034 typed parameters for functions with bodies, keeping
their name, exact named-type reference, parameter node, and owning function.

## Authority Extract

- `docs/SPEC.md`, “ADR-0034: Bootstrap Enum Subject Typing”.
- `docs/adr/ADR-0034-bootstrap-enum-subject-typing.md`, “Decision” and
  “Diagnostics And Recovery”.
- `crates/compiler/src/parser.rs`: function and named-type parsing.
- `crates/compiler/src/ast.rs`: declaration/type node conventions.
- Validation: `cargo test -p compiler --test parser m0021_typed_function_parameter`;
  `cargo fmt --all --check`; `git diff --check`.

## Scope

- Parse comma-separated `identifier : named-type` parameters only for
  functions with bodies.
- Record source-order metadata and ordinary parser diagnostics/recovery.

## Out Of Scope

- Parameter binding, named-type resolution, `when` subject validation,
  nullable/generic parameters, calls, and all coverage diagnostics.

## Required Tests Before Implementation

- A typed parameter retains its function, parameter node, name, and named
  annotation node.
- Malformed parameter entries record no complete parameter metadata.

## Acceptance Criteria

- [x] Tests fail before parameter metadata APIs exist.
- [x] Only ADR-0034 parameter syntax records complete metadata.
- [x] No binding or enum-match semantics are added.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=parser metadata isolated from later semantic resolution. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=parser output and AST lacked typed function parameter metadata. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=typed body parameters retain source-order function, parameter, and named-type records. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=focused parser tests, formatter, strict clippy, and workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=malformed entries create no complete parameter records and no-body placeholders remain intact. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=ADR-0034 parser-only scope and all required validation verified. handoff=none
