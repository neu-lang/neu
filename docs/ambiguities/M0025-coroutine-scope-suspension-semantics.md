# Ambiguity Report: M0025 Coroutine Scope And Suspension Semantics

## Metadata

- Report ID: `M0025-coroutine-scope-suspension-semantics`
- Related Task: `M0025-001`
- Related Milestone: `M0025`
- Filed By: `main task`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0008-structured-concurrency-semantics.md`
  - `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
- Milestone:
  - `docs/milestones/M0025-coroutine-scope-and-suspension-analysis.md`

## Exact Ambiguous Text Or Missing Rule

```text
Structured concurrency as the default concurrency model; detached work must be
explicit and constrained.

Allow borrows across suspension only when the compiler proves the suspended
frame cannot be concurrently accessed or outlive borrowed data; require explicit
annotations for advanced cases.
```

Missing rules:

- Which coroutine, async, suspension, and task-scope source forms are approved.
- Whether M0025 is source-syntax based or metadata-only like M0023 and M0024.
- Which records represent structured task scopes, child tasks, suspension
  points, suspended frames, and borrowed values.
- What exact conditions reject child task lifetime violations.
- What exact conditions reject borrow-across-suspension cases.
- Which diagnostics are emitted, and which spans identify the suspension point,
  borrowed value, child task, and containing scope.
- Whether cancellation resource-safety checks are in scope for M0025.
- How thread capability facts from M0024 combine with coroutine scopes.

## Competing Interpretations

1. Define concrete coroutine syntax now.
2. Use metadata-only task-scope and suspension records for M0025.
3. Treat all coroutine-like source forms as deferred and complete only a blocker.
4. Defer M0025 entirely until runtime scheduling exists.

## Why Guessing Is Unsafe

- Coroutine suspension can extend borrow lifetimes and create use-after-free or
  data-race risks if modeled incorrectly.
- Structured child task escape rules affect ownership, borrowing, cancellation,
  and thread capability checks.
- Adding source syntax here would contradict ADR-0024 and ADR-0037 deferrals
  unless a new accepted ADR authorizes it.
- Diagnostics must explain suspension-related borrow extension with precise
  source locations.

## Affected Work

- Tasks blocked:
  - `M0025-002` and later implementation tasks.
- Milestones affected:
  - `M0025`
  - `M0027`
- Tests blocked:
  - Positive structured-scope fixtures.
  - Negative task-escape fixtures.
  - Negative borrow-across-suspension fixtures.
  - Suspension diagnostic snapshots.
- Implementation areas blocked:
  - Coroutine semantic analysis.
  - Suspension borrow checks.
  - Cancellation resource-safety checks.

## Recommended Resolution Path

- [ ] main-task language review determines whether existing text resolves it.
- [ ] main-task semantic design drafts ADR or spec revision if new semantics are required.
- [ ] main-task adversarial check reviews soundness risk.
- [ ] main-task diagnostics check reviews diagnostic consequences.
- [ ] main-task simplicity check reviews complexity.
- [ ] main task approves final resolution.

## Temporary Rule

No implementation may proceed on M0025 coroutine scope or suspension analysis
until the source of truth defines either an approved source-syntax subset or a
metadata-only bootstrap subset with diagnostics.

## Resolution

- Decision:
  - Accepted `docs/adr/ADR-0038-bootstrap-coroutine-scope-and-suspension-analysis.md`.
- Source of truth updated:
  - `docs/SPEC.md`, `ADR-0038: Bootstrap Coroutine Scope And Suspension Analysis`.
- Date resolved:
  - `2026-07-11`
