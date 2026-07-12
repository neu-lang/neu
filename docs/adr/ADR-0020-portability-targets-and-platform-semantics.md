# ADR-0020: Portability, Targets, And Platform Semantics

## Question

What does cross-compilation guarantee semantically?

## Competing Designs

- Host-only system linking for the bootstrap compiler.
- Go-like bundled host linking with standard platform definitions.
- Host-toolchain-dependent cross compilation.
- VM/intermediate runtime portability.

## Trade-offs

Host-only linking gives the bootstrap compiler a small, explicit executable
contract while the ABI and runtime are still being established.

Bundled host linking could give reproducible cross compilation, but would also
freeze target ABI and runtime decisions prematurely.

Host toolchains are flexible but fragile.

Platform subsets complicate teaching and portability.

A VM conflicts with systems-language goals.

## Recommended Choice

Superseded by ADR-0100: the bootstrap compiler supports the current host only,
uses the host system linker, and rejects non-host targets explicitly. Future
cross compilation requires a new accepted portability decision.

## Downstream Consequences

- Host integer widths, calling convention, and runtime boundaries remain defined
  by the bootstrap ABI decisions.
- FFI and unsafe code remain deferred.
- Future targets must define their own ABI and runtime contracts.

## Dependencies

- ADR-0017
- ADR-0018
- ADR-0019
