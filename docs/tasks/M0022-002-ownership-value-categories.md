# Task: M0022-002 Ownership Value Categories

## Task Metadata

- Task ID: `M0022-002`
- Milestone: `M0022`
- Milestone File: `docs/milestones/M0022-ownership-and-move-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Implement the ADR-0035 value-category classifier for existing type identities.

## Authority Extract

- `docs/SPEC.md`, “ADR-0035: Bootstrap Ownership And Move Analysis”.
- `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`, “Decision”.
- `crates/compiler/src/types.rs`: `PrimitiveType`, `TypeKind`, `TypeRecord`,
  and `TypeArena`.
- `crates/compiler/tests/types.rs`: existing type-identity coverage patterns.

## Scope

- Define ownership value categories for ADR-0035.
- Classify `Bool`, `Int`, `Unit`, and `Null` as copyable.
- Classify `String` as move-only.
- Classify nominal user-defined type identities as move-only.
- Treat nullable and generic-parameter type records as unsupported for M0022
  value-category classification until a later accepted rule defines them.

## Out Of Scope

- Move-site scanning.
- Use-after-move diagnostics.
- Branch joins, calls, returns, captures, `when` ownership, borrowing,
  destructors, generic copyability, and user-declared copy.

## Required Tests Before Implementation

- Primitive category test covering all ADR-0035 primitive identities.
- Nominal category test showing user-defined identities are move-only.
- Unsupported category test for nullable and generic-parameter records.

## Acceptance Criteria

- [x] Tests fail before the ownership classifier exists.
- [x] Copyable primitives classify as copyable.
- [x] `String` and nominal identities classify as move-only.
- [x] Nullable and generic-parameter records classify as unsupported.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=ADR-0035 accepted and M0022 implementation unblocked. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p compiler --test ownership failed with unresolved import compiler::ownership. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=ownership category enum and classifier added over TypeArena records. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=docs/tasks/soundness/M0022-002-soundness.md. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=docs/tasks/reviews/M0022-002-review.md. handoff=Commit
