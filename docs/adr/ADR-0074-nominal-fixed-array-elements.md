# ADR-0074: Nominal Elements In Fixed Arrays

- Status: Accepted by the main-task Chief Architect
- Extends: ADR-0063 and ADR-0072

## Decision

Fixed inline arrays may use accepted class and interface types as elements,
including recursively nested fixed arrays. Type identity remains structural and
includes the nominal element identity and every length. Array literals may
construct class values with existing `new` syntax; elements initialize from
left to right and are destroyed in reverse order under the class lifecycle
contract.

Nominal-element arrays are move-only. Read-only indexed projections preserve
the existing implicit borrow rules. Indexed writes require a mutable `var`
binding and replace the owned element. Runtime-indexed projections overlap
conservatively. Class and interface method calls retain their existing direct,
virtual, interface, and `super` dispatch facts.

Nominal arrays use the existing compiler-private inline aggregate ABI. Their
layout is not public or available to FFI. Nullable nominal elements,
coroutine suspension of array borrows, dynamic-array nominal elements, slices,
strings-as-array-elements, and generic variance remain deferred until the
corresponding ABI and ownership contracts are accepted.

## Dependencies

ADR-0063, ADR-0070, ADR-0072, ADR-0062, ADR-0067, ADR-0055, and ADR-0059.
