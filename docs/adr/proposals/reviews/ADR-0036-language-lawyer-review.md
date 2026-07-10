# ADR-0036 Language Lawyer Review

## Metadata

- Proposal: `ADR-0036`
- Milestone: `M0023`
- Review: `main-task language-lawyer review`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/adr/proposals/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`
- `docs/ambiguities/M0023-borrow-lifetime-semantics.md`
- `docs/SPEC.md`, ADR-0002, ADR-0003, ADR-0015, and ADR-0035 summaries
- `docs/milestones/M0023-borrow-and-lifetime-analysis.md`

## Review

The proposal resolves the M0023 ambiguity without adding unaccepted source
syntax. It preserves ADR-0002's shared-or-exclusive rule and ADR-0003's
inferred-lifetime direction by defining explicit compiler-side borrow and
lifetime records for the bootstrap milestone.

The region-equality overlap rule is intentionally narrow and testable. It does
not claim to be the final lifetime model, and it does not contradict future
non-lexical or path-sensitive lifetime work.

The proposal correctly leaves source-level borrow expressions, reference types,
calls, returns, members, captures, async, unsafe, and FFI to later accepted
decisions.

## Open Questions

- None blocking acceptance.
