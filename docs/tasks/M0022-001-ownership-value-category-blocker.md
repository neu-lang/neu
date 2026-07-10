# Task: M0022-001 Ownership Value Category Blocker

## Task Metadata

- Task ID: `M0022-001`
- Milestone: `M0022`
- Milestone File: `docs/milestones/M0022-ownership-and-move-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Language Designer`

## Objective

Record the missing accepted semantics that block M0022 ownership and move
analysis implementation.

## Authority Extract

- `docs/SPEC.md`, “ADR-0001: Ownership Model” and “ADR-0005: Copy, Move, And
  Value Categories”.
- `docs/adr/ADR-0001-ownership-model.md`, “Recommended Choice” and
  “Downstream Consequences”.
- `docs/adr/ADR-0005-copy-move-and-value-categories.md`, “Recommended Choice”
  and “Downstream Consequences”.
- `docs/milestones/M0022-ownership-and-move-analysis.md`.
- `docs/ambiguities/M0022-ownership-value-categories.md`.

## Blocker

M0022 requires tests and implementation for primitive copy behavior and
user-defined move behavior, but accepted source of truth does not define the
bootstrap primitive scalar set, user-defined value forms in scope, move sites,
diagnostic identifiers, move-origin reporting shape, or any explicitly copyable
user-defined type rule.

## Required Resolution

ADR-0035 is accepted and defines the M0022 bootstrap ownership subset,
including copyable and move-only value categories, local-name initializer and
assignment transfer sites, and `use_after_move` diagnostics. Implementation
may proceed against ADR-0035.

## Execution Log

- 2026-07-11 agent=Main phase=blocker-recorded result=blocked evidence=ADR-0005 selects primitive-copy/user-move direction but does not define a testable bootstrap value-category catalog or diagnostics. handoff=Language-Designer
- 2026-07-11 agent=Main phase=resolution-accepted result=pass evidence=user accepted ADR-0035; accepted ADR and SPEC updated. handoff=Task-Decomposer
