# ADR-0035 Spec Compliance Review

## Metadata

- Proposal: `ADR-0035`
- Milestone: `M0022`
- Review: `main-task spec-compliance check`
- Date: `2026-07-11`
- Decision: `approve for Chief Architect acceptance`

## Inputs Read

- `docs/SPEC.md`
- `docs/adr/ADR-0001-ownership-model.md`
- `docs/adr/ADR-0005-copy-move-and-value-categories.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0027-type-checking-core.md`
- `docs/adr/ADR-0032-generic-constraint-enforcement-sequencing.md`
- `docs/adr/proposals/ADR-0035-bootstrap-ownership-and-move-analysis.md`

## Review

ADR-0035 is compatible with the current accepted source of truth as a
superseding/narrowing bootstrap decision. It does not contradict ADR-0001's
single-owner affine model or ADR-0005's primitive-copy/user-move direction.

It also preserves ADR-0027's boundary that primitive identities have no ABI or
layout meaning. The `String` move-only rule is a value-category rule only, not
a representation claim.

The proposal does not violate ADR-0032 because it does not enforce generic
`Copy` bounds or define capability satisfaction. Explicitly copyable
user-defined types remain deferred.

Required acceptance bundle: create accepted ADR-0035, update `docs/SPEC.md`
with a concise ADR-0035 section, and resolve
`docs/ambiguities/M0022-ownership-value-categories.md`.

## Handoff

Chief Architect for final owner-acceptance decision.
