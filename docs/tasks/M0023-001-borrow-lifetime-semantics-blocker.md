# Task: M0023-001 Borrow Lifetime Semantics Blocker

## Task Metadata

- Task ID: `M0023-001`
- Milestone: `M0023`
- Milestone File: `docs/milestones/M0023-borrow-and-lifetime-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Language Designer`

## Objective

Record the missing accepted semantics that block M0023 borrow and lifetime
analysis implementation.

## Authority Extract

- `docs/SPEC.md`, “ADR-0002: Borrowing Semantics” and “ADR-0003: Lifetime
  Model”.
- `docs/adr/ADR-0002-borrowing-semantics.md`, “Recommended Choice” and
  “Downstream Consequences”.
- `docs/adr/ADR-0003-lifetime-model.md`, “Recommended Choice” and
  “Downstream Consequences”.
- `docs/milestones/M0023-borrow-and-lifetime-analysis.md`.
- `docs/ambiguities/M0023-borrow-lifetime-semantics.md`.

## Blocker

M0023 requires concrete borrow sites, lifetime validity checks, and diagnostics,
but accepted source of truth currently selects only the broad shared-or-
exclusive borrowing direction and inferred lifetime direction. It does not
define the bootstrap borrow expression syntax or implicit borrow sites,
conflict rules for local synchronous code, lifetime escape cases, diagnostic
identifiers, primary/secondary spans, or recovery behavior.

## Required Resolution

ADR-0036 is accepted and defines the M0023 metadata-only bootstrap borrow and
lifetime subset, including borrow records, exact region-equality overlap,
lifetime escape records, and `borrow_conflict` and `lifetime_escape`
diagnostics. Implementation may proceed against ADR-0036.

## Execution Log

- 2026-07-11 agent=Main phase=blocker-recorded result=blocked evidence=ADR-0002 and ADR-0003 choose the direction but do not define testable M0023 borrow sites or diagnostics. handoff=Language-Designer
- 2026-07-11 agent=Main phase=resolution-accepted result=pass evidence=ADR-0036 accepted by delegated Chief Architect decision; accepted ADR and SPEC updated. handoff=Task-Decomposer
