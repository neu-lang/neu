# ADR-0036 Chief Architect Decision

## Metadata

- Proposal: `ADR-0036`
- Milestone: `M0023`
- Review: `main-task chief-architect decision`
- Date: `2026-07-11`
- Decision: `accept`

## Inputs Read

- `AGENTS.md`
- `docs/adr/proposals/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`
- `docs/adr/proposals/reviews/ADR-0036-language-lawyer-review.md`
- `docs/adr/proposals/reviews/ADR-0036-adversarial-review.md`
- `docs/adr/proposals/reviews/ADR-0036-diagnostics-review.md`
- `docs/adr/proposals/reviews/ADR-0036-simplicity-review.md`
- `docs/adr/proposals/reviews/ADR-0036-spec-compliance-review.md`
- `docs/ambiguities/M0023-borrow-lifetime-semantics.md`
- `docs/milestones/M0023-borrow-and-lifetime-analysis.md`

## Decision

Accept ADR-0036 under the delegated future-ADR acceptance rule in `AGENTS.md`.
The required reviews approve, and the proposal resolves M0023's blocker without
inventing source syntax or broad lifetime semantics.

## Acceptance Bundle

Apply one atomic source-of-truth update:

1. Create `docs/adr/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md` with
   `Status: Accepted`, removing draft-only non-authority framing.
2. Add a concise `ADR-0036: Bootstrap Borrow And Lifetime Analysis` section to
   `docs/SPEC.md`.
3. Mark `docs/ambiguities/M0023-borrow-lifetime-semantics.md` resolved.
4. Update `docs/tasks/M0023-001-borrow-lifetime-semantics-blocker.md` with the
   accepted-resolution evidence.

## Handoff

Main task may implement M0023 against accepted ADR-0036 after the acceptance
bundle is complete.
