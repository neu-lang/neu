# ADR-0035 Diagnostics Review

## Metadata

- Proposal: `ADR-0035`
- Milestone: `M0022`
- Review: `main-task diagnostics check`
- Date: `2026-07-11`
- Decision: `approve for owner acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0035-bootstrap-ownership-and-move-analysis.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/milestones/M0022-ownership-and-move-analysis.md`

## Review

The proposal defines the required `use_after_move` diagnostic with a primary
span on the invalid later local-name use and a secondary span on the transfer
expression. That satisfies M0022's requirement that diagnostics identify both
the move origin and invalid use.

The recovery rule is narrow and useful: keep the original type for cascading
type recovery, but do not clear moved state. This avoids one invalid use hiding
later invalid uses in the same analyzed sequence.

The safe suggestion policy is appropriately constrained. It avoids suggesting
clone, borrow, `copy`, allocation, mutability changes, or signature changes
before those semantics exist.

`unsupported_ownership_rule` has stable example identifiers, but the proposal
also permits silence for out-of-surface constructs when no invalid use can be
proven. That is acceptable because it prevents noisy diagnostics from
pretending unsupported ownership behavior was analyzed.

## Handoff

Chief Architect for final owner-acceptance decision.
