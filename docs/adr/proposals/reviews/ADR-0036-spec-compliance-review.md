# ADR-0036 Spec Compliance Review

## Metadata

- Proposal: `ADR-0036`
- Milestone: `M0023`
- Review: `main-task spec-compliance check`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/SPEC.md`
- `docs/adr/ADR-0002-borrowing-semantics.md`
- `docs/adr/ADR-0003-lifetime-model.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`
- `docs/adr/proposals/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`

## Review

ADR-0036 is compatible with accepted source of truth. It narrows ADR-0002's
shared-or-exclusive borrowing and ADR-0003's inferred lifetime direction into a
testable metadata subset for M0023.

It does not contradict ADR-0024 because it adds no source syntax. It does not
contradict ADR-0035 because ownership transfers remain separate from borrow
facts.

Required acceptance bundle: create accepted ADR-0036, update `docs/SPEC.md`,
resolve `docs/ambiguities/M0023-borrow-lifetime-semantics.md`, and update the
M0023 blocker task.
