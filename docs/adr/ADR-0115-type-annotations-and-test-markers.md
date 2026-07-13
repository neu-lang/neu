# ADR-0115: Type Annotations and Test Markers

Status: Proposed

## Question

How can libraries attach structured metadata to declarations while keeping
annotations separate from runtime values and ordinary enum semantics?

## Decision

Annotations are declaration metadata written immediately before a nominal type
declaration with an `@Name` marker. The first target is an enum declaration;
the same metadata model is reserved for classes, structs, interfaces, and
other nominal types once each target is explicitly enabled. An annotation is
not a value expression, constructor call, or inheritance relationship, and it
does not change the annotated type's ownership, layout, capabilities, or
runtime representation.

Annotation definitions live in ordinary library packages and are closed
nominal declarations. The initial `stdlib/test` package defines a `Test`
annotation marker. A declaration may carry each marker at most once, and
unknown, duplicated, malformed, or target-incompatible markers are diagnosed
before lowering. Annotation arguments are deferred; the first marker is
zero-argument and has no runtime payload.

The compiler preserves annotation identity, source span, target declaration,
and package provenance in parsed metadata and compiler IR as needed by
validation and tooling. Annotations are not implicitly exported through a
prelude and are not part of public ABI, FFI layout, equality, or enum
discriminants. A library can inspect a marker only through an explicitly
accepted compiler/tooling operation; ordinary Neu code cannot reflect over
annotations in this phase.

`stdlib/test` may use `@Test` on enum declarations as its initial executable
test target. Test discovery, fixture construction, assertion reporting,
ordering, isolation, and process exit behavior require a separate test-runner
contract; this ADR only defines the marker and its type-target validation.

## Non-goals

This ADR does not define annotation arguments, repeatable annotations,
retention policies, runtime reflection, procedural expansion, arbitrary
expression targets, or test execution. It does not make annotations part of
the source-level type identity.

## Consequences

Libraries can mark types without compiler-private runtime hooks, and a future
test package can identify marked declarations deterministically. The compiler
must reject annotation misuse rather than silently dropping metadata.

## Required follow-up

Accept this ADR (or a superseding decision), add annotation grammar and
metadata rules to `docs/SPEC.md`, add parser/type-checker diagnostics for
unknown, duplicate, and invalid-target markers, and define the separate
`stdlib/test` discovery and reporting contract before implementing a test
runner.

## Dependencies

This proposal depends on ADR-0010, ADR-0025, ADR-0026, ADR-0107, and the
stdlib/core implementation branch. It does not revise accepted ADR text.
