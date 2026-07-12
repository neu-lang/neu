# ADR-0096: Public, Private, And Protected Visibility

Status: Accepted

## Question

What source-level visibility model replaces the bootstrap `internal` default
while remaining deterministic across directory packages, inheritance,
interfaces, and compiler-private metadata?

## Competing Designs

1. Keep `internal` and add `protected`.
2. Remove `internal`, default declarations to `private`, and require `public`
   for every exported declaration.
3. Remove `internal`, default declarations to `public`, and use `private` and
   `protected` only for restrictions.
4. Make package visibility the default and add friend-package metadata.

## Trade-offs

Keeping `internal` preserves the bootstrap spelling but leaves package
visibility implicit and conflicts with the directory-package boundary. A
private default is restrictive and creates substantial migration noise for
the existing Kotlin-like surface. A public default is compatible with the
language ergonomics and makes package APIs explicit only when they need to be
restricted. Friend packages and package-default visibility require a package
policy and metadata that are not otherwise defined.

## Decision

`internal` is removed as a source-level visibility modifier. It is a parser
diagnostic in declaration positions and is not an accepted compatibility
spelling. The accepted modifiers are `public`, `private`, and `protected`.
Omitted visibility defaults to `public`.

Top-level `private` declarations are visible only in their declaring source
file. Top-level `protected` is invalid because there is no enclosing type or
subclass context. Top-level `public` declarations are importable through the
directory-package alias. Package membership does not grant access to private
declarations.

For class members, `public` is visible wherever the declaring class is
visible, `private` is visible only in the declaring class, and `protected` is
visible in the declaring class and its subclasses. Nested types are not a
special friend scope. Interfaces may declare public and private members only;
protected interface members are rejected because an interface does not define
the subclass storage/receiver boundary needed by this contract.

Constructors, fields, methods, static class functions, enum functions, and
accepted type members use the same category rules for their declaration
context. Constructors are not override targets. An override must retain the
inherited member's effective visibility; widening or narrowing visibility is a
diagnostic. A private member cannot be overridden or satisfy an interface
member. Abstract completion and interface implementation must satisfy the
effective visibility and signature contract.

Inherited public and protected members remain visible under their original
declaring-type rules. Private members do not enter inherited lookup. `this`,
`super`, virtual dispatch, interface dispatch, function values, ownership and
effect contracts, generic constraints, and capability metadata retain their
existing identity; visibility is an access predicate, not a new ABI type.

Import aliases do not bypass visibility. Lookup reports an inaccessible
declaration at the use span and includes the declaration span when available.
Ambiguous names remain ambiguity diagnostics. Same-package unqualified lookup
continues to work, subject to source-file private scope and the existing
package graph.

Effective visibility and declaration provenance are compiler metadata. They
are exported for separate compilation with the existing module/package
identity. Missing or stale visibility metadata is a compilation diagnostic.
Visibility does not change symbol naming, dispatch slots, object layout,
calling convention, object format, linking, or public ABI.

## Deferred

Friend packages, re-exports, wildcard imports, reflection, dynamic loading,
FFI visibility contracts, package registries, and visibility annotations on
unsupported declaration forms remain deferred.

## Dependencies

This ADR supersedes the visibility categories and default in ADR-0025. It
depends on ADR-0017, ADR-0022, ADR-0025, ADR-0070, ADR-0095, and the accepted
class, interface, inheritance, generic, ownership, effect, HIR, MIR, and
host-linking contracts.
