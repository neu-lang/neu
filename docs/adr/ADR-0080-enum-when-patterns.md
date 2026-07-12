# ADR-0080: Zero-Payload Enum `when` Patterns

## Status

Accepted.

## Decision

`when (subject) { ... }` supports zero-payload enum variant patterns written as
`EnumName.VariantName` and the wildcard `_`. The subject is evaluated once;
arms are tested and selected in source order, and only the selected arm is
evaluated. Statement-form arms may contain the existing terminating statements
and do not produce a value. Expression-form `when` requires every possible
variant to be covered by explicit variants or one wildcard arm, and every
reachable arm must produce the same exact type. A wildcard arm covers any
remaining zero-payload variant. Duplicate or unreachable arms are diagnostics.

The initial lowering supports direct zero-payload enum subjects and literal or
direct-call arm expressions already accepted by the expression frontend.
Nested `when` expressions use the same rule. `Unit` is a valid exact result
type. Ownership, initialization, and cleanup join conservatively using the
existing branch rules. Invalid internal tags trap through the existing
compiler bootstrap trap boundary; no public enum layout is exposed.

Payload patterns, destructuring, guards, nullable enum matching, binding
patterns, generic patterns, runtime type tests, downcasts, reflection, and new
pattern syntax are deferred. HIR and MIR preserve subject identity, ordered
patterns, variant identities, result type, branch spans, and join facts.

## Dependencies

ADR-0077, ADR-0079, ADR-0021, ADR-0028, ADR-0053.
