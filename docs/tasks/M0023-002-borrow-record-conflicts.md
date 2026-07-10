# Task: M0023-002 Borrow Record Conflicts

## Task Metadata

- Task ID: `M0023-002`
- Milestone: `M0023`
- Milestone File: `docs/milestones/M0023-borrow-and-lifetime-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Implement ADR-0036 borrow records and exact-region borrow conflict diagnostics.

## Authority Extract

- `docs/SPEC.md`, “ADR-0036: Bootstrap Borrow And Lifetime Analysis”.
- `docs/adr/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`, “Borrow
  Records” and “Diagnostics And Recovery”.
- `crates/compiler/src/name_resolution.rs`: `LocalBinding` identity.

## Scope

- Add borrow kind records for `shared` and `exclusive`.
- Add borrow record data carrying node, borrowed local binding, kind, and
  region.
- Diagnose same-local same-region conflicts when either borrow is exclusive.
- Permit multiple shared borrows of the same local in the same region.
- Diagnose on the later conflicting borrow and record the earlier borrow as
  conflict origin.

## Out Of Scope

- Lifetime escape records.
- Source-level borrow syntax.
- Nested region overlap, path sensitivity, calls, returns, captures, async,
  unsafe, FFI, and reference types.

## Required Tests Before Implementation

- Multiple shared borrows in the same region are accepted.
- Later exclusive borrow after shared borrow in the same region diagnoses.
- Later shared borrow after exclusive borrow in the same region diagnoses.
- Borrows of different locals or different regions do not conflict.

## Acceptance Criteria

- [x] Tests fail before borrow conflict analysis exists.
- [x] Shared/shared same-region borrows are accepted.
- [x] Exclusive/shared conflicts diagnose with later borrow as primary.
- [x] Diagnostics record the earlier conflicting borrow origin.
- [x] Different locals and different regions do not conflict.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=ADR-0036 accepted and M0023 unblocked. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p compiler --test borrow failed with unresolved compiler::borrow import. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=BorrowRecord, BorrowDiagnostic, and analyze_borrow_conflicts implemented. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=docs/tasks/soundness/M0023-002-soundness.md. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=docs/tasks/reviews/M0023-002-review.md. handoff=Commit
