# ADR-0031: Region-Exit Refinement Invalidation

Status: Draft — non-authoritative; requires main task approval.

## Question

When an eligible immutable local `x: T?` was refined to `T` in an `if` branch,
does a bare resolved use of `x` where `T` is required after that branch exits
report `invalidated_refinement` or ordinary `invalid_nullable_use`?

## Alternatives And Trade-offs

1. Report `invalidated_refinement` with
   `region_exit_invalidated_refinement`. This gives effect to ADR-0028's
   express region-exit invalidator and makes the loss of the preceding
   smart-cast explainable, but distinguishes a narrow historical use from an
   otherwise identical nullable use.
2. Report `invalid_nullable_use`. This treats refinement as ordinary lexical
   scoping and is simpler, but leaves ADR-0028's region-exit invalidator and
   stable rule identifier without a current-subset use.
3. Defer diagnostic selection. This avoids a semantic decision, but continues
   to block M0019-016's only presently specified invalidator.

## Recommendation

Choose alternative 1. A **region-exit use** is only a simple, unqualified name
expression resolving to the same eligible immutable local whose `T?` to `T`
refinement ended at the closing brace of its guarded `if` branch. It must occur
in a later statement or trailing expression directly contained by that `if`'s
enclosing block and be required by an already accepted rule to have base type
`T`. This excludes uses inside the guarded branch (including descendants),
sibling or `else` branches, uses before the `if` or its refined branch, and
uses resolving to a shadowing binding. It reports
`invalidated_refinement` with stable identifier
`region_exit_invalidated_refinement`.

Where this exact region-exit use has a matching prior immutable-local
refinement, it overrides ADR-0030's initializer mapping. Otherwise ADR-0030's
initializer mapping remains applicable without change.

Its primary span is the later bare name expression. No secondary span is
required: branch exit has no separate invalidating operation span in this
subset. Recovery treats that use as its original `T?`, omits a successful
refined-type entry for it, and continues independent checking. All other uses
without an active refinement retain `invalid_nullable_use`.

## Scope And Consequences

This is diagnostic classification for an existing syntactic region boundary;
it adds no runtime state, GC, manual memory management, ownership rule, or
thread-safety exception. It creates no trigger for assignment, mutable
treatment, exclusive borrows, aliases, calls, suspension, member mutation,
unsafe, FFI, or other effects. Those forms remain explicitly deferred.

## Proposed Supersession Targets And Dependencies

If accepted, this narrowly refines ADR-0028's **Mutation Invalidation** and
**Flow Diagnostics** sections and the ADR-0028 M0019 summary in `docs/SPEC.md`;
it does not revise them until the spec workflow. Dependencies: ADR-0027
diagnostic-span/recovery guidance, ADR-0028 branch regions and diagnostics,
and M0019-016.

## Handoff

Request main-task adversarial check, main-task diagnostics check, and main-task simplicity check
reviews, then main task acceptance. No implementation or tests follow
from this draft alone.
