# Task: M0023-003 Lifetime Escape Diagnostics

## Task Metadata

- Task ID: `M0023-003`
- Milestone: `M0023`
- Milestone File: `docs/milestones/M0023-borrow-and-lifetime-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Implement ADR-0036 lifetime escape records and exact-region lifetime escape
diagnostics.

## Authority Extract

- `docs/SPEC.md`, “ADR-0036: Bootstrap Borrow And Lifetime Analysis”.
- `docs/adr/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`,
  “Lifetime Escape Records” and “Diagnostics And Recovery”.
- `crates/compiler/src/borrow.rs`: borrow records and diagnostics.

## Scope

- Add lifetime escape input records carrying escape node, borrowed local
  binding, borrow node, borrow region, and use region.
- Diagnose `lifetime_escape` when use region differs from borrow region.
- Diagnose on the escape node and record the original borrow node as origin.
- Accept exact same-region uses.

## Out Of Scope

- Source-level borrow syntax.
- Nested, sibling, non-lexical, path-sensitive, loop, call, return, capture,
  async, unsafe, FFI, and reference-type lifetime semantics.
- Combining borrow-conflict and lifetime-escape diagnostics into a single
  report.

## Required Tests Before Implementation

- Same-region lifetime escape records do not diagnose.
- Different-region lifetime escape records diagnose.
- Diagnostic records escape node and original borrow origin.
- Different local bindings are handled independently.

## Acceptance Criteria

- [x] Tests fail before lifetime escape analysis exists.
- [x] Same-region records are accepted.
- [x] Different-region records diagnose `lifetime_escape`.
- [x] Diagnostics record escape node and original borrow origin.
- [x] Independent records continue after a diagnostic.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=ADR-0036 accepted and borrow conflict records exist. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p compiler --test borrow failed with unresolved LifetimeEscapeRecord, analyze_lifetime_escapes, and lifetime_escape diagnostic API. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=LifetimeEscapeRecord and analyze_lifetime_escapes implemented exact region-equality rule. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=docs/tasks/soundness/M0023-003-soundness.md. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=docs/tasks/reviews/M0023-003-review.md. handoff=Commit
