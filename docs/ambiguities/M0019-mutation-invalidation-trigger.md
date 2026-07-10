# Ambiguity Report: M0019 Mutation Invalidation Trigger

## Metadata

- Report ID: `M0019-mutation-invalidation-trigger`
- Related Task: `M0019-016`
- Related Milestone: `M0019`
- Filed By: `main-task language review`
- Date: `2026-07-10`
- Status: `resolved`
- Resolved By: `ADR-0031`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, “ADR-0028: Nullability And Flow Typing”
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0013-mutability-model.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0028 makes only immutable local bindings eligible for M0019 refinement.
It then lists assignment, mutable treatment, guarded-region exit, and an
unproven nested region as invalidators. It expressly says ordinary assignment
to the same binding is already illegal or ineligible in that initial subset,
and reserves invalidated_refinement for future mutable or exclusive-borrow
cases.
```

Assignment cannot be a current-subset trigger: a refined binding must be
immutable, while ADR-0013 makes mutation authority explicit and ADR-0028 says
ordinary assignment to that binding is illegal or ineligible. Mutable and
exclusive-borrow refinements are deferred. No accepted source shape therefore
reaches assignment or mutable treatment after a valid M0019 refinement.

Region exit is already a specified invalidator, not merely absence of a fact:
ADR-0028 says a refinement ends at the branch closing brace, lists leaving the
guarded branch as an invalidator, defines `invalidated_refinement` recovery,
and gives `region_exit_invalidated_refinement` as its stable identifier. The
missing rule is whether every later nullable misuse after that syntactic exit
must select that diagnostic, rather than `invalid_nullable_use` for a use with
no active refinement; it also gives no separate secondary source span for a
closing brace.

## Competing Interpretations

1. Treat branch exit as the current M0019 invalidation event: a later
   refinement-dependent use reports `invalidated_refinement` with
   `region_exit_invalidated_refinement`, original-type recovery, and no
   secondary span when no operation span exists.
2. Treat exit solely as branch scoping: omit the refinement outside the branch
   and report the normal `invalid_nullable_use` required for a nullable use
   without an active refinement.
3. Defer only diagnostic selection for region exit and all future mutation
   triggers; do not add M0019-016 tests until accepted authority selects one.

## Why Guessing Is Unsafe

The alternatives change fixture-visible diagnostic kind, rule identifier, and
secondary-span expectations. Choosing assignment as a trigger would additionally
create a mutable/exclusive-borrow semantic surface that ADR-0028 defers.

## Affected Work

- Task blocked: `M0019-016`
- Tests blocked: region-exit invalidation versus ordinary nullable-use fixtures
  and all mutation-trigger fixtures.
- Implementation blocked: invalidation event classification and diagnostic
  selection; no test may encode either interpretation.

## Resolution

ADR-0031 resolves the diagnostic selection for the exact region-exit use:
`invalidated_refinement` with stable identifier
`region_exit_invalidated_refinement`, primary span on the later bare name, no
secondary span, and original-`T?` recovery. That exact mapping takes precedence
over ADR-0030's initializer mapping; ADR-0030 remains applicable otherwise.

ADR-0031 expressly limits the current M0019 invalidation trigger to region
exit. Assignment, mutable treatment, exclusive borrows, aliases, calls,
suspension, member mutation, unsafe, FFI, and other effects remain deferred
and are not current M0019 invalidation triggers.
