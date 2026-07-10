# Task: M0020-001 Generic Parameter Metadata

## Task Metadata

- Task ID: `M0020-001`
- Milestone: `M0020`
- Milestone File: `docs/milestones/M0020-generic-constraints-and-capability-bounds.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `main-task test work`, then `main-task implementation`

## Objective

Expose parser side-table metadata for each syntactically accepted generic
parameter and each of its explicitly parsed capability bounds.

## Authority Extract

- `docs/SPEC.md`, “ADR-0016: Generics And Parametric Polymorphism” and
  “ADR-0023: Type And Generic Syntax”.
- `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`, “Recommended
  Choice”.
- `crates/compiler/src/parser.rs`: `ParseOutput`, parsed metadata records,
  `parse_generic_parameters`, and `parse_capability_bound`.
- `crates/compiler/tests/parser.rs`: generic parser coverage.
- Validation: `cargo test -p compiler --test parser m0020_generic_parameter_metadata`;
  `cargo fmt --all --check`; `git diff --check`.
- Expand context only if accepted parser recovery or generic syntax is unclear;
  then consult ADR-0023 rather than inferring semantics.

## Scope

- Record generic-parameter AST node, identifier text, and identifier span.
- Record capability-bound AST node, qualified-name text, and name span.
- Associate each recorded bound with its syntactic parameter in source order.
- Expose the records through `ParseOutput`.

## Out Of Scope

- Constraint solving, capability meaning, nominal declaration resolution,
  generic argument checking, variance, inference, monomorphization, or new
  diagnostics.
- Parser grammar or recovery changes.
- `docs/SPEC.md`, ADR, backend, and example changes.

## Required Tests Before Implementation

- A declaration with both bounded and unbounded parameters records parameter
  metadata in source order.
- A bounded parameter records each `&`-joined bound in source order with exact
  qualified text and spans.
- Empty bound lists, malformed generic lists, and generic arguments do not
  synthesize parameter metadata.
- The initial failure must be an absent metadata API or record, not a changed
  parser grammar expectation.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Tests precede implementation and initially fail for the expected reason.
- [x] Metadata is syntax-only and preserves accepted parser behavior.
- [x] Parameters and bounds are source ordered with exact spans.
- [x] No generic semantics, resolution, or diagnostics are added.
- [x] Focused tests, formatter, main-task review, adversarial check, and CI pass.

## Review Routing

- Base review: `main-task review`.
- Additional review: `main-task test work` for test-first integrity.
- No semantic, diagnostics, build, or soundness specialty review unless the
  diff widens this syntax-only task.

## Execution Log

- 2026-07-10 main_task=Main phase=create-task result=pass evidence=bounded task created from ADR-0016 and ADR-0023. handoff=main-task test work
- 2026-07-10 main_task=Main phase=test-first result=fail evidence=`cargo test -p compiler --test parser m0020_generic_parameter_metadata` failed because `ParseOutput.generic_parameters` did not exist. handoff=main-task implementation
- 2026-07-10 main_task=Main phase=implementation result=pass evidence=added syntax-only generic parameter and capability-bound metadata; focused parser tests passed. handoff=main-task review
- 2026-07-10 main_task=Main phase=ordinary-tests result=pass evidence=focused metadata tests, validator, formatting, strict clippy, and 216 workspace tests passed. handoff=main-task review
- 2026-07-10 main_task=Main phase=adversarial result=pass evidence=syntax-only metadata and malformed-input boundaries verified; `docs/tasks/soundness/M0020-001-soundness.md`. handoff=main-task review
- 2026-07-10 main_task=Main phase=review result=approve evidence=scope and ADR-0016/0023 compliance verified; `docs/tasks/reviews/M0020-001-review.md`. handoff=none
