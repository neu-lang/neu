# ADR-0027 Simplicity Review

## Metadata

- ADR: `ADR-0027`
- Milestone: `M0018`
- Reviewer: `Simplicity Guardian`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Findings

The proposal risks making M0018 too broad if it accepts literals, primitives, assignments, direct calls, and structural function application all at once. The bootstrap subset should be small enough for one implementation agent to complete in under five working days.

The accepted ADR should prefer exact type identity and explicit deferrals over early coercion, overload, subtyping, protocol conformance, or generic solving.

If direct calls require signature metadata that is not already present, calls should be deferred rather than forcing parser and declaration-model expansion into M0018.

## Required Revisions

- Keep the bootstrap subset minimal and testable.
- Use exact type identity for initial compatibility.
- Defer direct calls if signature metadata is not ready.
- Defer structural function type application unless M0017 type representation supports it explicitly.
- Avoid adding abstraction beyond the concrete typed output and diagnostics M0018 needs.

## Boundary

This review is not accepted source of truth. Do not implement M0018 from this review.
