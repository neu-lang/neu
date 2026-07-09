# ADR-0017: Modules, Visibility, And API Evolution

## Question

What is the semantic unit of encapsulation and separate compilation?

## Competing Designs

- Kotlin-like packages and modules.
- Rust-like crates.
- Go-like packages as compilation units.
- Header/interface files.

## Trade-offs

Kotlin-like modules fit the syntax goal and tooling expectations.

Crates give strong compilation boundaries but may feel foreign.

Go packages are simple but less expressive for large APIs.

Header files hurt ergonomics.

## Recommended Choice

Modules as explicit compilation and visibility units, containing packages/namespaces for organization.

## Downstream Consequences

- Public/private/internal visibility must be specified.
- Sealed types, extension methods, and protocol conformance need module-scoped rules.
- Cross-compilation target packs must define module artifact compatibility.

## Dependencies

- ADR-0010
- ADR-0012
- ADR-0016
- ADR-0020

