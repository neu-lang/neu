# Task: M0022-004 Use-After-Move Diagnostics

## Task Metadata

- Task ID: `M0022-004`
- Milestone: `M0022`
- Milestone File: `docs/milestones/M0022-ownership-and-move-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Diagnose later bare local-name uses of a binding after an ADR-0035 ownership
transfer moves that binding.

## Authority Extract

- `docs/SPEC.md`, “ADR-0035: Bootstrap Ownership And Move Analysis”.
- `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`,
  “Move Sites”, “Move-State Model”, and “Diagnostics And Recovery”.
- `crates/newlang/src/ownership.rs`: ownership categories and transfer
  records.
- `crates/newlang/src/name_resolution.rs`: `ResolvedLocalBinding` and local
  binding identity.

## Scope

- Add ownership diagnostics for use-after-move.
- Use `OwnershipTransfer` records as move origins.
- Diagnose a resolved bare-name local use of the moved binding when the use
  node is later than the transfer source-use node in the current linear
  side-table order.
- Record primary invalid-use node and secondary move-origin node.
- Keep the moved state after a diagnostic so later uses are also diagnosed.

## Out Of Scope

- Parser/type-check orchestration.
- Branch joins, loops, path sensitivity, calls, returns, captures, `when`
  subject ownership, borrowing, destructors, generic copyability, and
  user-declared copy.
- Diagnostic rendering text or snapshots beyond the diagnostic data model.

## Required Tests Before Implementation

- A later use of a moved binding reports `use_after_move` with the transfer
  source as move origin.
- Multiple later uses of the same moved binding each report diagnostics.
- The transfer source use itself and uses before the transfer are not
  diagnosed.
- Uses of other bindings are not diagnosed.

## Acceptance Criteria

- [x] Tests fail before use-after-move analysis exists.
- [x] Later uses after a transfer diagnose on the later use.
- [x] Move origin is recorded as the transfer source use.
- [x] The moved state is not cleared after the first diagnostic.
- [x] Other bindings and pre-transfer uses are ignored.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=transfer records from M0022-003 are available. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p newlang --test ownership failed with unresolved ownership diagnostic API imports. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=analyze_use_after_move reports later resolved local uses with transfer source origin. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=docs/tasks/soundness/M0022-004-soundness.md. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=docs/tasks/reviews/M0022-004-review.md. handoff=Commit
