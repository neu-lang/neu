# ADR-0031: Region-Exit Refinement Invalidation

Status: Accepted

## Question

When an eligible immutable local `x: T?` was refined to `T` in an `if` branch,
does a bare resolved use of `x` where `T` is required after that branch exits
report `invalidated_refinement` or ordinary `invalid_nullable_use`?

## Competing Designs

1. Report `invalidated_refinement` with `region_exit_invalidated_refinement`.
2. Report `invalid_nullable_use` because the refinement is lexically scoped.
3. Defer diagnostic selection.

## Decision

A **region-exit use** is only a simple, unqualified name expression resolving
to the same eligible immutable local whose `T?` to `T` refinement ended at the
closing brace of its guarded `if` branch. It must occur in a later statement
or trailing expression directly contained by that `if`'s enclosing block and
be required by already accepted authority to have base type `T`.

Such a use reports `invalidated_refinement` with stable identifier
`region_exit_invalidated_refinement`. Its primary span is the later bare name
expression. No secondary span is required because this subset has no separate
invalidating-operation span. Recovery treats the use as its original `T?`,
omits a successful refined-type entry for it, and continues independent
checking.

This excludes uses inside the guarded branch (including descendants), sibling
or `else` branches, uses before the `if` or its refined branch, and uses
resolving to a shadowing binding. All other uses without an active refinement
remain `invalid_nullable_use`.

Where this exact region-exit use has a matching prior immutable-local
refinement, it takes precedence over ADR-0030's initializer mapping. Otherwise
ADR-0030's initializer mapping remains applicable without change.

## Trade-offs And Rejected Alternatives

The selected diagnostic gives effect to ADR-0028's explicit region-exit
invalidator and makes loss of the preceding smart cast explainable. Ordinary
`invalid_nullable_use` is rejected for the exact region-exit shape because it
would leave that invalidator and its stable identifier without a current-subset
application. Deferral is rejected because it would continue to block the only
presently specified invalidator.

## Scope And Consequences

This is diagnostic classification for an existing syntactic region boundary.
It adds no runtime state, GC, manual memory management, ownership rule, or
thread-safety exception. It creates no trigger for assignment, mutable
treatment, exclusive borrows, aliases, calls, suspension, member mutation,
unsafe, FFI, or other effects; those forms remain deferred.

## Dependencies And Supersession

This ADR narrowly refines ADR-0028's **Mutation Invalidation** and **Flow
Diagnostics** sections for the exact region-exit use above. It does not revise
ADR-0030, whose initializer mapping remains controlling outside this ADR's
precedence case. Dependencies: ADR-0027 diagnostic-span and recovery guidance,
ADR-0028 branch regions and diagnostics, and ADR-0030.

## Acceptance Bundle

This ADR and the conforming ADR-0028 summary in `docs/SPEC.md` are
authoritative together. The resolved ambiguity report
The corresponding ambiguity report records this
decision; no deferred mutation or effect trigger is accepted by implication.
