# ADR-0020: Portability, Targets, And Platform Semantics

## Question

What does cross-compilation guarantee semantically?

## Competing Designs

- Go-like bundled target packs with standard platform definitions.
- Host-toolchain-dependent cross compilation.
- Per-platform language subsets.
- VM/intermediate runtime portability.

## Trade-offs

Bundled target packs give reproducible cross compilation and a strong user experience.

Host toolchains are flexible but fragile.

Platform subsets complicate teaching and portability.

A VM conflicts with systems-language goals.

## Recommended Choice

Go-like bundled target packs with explicit target triples, standard layout rules, platform capability declarations, and no hidden host dependency for ordinary builds.

## Downstream Consequences

- Integer widths, pointer sizes, alignment, atomics, calling conventions, and platform APIs must be specified per target.
- FFI and unsafe code must be target-aware.
- Standard library availability must be governed by target capabilities.

## Dependencies

- ADR-0017
- ADR-0018
- ADR-0019

