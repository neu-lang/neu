# ADR-0072: Fixed Arrays Use Type Suffixes

- Status: Accepted by the main-task Chief Architect
- Supersedes: the fixed-array source-spelling portion of ADR-0063

## Decision

Fixed-size array types use `T[N]`, where `T` is an accepted element type and
`N` is the existing non-negative integer literal or named compile-time `const
Int` length form. The prior `[T; N]` spelling is rejected with a deterministic
`MalformedArrayType` diagnostic and is not a compatibility alias.

Multiple suffixes are written in outer-to-inner order: `Int[2][3]` means an
outer array of length two whose elements are arrays of length three. The
structural type identity remains recursive and includes each element type and
length. Array literals remain `[e1, e2, ...]`; expression indexing remains
`array[index]` and is parsed in expression context, independently of type
suffixes.

This ADR changes source spelling and parser recovery only. Fixed-array literal,
indexing, mutation, ownership, cleanup, inline representation, same-module
parameter/return, backend, host-linking, and deferred dynamic-array contracts
remain those accepted by ADR-0063. Dynamic arrays remain distinct and are not
introduced by `T[]` in this decision.

Malformed suffix lengths, missing brackets, and legacy bracket-semicolon forms
diagnose at the type boundary while preserving the parser's existing recovery
behavior. Whitespace does not change the type identity.

## Dependencies

ADR-0063, ADR-0023, ADR-0044, ADR-0045, ADR-0046, ADR-0055, ADR-0059, and
ADR-0071.
