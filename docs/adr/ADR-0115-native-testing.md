# ADR-0115: Native compiler-owned testing

## Decision

Neu supports compiler-recognized top-level declarations of the exact form
`public test func name() { ... }`. Tests are not annotations, stdlib types, or
an I/O API. The compiler discovers valid declarations after parsing and type
checking the project graph, records fully qualified symbols and source spans,
and runs them in deterministic symbol order. Invalid visibility, nesting,
parameters, generics, suspend modifiers, explicit return annotations, or
missing bodies are diagnostics.

`assert(Bool, String)` and `fail(String)` are compiler/runtime intrinsics. A
normal return passes; an intrinsic failure or native trap fails only the
isolated test process. The CLI reports one failure line per failed test and
returns status 1 if any test fails. `neu test --list` performs discovery only,
and test-only manifests need no `main`. The ordinary build ABI and entry-point
validation are unchanged.

## Consequences

The test ABI and metadata remain internal compiler interfaces. No stdlib test
module, annotation mechanism, public string ABI, or I/O surface is introduced.
