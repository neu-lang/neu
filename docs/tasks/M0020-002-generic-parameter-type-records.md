# Task: M0020-002 Generic Parameter Type Records

## Task Metadata

- Task ID: `M0020-002`
- Milestone: `M0020`
- Milestone File: `docs/milestones/M0020-generic-constraints-and-capability-bounds.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `main-task test work`, then `main-task implementation`

## Objective

Construct stable `GenericParameterType` records from M0020-001 syntax metadata
and interned parameter names.

## Authority Extract

- `docs/SPEC.md`, “ADR-0016: Generics And Parametric Polymorphism” and
  “ADR-0023: Type And Generic Syntax”.
- `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`, “Recommended
  Choice” and “Downstream Consequences”.
- `crates/compiler/src/types.rs`: `GenericParameterType`, `TypeRecord`, and
  `TypeArena`.
- `crates/compiler/src/parser.rs`: `ParsedGenericParameter`.
- `crates/compiler/src/symbol.rs`: `SymbolInterner`.
- Validation: `cargo test -p compiler --test types m0020_generic_parameter_types`;
  `cargo fmt --all --check`; `git diff --check`.

## Scope

- Intern each parsed parameter name.
- Insert one generic parameter type record per parsed parameter in source order.
- Return an explicit parameter-node to type-id mapping.
- Preserve independently declared parameters with equal names as distinct type
  records while reusing the interned symbol.

## Out Of Scope

- Capability-bound interpretation, generic name resolution, duplicate parameter
  diagnostics, substitution, generic argument checking, constraint solving,
  variance, inference, or specialization.
- Parser grammar and examples.

## Required Tests Before Implementation

- Parsed `T`, `U`, and a second declaration's `T` create source-ordered,
  distinct type records with interned symbols.
- Each mapping identifies its exact parameter node and type id.
- Empty metadata inserts no type records or symbols.
- Bound text is not inspected or interpreted by the construction helper.

## Acceptance Criteria

- [x] Tests fail before the helper exists.
- [x] One type record is built per parsed parameter in source order.
- [x] Same spelling does not merge distinct parameter declarations.
- [x] Bounds remain opaque syntax metadata.
- [x] Focused tests, formatter, review, adversarial check, and CI pass.

## Execution Log

- 2026-07-10 main_task=Main phase=create-task result=pass evidence=bounded type-record task created from ADR-0016 and M0020-001 output. handoff=main-task test work
- 2026-07-10 main_task=Main phase=test-first result=fail evidence=`cargo test -p compiler --test type_check m0020_generic_parameter_types` failed only because `build_m0020_generic_parameter_types` was absent. handoff=main-task implementation
- 2026-07-10 main_task=Main phase=implementation result=pass evidence=helper creates one generic parameter type record per parsed parameter without interpreting bounds; focused test passed. handoff=main-task review
- 2026-07-10 main_task=Main phase=ordinary-tests result=pass evidence=focused type-record test, validator, formatting, strict clippy, and 217 workspace tests passed. handoff=main-task review
- 2026-07-10 main_task=Main phase=adversarial result=pass evidence=distinct declaration identity and opaque-bound boundary verified; `docs/tasks/soundness/M0020-002-soundness.md`. handoff=main-task review
- 2026-07-10 main_task=Main phase=review result=approve evidence=scope and ADR-0016/0023 compliance verified; `docs/tasks/reviews/M0020-002-review.md`. handoff=none
