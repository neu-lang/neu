# ADR-0034: Bootstrap Enum Subject Typing

Status: Proposed

## Question

What minimum accepted source and type-resolution rules let an ADR-0033 `when`
subject identify a declared bootstrap enum without importing general Kotlin
parameter, constructor, member-access, or nominal-type semantics?

## Competing Designs

1. Define a narrow typed function-parameter subset and qualified enum-variant
   value expressions, then resolve enum types nominally.
2. Allow only annotated local enum subjects and defer a source spelling for
   enum values.
3. Add general function signatures, constructors, member resolution, and
   nominal type checking.
4. Defer executable M0021 exhaustiveness semantics.

## Trade-offs

Option 1 supports the accepted ADR-0033 example while bounding the new surface
to enum use. Option 2 cannot provide a complete source-level enum workflow.
Option 3 exceeds M0021 and risks importing unrelated semantics. Option 4
preserves current constraints but leaves accepted exhaustiveness unusable.

## Recommended Choice

Choose option 1, but define only the exact grammar, scope, type identity,
diagnostics, and recovery needed for enum-typed parameters and
`Enum.Variant` values. Explicitly defer general parameter behavior,
constructors, fields, member lookup, overloads, and non-enum nominal typing.

## Downstream Consequences

M0021 can resolve subjects and qualified patterns against enum identity before
checking duplicate and missing coverage. Parser, name resolution, type
checking, diagnostics, examples, and task ordering require revision after
acceptance.

## Dependencies

ADR-0010, ADR-0015, ADR-0022, ADR-0023, ADR-0026, ADR-0027, and ADR-0033.
