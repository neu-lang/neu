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

## Concrete Draft Model

This concrete model is a draft only and is not accepted source of truth.

### Module Identity

```text
module-name = identifier (`.` identifier)*
```

For the bootstrap compiler, a module is identified by an explicit module name supplied by the compiler invocation or test harness.

The module name uses ADR-0021 identifier spelling and dot separators. Empty module names, leading dots, trailing dots, repeated dots, and non-identifier segments are malformed.

The deterministic module ID for tests is the exact module name string after lexical validation. No host path, source root path, current directory, output path, or package name participates in module identity.

Host paths are not module identity.

host paths are not module identity.

### Source File Assignment

Each parsed source file belongs to exactly one module for one compilation.

The bootstrap frontend receives source files as an ordered set paired with one explicit module name. All files in that set belong to the same module.

A source file cannot belong to multiple modules in one frontend invocation.

### Package Namespace Model

Packages are namespaces inside a module.

The package namespace path is the qualified name from ADR-0022 package declarations.

If a file omits a package declaration, it belongs to the root package for its module.

The root package is represented as the empty package path.

Multiple files in one module may declare the same package namespace. Their top-level declarations share that package namespace for later name resolution.

The same package namespace may appear in multiple modules. Later name resolution distinguishes those declarations by module identity.

Imports are syntax only for M0014 and do not define module dependencies.

### Visibility Categories

The bootstrap visibility categories are:

- `public`
- `internal`
- `private`

Default visibility is `internal`.

The default visibility is `internal`.

`public` means visible to other modules, subject to later dependency and name resolution rules.

`internal` means visible within the same module.

`private` means visible only within the declaring source file.

Visibility attaches to top-level declarations and member declarations that syntactically accept ADR-0022 visibility modifiers. Package declarations and import declarations do not have visibility metadata.

### Visibility Metadata

```text
visibility-metadata = explicit-visibility | default-visibility
explicit-visibility = `public` | `internal` | `private`
default-visibility = `internal`
```

Each declaration has exactly one effective visibility category.

If the declaration has an explicit ADR-0022 visibility modifier, metadata records that category and marks it explicit.

If the declaration omits visibility, metadata records `internal` and marks it defaulted.

Duplicate visibility modifiers are parser diagnostics from ADR-0022; M0014 metadata receives only the parser-accepted effective visibility if parsing succeeds.

### Module Metadata Record

The bootstrap module metadata record contains:

- module name
- ordered source file identities from the source database
- package namespace for each source file
- effective visibility metadata for declarations that have parsed visibility scope

The draft record does not contain:

- module dependencies
- target triples
- package manager metadata
- manifest paths
- artifact hashes
- resolved symbols
- imported names

### Required Diagnostics

| Diagnostic | Primary span or external input location | Recovery action | Safe suggestion |
| --- | --- | --- | --- |
| `missing_module_identity` | compiler invocation module-name input | reject module metadata construction | provide an explicit module name |
| `invalid_module_identity` | invalid module-name input segment | reject module metadata construction | use dot-separated identifiers |
| `ambiguous_source_module_assignment` | source file identity or invocation input | reject conflicting assignment | assign each source file to one module |
| `invalid_package_namespace` | package qualified-name span | use root package for recovery metadata only | fix package qualified name |
| `unsupported_visibility_category` | unsupported modifier span | ignore unsupported modifier for metadata | use `public`, `internal`, or `private` |
| `duplicate_visibility_metadata` | second visibility modifier span | use first valid visibility for recovery metadata | remove duplicate visibility modifier |

Each accepted diagnostic must cite ADR-0015, ADR-0017, ADR-0022, and ADR-0025 if accepted.

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

### Explicit Draft Deferrals

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
