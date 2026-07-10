# Task: M0023-004 Borrow Analysis Report

## Task Metadata

- Task ID: `M0023-004`
- Milestone: `M0023`
- Milestone File: `docs/milestones/M0023-borrow-and-lifetime-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Provide the M0023 borrow analysis pass as a single report-producing entry point
over ADR-0036 borrow records and lifetime escape records.

## Authority Extract

- `docs/SPEC.md`, “ADR-0036: Bootstrap Borrow And Lifetime Analysis”.
- `docs/adr/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`, “Borrow
  Records”, “Lifetime Escape Records”, and “Diagnostics And Recovery”.
- `docs/milestones/M0023-borrow-and-lifetime-analysis.md`, completion
  checklist.
- `crates/compiler/src/borrow.rs`: borrow records, lifetime escape records, and
  diagnostics.

## Scope

- Add a borrow analysis report containing borrow records, lifetime escape
  records, and diagnostics.
- Add a single analysis entry point that runs borrow conflict and lifetime
  escape analysis.
- Update the M0023 milestone checklist when acceptance criteria are satisfied.

## Out Of Scope

- Source-level borrow syntax.
- Parser/type-check driver orchestration.
- Nested region overlap, path sensitivity, calls, returns, captures, async,
  unsafe, FFI, and reference types.

## Required Tests Before Implementation

- The report entry point exposes borrow records.
- The report entry point exposes lifetime escape records.
- The report combines borrow conflict and lifetime escape diagnostics.

## Acceptance Criteria

- [x] Tests fail before the report entry point exists.
- [x] The report exposes borrow records.
- [x] The report exposes lifetime escape records.
- [x] The report includes both diagnostic kinds.
- [x] M0023 checklist reflects completed borrow conflicts, lifetime escapes,
  and async deferral.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=M0023-002 and M0023-003 diagnostic analyzers are available. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p compiler --test borrow failed with unresolved analyze_borrow import. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=BorrowReport and analyze_borrow compose conflict and lifetime escape diagnostics. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=docs/tasks/soundness/M0023-004-soundness.md. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=docs/tasks/reviews/M0023-004-review.md. handoff=Commit
