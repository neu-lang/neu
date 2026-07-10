# Task: M0020-004 Generic Constraint Semantics Blocker

## Task Metadata

- Task ID: `M0020-004`
- Milestone: `M0020`
- Milestone File: `docs/milestones/M0020-generic-constraints-and-capability-bounds.md`
- Specification: `docs/SPEC.md`
- Status: `blocked`
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

Language Designer must propose an accepted semantic decision covering capability
identity, allowed bound names or resolution, satisfaction rules, generic use
substitution, and diagnostic obligations. Roadmap Planner must then split the
remaining M0020 tasks against that decision.

## Execution Log

- 2026-07-10 agent=Main phase=blocker-recorded result=blocked evidence=SPEC explicitly defers generic constraint solving and capability semantics. handoff=Language-Designer
