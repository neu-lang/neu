# ADR-0112: Explicit Numeric Utility Contracts

Status: Proposed

## Question

Which numeric helpers may `stdlib/core` expose without changing the bootstrap
integer semantics or introducing implicit conversions?

## Decision

The first numeric utility surface is limited to helpers over the accepted
`Int` type: `min`, `max`, `clamp`, `abs`, and `sign`. They take and return
`Int`, preserve left-to-right evaluation, and perform no implicit conversion.
`abs(Int.MIN)` is an arithmetic overflow and follows ADR-0043; it does not
silently wrap or widen. `clamp(value, lower, upper)` requires `lower <= upper`
as a documented precondition; violating it is a diagnosed programmer fault,
not an arbitrary result.

Checked, saturating, and wrapping arithmetic are distinct named operations.
They must not be aliases for the ordinary operators, whose overflow behavior
remains trapping under ADR-0043. This ADR does not add those operations until
their return types and overflow contracts are specified. Likewise, numeric
casts are explicit functions with a documented failure or range policy; no
assignment, call, generic specialization, or comparison performs a numeric
conversion implicitly.

Unsigned integers, floats, fixed-width aliases, machine-word integers,
rotates, population-count, bit scans, and numeric parsing remain outside the
first core surface. They require separate accepted contracts and must not be
represented by an `Int` helper with undocumented behavior.

## Non-goals

This ADR does not define allocation, formatting, string parsing, generic
numeric protocols, or compiler intrinsics. It does not revise ADR-0043 or
introduce a standard-library prelude.

## Consequences

Core numeric helpers can be implemented as ordinary pure Neu functions once
the contracts are accepted. Existing overflow and diagnostic behavior remains
the source of truth, and callers can see every conversion or failure at the
API boundary.

## Required follow-up

Accept this ADR (or a superseding decision), add helper signatures and
precondition diagnostics to `docs/SPEC.md`, and add boundary tests for
`Int.MIN`, `Int.MAX`, invalid clamp ranges, and conversion failures before
implementing the helpers.

## Dependencies

This proposal depends on ADR-0043, ADR-0048, ADR-0107, and ADR-0111. It does
not revise accepted ADR text.
