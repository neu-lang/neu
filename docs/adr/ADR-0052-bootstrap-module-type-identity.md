# ADR-0052: Bootstrap Module Type Identity

Status: Accepted

## Question

What type identity domain applies to executable-source files participating in
same-module direct calls?

## Competing Designs

1. Independent source-file arenas with cross-arena raw IDs.
2. One module-wide arena for every source file in one compilation.
3. Global primitive IDs with source-local nonprimitive IDs.

## Recommended Choice

The bootstrap compiler creates one TypeArena per module compilation. All
executable-source files in that module use its identities for primitive types,
function signatures, expression types, and direct-call compatibility. A TypeId
is meaningful only with its owning module TypeArena and must not be compared
across separate module compilations. Module identity and source-file membership
remain explicit invocation/test-harness input under ADR-0025.

## Consequences

- The compiler can compare same-module argument and parameter types safely.
- Later nominal, generic, HIR, and MIR typing extends this one module type
  domain instead of adding cross-arena conversion.
- Cross-module calls remain deferred.

## Dependencies

- ADR-0025
- ADR-0027
- ADR-0041
