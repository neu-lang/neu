# ADR-0036 Simplicity Review

## Metadata

- Proposal: `ADR-0036`
- Milestone: `M0023`
- Review: `main-task simplicity check`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/adr/proposals/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`
- `docs/ambiguities/M0023-borrow-lifetime-semantics.md`
- `docs/milestones/M0023-borrow-and-lifetime-analysis.md`

## Review

The proposal chooses the smallest useful M0023 surface: side-table borrow
records, exact region identity, conflict diagnostics, and lifetime escape
diagnostics. It avoids adding syntax, reference types, HIR dependency, call
semantics, path sensitivity, or coroutine interaction before they are needed.

The cost is that early borrow tests are metadata-level rather than
source-syntax-level. That is acceptable because the language has no accepted
borrow syntax yet, and this keeps the safety invariant moving without
premature grammar decisions.
