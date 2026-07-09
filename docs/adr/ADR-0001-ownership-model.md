# ADR-0001: Ownership Model

## Question

What is the core memory-safety discipline?

## Competing Designs

- Single-owner affine values with explicit borrowing.
- Region-based memory with scoped lifetimes.
- Reference-counted ownership with compiler insertion.
- Linear ownership where all values must be consumed exactly once.

## Trade-offs

Single ownership gives strong safety and predictable destruction, but requires lifetime and move rules.

Region systems can be ergonomic for short-lived data, but struggle with flexible ownership transfer.

Reference counting improves ergonomics, but adds runtime cost and can hide cycles.

Linear ownership is very rigorous, but too strict for general-purpose systems programming.

## Recommended Choice

Single-owner affine ownership with borrowing and deterministic destruction.

## Downstream Consequences

- Every value category must specify whether it is movable, copyable, borrowable, or pinned.
- Destructors become part of the semantic model.
- APIs must expose ownership transfer clearly.

## Dependencies

- ADR-0002
- ADR-0003
- ADR-0004
- ADR-0010

