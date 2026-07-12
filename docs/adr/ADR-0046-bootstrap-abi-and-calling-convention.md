# ADR-0046: Bootstrap ABI And Calling Convention

Status: Accepted

## Question

What ABI and calling convention assumptions are accepted for the first
executable backend smoke?

## Competing Designs

1. Single initial host-target ABI subset.
2. Full host-linking ABI matrix immediately.
3. C ABI for all language functions.
4. Backend-private ABI.

## Trade-offs

A single initial host-target ABI subset is enough to run the first executable
while keeping M0033 responsible for cross-host linking.

A full ABI matrix belongs after host-linking work.

C ABI for all language functions constrains future language-level calling
conventions and symbol management.

Backend-private ABI prevents honest object/link tests.

## Recommended Choice

The bootstrap backend assumes the current host target as the initial smoke
target. Cross-target behavior remains deferred to M0033.

Bootstrap primitive lowering:

- language `Int` lowers to a signed 64-bit integer value;
- `Bool` lowers to an 8-bit integer value when needed, but `Bool` is not part
  of the first executable smoke;
- `Unit` lowers to no return value when needed, but `Unit main` is deferred;
- unsupported runtime types must not reach ABI lowering.

Bootstrap language functions use an internal Neu calling convention that may be
implemented using Cranelift's platform-default call convention for the initial
host target. This is not a stable external ABI.

Bootstrap symbol names for source functions are deterministic internal symbols
derived from module identity, package namespace, and function name. The exact
mangling format is a compiler artifact, but it must be stable within one
object/link invocation and collision-free for the bootstrap subset.

Fixed-size arrays use compiler-internal inline aggregate lowering for
same-module parameters and returns under ADR-0063. This does not define a
stable public layout or FFI ABI.

Language `main` from ADR-0040 is not the raw platform entry symbol. The object
and link pipeline must arrange a bootstrap executable entry path that calls the
language `main` and maps its `Int` result according to ADR-0047.

## Downstream Consequences

- M0031 may emit host-target Cranelift functions for bootstrap `Int` calls and
  returns.
- M0032 must provide or select the executable entry path rather than treating
  language `main` as a platform `_start`.
- Stable public ABI, FFI ABI, host linking, and symbol export policy are
  deferred.

## Dependencies

- ADR-0020
- ADR-0040
- ADR-0043
- ADR-0045
- ADR-0047
