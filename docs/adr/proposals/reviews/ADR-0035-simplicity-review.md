# ADR-0035 Simplicity Review

## Metadata

- Proposal: `ADR-0035`
- Milestone: `M0022`
- Review: `main-task simplicity check`
- Date: `2026-07-11`
- Decision: `approve for owner acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0035-bootstrap-ownership-and-move-analysis.md`
- `docs/ambiguities/M0022-ownership-value-categories.md`
- `docs/milestones/M0022-ownership-and-move-analysis.md`

## Review

The proposal chooses the minimum useful ownership subset for M0022: local
binding identity, type-category classification, direct local-name transfer
sites, and use-after-move diagnostics. It does not add a general ownership
framework, destructor scheduler, dataflow engine, control-flow join algorithm,
trait system, copy declaration system, or HIR dependency before the roadmap
needs them.

The deferrals are explicit enough to prevent accidental expansion during
implementation. The pass can be side-table based, matching the existing typed
frontend style.

One cost is that branch-local moves do not affect following statements. That
is a deliberate simplification and is clearly recorded as a future
path-sensitive ownership-join decision.

## Handoff

Chief Architect for final owner-acceptance decision.
