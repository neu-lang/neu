# Task: M0020-004 Generic Constraint Semantics Blocker

## Task Metadata

- Task ID: `M0020-004`
- Milestone: `M0020`
- Milestone File: `docs/milestones/M0020-generic-constraints-and-capability-bounds.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Language Designer`

## Objective

Record the missing accepted semantics that block M0020 constraint checking.

## Authority Extract

- `docs/SPEC.md`, “ADR-0014: Thread Safety And Data-Race Freedom”,
  “ADR-0016: Generics And Parametric Polymorphism”, “ADR-0023: Type And
  Generic Syntax”, and “ADR-0027: Type Checking Core”.
- `docs/milestones/M0020-generic-constraints-and-capability-bounds.md`.
- `docs/ambiguities/M0020-generic-constraint-semantics.md`.

## Blocker

The source of truth selects constrained generics but explicitly defers the
constraint-solving and capability semantics required to enforce bounds. No
implementation or test may decide which names are capabilities, how bounds
resolve, how types satisfy them, or which diagnostic applies.

## Required Resolution

ADR-0032 is accepted. M0020 completes representation only; a later accepted
ADR must define enforcement semantics after M0024.

## Execution Log

- 2026-07-10 agent=Main phase=blocker-recorded result=blocked evidence=SPEC explicitly defers generic constraint solving and capability semantics. handoff=Language-Designer
- 2026-07-10 agent=Main phase=proposal-drafted result=pass evidence=ADR-0032 proposes deferring enforcement rather than inventing provisional capability satisfaction. handoff=Language-Designer
- 2026-07-10 agent=Main phase=resolution-accepted result=pass evidence=user accepted ADR-0032; source of truth and roadmap updated to defer enforcement post-M0024. handoff=Roadmap-Planner
