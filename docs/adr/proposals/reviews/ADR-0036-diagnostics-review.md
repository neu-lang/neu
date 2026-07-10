# ADR-0036 Diagnostics Review

## Metadata

- Proposal: `ADR-0036`
- Milestone: `M0023`
- Review: `main-task diagnostics check`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/adr/proposals/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/milestones/M0023-borrow-and-lifetime-analysis.md`

## Review

The proposal defines stable diagnostic identifiers: `borrow_conflict` and
`lifetime_escape`.

`borrow_conflict` has the later conflicting borrow as primary span and the
earlier conflicting borrow as secondary span, satisfying the M0023 requirement
to identify conflicting borrow sites.

`lifetime_escape` has the escaping use as primary span and the original borrow
as secondary span. Recovery keeps facts and continues, which is appropriate for
independent record checking.

The safe suggestion policy is constrained and does not invent fixes involving
allocation, unsafe, lifetime annotations, cloning, or mutability changes.
