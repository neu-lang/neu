# ADR-0061: Immutable `val` And Compile-Time `const`

Status: Accepted

## Question

Should Neu use `val` for immutable runtime bindings and give `const` local
declarations compile-time constant semantics?

## Decision

Both `val` and `const` are reserved local declaration keywords.

`val` is the immutable runtime-binding introducer. It maps to the existing
`LocalBindingKind::Immutable` semantic category and preserves existing scope,
initializer, ownership, borrowing, flow-typing, and mutation rules. It has no
compile-time evaluation meaning.

`const` is valid only in the existing local declaration position and requires
an initializer that is evaluable at compile time. Its accepted initializer
language is limited to literals and pure primitive operators already defined
for `Bool`, `Int`, `Float`, `Byte`, and `Unit`. Calls, local reads, allocation,
I/O, control-flow expressions, strings, nullable values, user-defined values,
and unsupported operators are rejected.

Compile-time `const` values are typed facts that may be used in ordinary
runtime expressions and in future fixed-array length expressions. They do not
create global declarations, static storage, dependent types, layout values, or
new runtime APIs. Constant dependency cycles and unsupported initializers are
diagnosed deterministically with source spans.

`var` remains the mutable runtime-binding introducer. `LocalBindingKind` is not
expanded for `val`; compile-time constant facts are separate from binding
mutability and ownership categories.

## Evaluation Boundary

The compile-time evaluator accepts the existing primitive literal forms and
pure primitive unary and binary operators whose operands and result have exact
primitive types. It does not call functions, read runtime locals, allocate,
perform I/O, evaluate control flow, or infer conversions. A `const` initializer
must produce one accepted primitive value and must not depend on a later or
cyclic constant.

Compile-time evaluation must preserve the existing integer overflow,
division, shift, exponent, byte, float, and primitive mismatch diagnostics.
It must not silently wrap or reinterpret values. Constant evaluation is a
semantic check and value fact; optimization and storage placement remain
implementation concerns.

## Diagnostics

The compile-time boundary uses these stable diagnostic rules:

- `const_initializer_required` for a `const` without an initializer;
- `const_initializer_not_constant` for a call, local read, allocation, I/O,
  control-flow, string, nullable, user-defined, or otherwise unsupported form;
- `const_dependency_cycle` for cyclic constant dependencies; and
- existing primitive diagnostics for invalid literal or operator behavior.

The primary span is the offending initializer or declaration. Independent
constant declarations continue to be checked after one failure. No diagnostic
is emitted for a valid `val` binding solely because its initializer is not
compile-time evaluable.

## Compatibility And Supersession

This ADR supersedes ADR-0029's keyword and compile-time-meaning decisions:

- `val` is reserved and is the immutable-local declaration starter;
- `const` remains reserved and is a compile-time local declaration; and
- neither spelling is accepted as an ordinary identifier in keyword positions.

The immutable binding category, `var`, lexical scope, declaration order,
ownership, borrowing, flow typing, and thread-safety rules remain governed by
their existing ADRs. Existing `const` declarations that use only accepted
literal or pure primitive initializers remain valid. Existing `const`
declarations that use runtime expressions must migrate to `val` or become
diagnostics under this decision.

No arrays, dynamic collections, slices, allocation APIs, standard-library
support, FFI, global constants, member constants, generic constants, or
dependent-type features are introduced.

## Dependencies

- ADR-0001
- ADR-0002
- ADR-0003
- ADR-0005
- ADR-0013
- ADR-0015
- ADR-0019
- ADR-0021
- ADR-0024
- ADR-0026
- ADR-0027
- ADR-0029
- ADR-0035
- ADR-0059
