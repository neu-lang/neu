# ADR-0034: Bootstrap Enum Subject Typing

Status: Accepted

## Question

What minimum source and type-resolution rules let an ADR-0033 `when` subject
identify a declared bootstrap enum without importing general parameter,
constructor, member-access, or nominal-type semantics?

## Decision

For functions with bodies, the parameter-list placeholder is replaced with:

```text
parameter-list = `(` (parameter (`,` parameter)*)? `)`
parameter = identifier `:` named-type
```

Parameters are comma-separated and an empty list remains valid. A missing
name, colon, or named type is a malformed function header with ordinary parser
recovery at a comma, closing parenthesis, declaration boundary, or end of
file. Function declarations without a body retain ADR-0022 placeholder
behavior.

For the ADR-0033 subset, a `when` subject is valid only when it is a bare name
expression resolving to a parameter in its containing function body. The
parameter's named-type annotation must resolve, in its declaring
module/package, to exactly one bootstrap enum declaration. That declaration is
the subject identity. Any other subject shape, unresolved type, or non-enum
named type reports `invalid_match_subject` on the subject expression.

Parameters have immutable local-binding identity and are visible throughout
their containing function body. This grants no general parameter typing,
assignment compatibility, flow refinement, call checking, or ABI meaning.

## Diagnostics And Recovery

Malformed parameter syntax uses ordinary parser diagnostics and recovery.
`invalid_match_subject` remains the sole ADR-0033 subject-resolution
diagnostic and uses the subject expression as its primary span. Semantic
diagnostics preserve independently valid arms and continue checking.

## Deferrals

This does not accept enum variants as value expressions, constructor calls,
member lookup, generic or nullable parameter types, defaults, `vararg`,
destructuring parameters, cross-module type lookup, or general non-enum
nominal type checking.

## Consequences

M0021 can derive a finite enum identity from an accepted source-level `when`
subject before resolving qualified arms and checking coverage.

## Dependencies And Supersession

Depends on ADR-0010, ADR-0015, ADR-0022, ADR-0023, ADR-0026, ADR-0027, and
ADR-0033. It narrowly supersedes ADR-0022's parameter-list placeholder for
functions with bodies and resolves `docs/ambiguities/M0021-enum-subject-typing.md`.
