# ADR-0025: Module Package And Visibility Model

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language semantics, not an accepted ADR, and not a valid basis for implementation.

No implementation may depend on this proposal until accepted by Chief Architect and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0014-module-package-visibility-model.md`.

## Question

What concrete module identity, package mapping, namespace behavior, visibility category semantics, and module metadata model should the frontend use before name resolution?

## Competing Designs

1. Manifest-defined modules containing package namespaces.
2. Command-line source-set modules containing package namespaces.
3. Package-as-module model where each package is a compilation and visibility unit.
4. Source-root inferred modules with package namespaces.

## Trade-offs

Manifest-defined modules make module identity explicit and stable, but require a manifest format before the language has a package manager.

Command-line source-set modules are simple for bootstrap tests and compiler development, but need a deterministic identity rule that does not depend on host paths.

Package-as-module is simple, but makes `internal` visibility equivalent to package visibility and conflicts with ADR-0017's separation between modules and packages or namespaces.

Source-root inferred modules are convenient for tools, but risk unstable identity when files move or build systems change.

## Recommended Draft Choice

Define a small frontend module model with explicit module identity supplied by the compiler invocation or a future manifest, and packages as namespaces inside a module.

The accepted version should specify only the model needed by the near-term frontend pipeline:

- module identity
- source file to module assignment
- package-to-module mapping
- package declaration absence behavior
- visibility categories
- default visibility
- module metadata representation
- diagnostics for invalid or ambiguous module metadata

The accepted version must not rely on Kotlin, Rust, Go, file paths, or existing compiler behavior as implicit authority.

## Draft Model Direction

This section is a draft direction, not accepted semantics.

The draft direction is:

- A module is the frontend compilation and visibility unit selected by the compiler invocation.
- A module has a stable textual module name supplied explicitly by tests, command line, or future manifest data.
- A source file belongs to exactly one module for a compilation.
- A package is a namespace path declared by `package qualified.name` from ADR-0022.
- Files without a package declaration belong to an explicit root package for that module if accepted by review.
- Multiple packages may exist inside one module.
- The same package path may appear in multiple modules; name resolution later distinguishes them by module identity.
- `public`, `private`, and `internal` are the only bootstrap visibility categories.
- `internal` means visible within the same module if accepted by review.
- `private` meaning remains a required accepted decision before implementation.
- Omitted visibility defaults remain a required accepted decision before implementation.
- Imports do not create module dependencies by themselves until name resolution defines dependency lookup.

## Required Accepted Content

Before implementation, the accepted source of truth must define:

- module identity input format for tests and compiler invocation
- whether module identity is a string, interned ID, manifest reference, or target-pack artifact key
- deterministic module ID construction for test inputs
- source file to module assignment rules
- package declaration absence behavior
- package namespace representation
- duplicate package handling inside one module
- whether a source file may contain declarations without a package
- exact `public` visibility meaning
- exact `private` visibility meaning
- exact `internal` visibility meaning
- default visibility for omitted modifiers
- visibility metadata shape
- whether visibility attaches to package and import declarations or only declarations
- module dependency metadata, if any, before name resolution
- diagnostics and recovery rules for invalid module metadata

## Required Diagnostics

The accepted version must define diagnostics for:

- missing module identity when required
- invalid module identity spelling
- duplicate module identity in one compilation graph
- ambiguous source file to module assignment
- invalid package namespace
- conflicting package declaration policy
- unsupported visibility category
- duplicate visibility metadata
- default visibility ambiguity, if omitted visibility remains unresolved
- module dependency metadata ambiguity, if dependency metadata is deferred

Each diagnostic must define a primary span or external input location, recovery action, source-of-truth citation, and safe suggestion policy.

## Explicit Draft Deferrals

This draft does not define:

- package manager behavior
- manifest file syntax
- build graph format
- module artifact binary format
- cross-target module compatibility
- name resolution
- symbol tables
- re-export semantics
- friend modules
- protected visibility
- sealed type scope
- extension method scope
- protocol conformance scope
- incremental compilation identity
- IDE workspace behavior

## Downstream Consequences

- M0014 can add module metadata data structures only after this proposal is accepted or revised into source of truth.
- M0015 symbol interning depends on stable module and package identity.
- M0016 name resolution depends on visibility category semantics.
- M0021 sealed or algebraic data behavior may depend on module scope.
- M0031 target packs need module artifact compatibility rules later.

## Dependencies

- `docs/adr/ADR-0010-type-system-shape.md`
- `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
- `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- `docs/adr/ADR-0022-declaration-syntax.md`
