# ADR-0034: Bootstrap Enum Subject Typing

Status: Proposed

## Question

What minimum accepted source and type-resolution rules let an ADR-0033 `when`
subject identify a declared bootstrap enum without importing general Kotlin
parameter, constructor, member-access, or nominal-type semantics?

## Competing Designs

1. Define a narrow typed function-parameter subset and use a bare
   enum-typed parameter as the M0021 `when` subject.
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
diagnostics, and recovery needed for enum-typed parameters. Explicitly defer
general parameter behavior, qualified enum-variant value expressions,
constructors, fields, member lookup, overloads, and non-enum nominal typing.

## Concrete Draft

This section is proposed only and does not change `docs/SPEC.md` until
accepted.

### Syntax

Replace ADR-0022's parameter-list placeholder for functions with bodies:

```text
parameter-list = `(` (parameter (`,` parameter)*)? `)`
parameter = identifier `:` named-type
```

Parameters are comma-separated; empty lists remain valid. A missing name,
colon, or named type is a malformed function header and uses ordinary parser
recovery at a comma, closing parenthesis, declaration boundary, or end of
file. Function declarations without a body retain ADR-0022 placeholder
behavior.

### M0021 Subject Resolution

For the ADR-0033 subset, a `when` subject is valid only when it is a bare name
expression that resolves to a function parameter in the containing function
body. Its named-type annotation must resolve, in the parameter's declaring
module/package, to exactly one bootstrap enum declaration. The resolved subject
identity is that enum declaration. Any other subject shape, unresolved type,
or non-enum named type produces ADR-0033's `invalid_match_subject` diagnostic
on the subject expression.

The parameter has immutable local-binding identity and is visible throughout
its containing function body. This grants no general parameter typing,
assignment compatibility, flow refinement, call checking, or ABI meaning.

### Explicit Deferrals

The draft does not accept enum variants as value expressions, constructor
calls, member lookup, generic parameter types, nullable parameter types,
defaults, `vararg`, destructuring parameters, cross-module type lookup, or
general non-enum nominal type checking. These require later accepted
decisions.

## Downstream Consequences

M0021 can resolve subjects and qualified patterns against enum identity before
checking duplicate and missing coverage. Parser, name resolution, type
checking, diagnostics, examples, and task ordering require revision after
acceptance.

## Dependencies

ADR-0010, ADR-0015, ADR-0022, ADR-0023, ADR-0026, ADR-0027, and ADR-0033.
