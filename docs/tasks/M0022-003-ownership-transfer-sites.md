# Task: M0022-003 Ownership Transfer Sites

## Task Metadata

- Task ID: `M0022-003`
- Milestone: `M0022`
- Milestone File: `docs/milestones/M0022-ownership-and-move-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Record ADR-0035 ownership transfer sites for bare resolved local names of
move-only type.

## Authority Extract

- `docs/SPEC.md`, “ADR-0035: Bootstrap Ownership And Move Analysis”.
- `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`, “Move Sites”
  and “Move-State Model”.
- `crates/compiler/src/ownership.rs`: M0022 value-category classifier.
- `crates/compiler/src/parser.rs`: `ParsedLocalDeclaration` and
  `ParsedAssignmentStatement`.
- `crates/compiler/src/name_resolution.rs`: `ResolvedLocalBinding`.
- `crates/compiler/src/type_check.rs`: `DeclarationSignature`.

## Scope

- Identify local declaration initializer transfer sites.
- Identify assignment value transfer sites.
- Record the transfer site node, moved source-use expression, and moved source
  binding.
- Ignore copyable, unsupported, unresolved, and non-local-name values.

## Out Of Scope

- Use-after-move diagnostics.
- Ordering invalid uses after transfer.
- Branch joins, calls, returns, captures, `when` subject ownership, borrowing,
  destructors, generic copyability, and user-declared copy.

## Required Tests Before Implementation

- Move-only local declaration initializer records a transfer.
- Move-only assignment value records a transfer.
- Copyable and unsupported source types do not record transfers.

## Acceptance Criteria

- [x] Tests fail before transfer analysis exists.
- [x] Move-only local initializer transfers are recorded.
- [x] Move-only assignment transfers are recorded.
- [x] Copyable and unsupported source categories are ignored.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=ownership classifier from M0022-002 is available. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p compiler --test ownership failed with unresolved transfer API imports. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=collect_ownership_transfers records move-only initializer and assignment source uses. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=docs/tasks/soundness/M0022-003-soundness.md. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=docs/tasks/reviews/M0022-003-review.md. handoff=Commit
