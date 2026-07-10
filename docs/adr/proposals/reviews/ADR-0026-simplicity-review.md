# ADR-0026 Simplicity Guardian Review

Decision: request-revision-before-acceptance

## Reviewed Artifact

- Proposal: `docs/adr/proposals/ADR-0026-name-resolution-policy.md`
- Related ambiguity: `docs/ambiguities/M0016-name-resolution-policy.md`
- Related milestone: `docs/milestones/M0016-name-resolution-pass.md`

## Findings

The proposal chooses the right general direction by limiting M0016 to a bootstrap subset and deferring imports, cross-module lookup, member lookup, overloads, extensions, and type-directed lookup.

The proposal still needs a sharper bootstrap subset. "Local lexical scope plus module top-level resolution" can grow too large unless accepted text explicitly excludes pattern bindings, function parameters, member declarations, and complex qualified forms where not yet supported by AST.

The accepted version should keep the first implementation small enough to test thoroughly.

## Required Revisions

- Define a minimal bootstrap subset with exact included AST node kinds.
- Defer pattern bindings unless they are required for the next milestone.
- Defer function parameters until parameter AST representation exists.
- Defer active imports and cross-module dependencies.
- Keep member and overload lookup out of M0016.

## Source-Of-Truth Boundary

This review does not accept ADR-0026.

The next safe task is a concrete draft revision, not implementation.
