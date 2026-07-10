# Task: M0020-003 Capability Bound Records

## Task Metadata

- Task ID: `M0020-003`
- Milestone: `M0020`
- Milestone File: `docs/milestones/M0020-generic-constraints-and-capability-bounds.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `main task`

## Objective

Represent each explicitly parsed capability-bound occurrence as an opaque record
linked to its generic parameter type.

## Authority Extract

- `docs/SPEC.md`, “ADR-0016: Generics And Parametric Polymorphism”,
  “ADR-0014: Thread Safety And Data-Race Freedom”, and “ADR-0023: Type And
  Generic Syntax”.
- `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`, “Recommended
  Choice”.
- `crates/newlang/src/parser.rs`: `ParsedGenericParameter` and
  `ParsedCapabilityBound`.
- `crates/newlang/src/type_check.rs`: M0020 generic parameter type records.
- `crates/newlang/src/symbol.rs`: `SymbolInterner`.
- Validation: `cargo test -p newlang --test type_check m0020_capability_bound_records`;
  `cargo fmt --all --check`; `git diff --check`.

## Scope

- Create records with parameter node, parameter type id, bound node, and
  interned qualified bound name.
- Preserve parameter and bound source order.
- Ignore parsed parameters lacking a corresponding type record.

## Out Of Scope

- Interpreting `Send`, `Share`, `Copy`, or any other bound; validating bound
  names; constraint satisfaction; diagnostics; generic argument substitution;
  trait/interface lookup; variance; inference; or specialization.
- Parser grammar and examples.

## Required Tests Before Implementation

- Multiple `&`-joined bounds produce ordered records linked to the correct
  parameter type.
- Unbounded parameters produce no bound records.
- Same bound spelling may reuse an interned symbol without merging occurrences.
- Missing parameter type records produce no synthetic constraint record.

## Acceptance Criteria

- [x] Tests fail before the builder exists.
- [x] Bound records preserve parameter identity, type id, bound node, and name.
- [x] Bound names are opaque and no diagnostics are emitted.
- [x] Focused tests, formatter, review, adversarial check, and CI pass.

## Execution Log

- 2026-07-10 main_task=Main phase=create-task result=pass evidence=opaque bound-record task created from accepted generic syntax and M0020-002 type records. handoff=main task
- 2026-07-10 main_task=Main phase=test-first result=fail evidence=`cargo test -p newlang --test type_check m0020_capability_bound_records` failed only because `build_m0020_capability_bound_records` was absent. handoff=main task
- 2026-07-10 main_task=Main phase=implementation result=pass evidence=opaque bound occurrence records preserve parameter/type/node identity and interned names; focused test passed. handoff=main task
- 2026-07-10 agent=Main phase=ordinary-tests result=pass evidence=focused bound-record test, validator, formatting, strict clippy, and 218 workspace tests passed. handoff=Reviewer
- 2026-07-10 agent=Main phase=adversarial result=pass evidence=missing mappings and opaque capability-bound behavior verified; `docs/tasks/soundness/M0020-003-soundness.md`. handoff=Reviewer
- 2026-07-10 agent=Main phase=review result=approve evidence=scope and ADR-0014/0016/0023 compliance verified; `docs/tasks/reviews/M0020-003-review.md`. handoff=none
