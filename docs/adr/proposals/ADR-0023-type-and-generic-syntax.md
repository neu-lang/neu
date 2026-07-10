# ADR-0023: Type And Generic Syntax

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language syntax, not an accepted ADR, and not a valid basis for parser implementation.

No parser implementation may depend on this proposal until accepted by main task and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0008-type-generic-syntax.md`.

## Question

What concrete type, nullable type, generic parameter, generic argument, function type, and capability-bound syntax should the language use for the bootstrap compiler?

## Competing Designs

1. Adopt Kotlin type and generic syntax directly.
2. Define a small Kotlin-like custom type grammar.
3. Define a Rust-like generic and bound grammar with Kotlin-like declarations.
4. Continue deferring type and generic syntax until type representation and checking milestones.

## Trade-offs

Adopting Kotlin syntax maximizes familiarity, especially for nullable types and function types, but imports features and grammar interactions not yet accepted by this project, including variance annotations, star projections, receiver function types, annotations, flexible types, and platform type concerns.

A small Kotlin-like custom type grammar preserves ergonomic direction while requiring explicit decisions for nullable marker placement, generic argument shape, generic parameter lists, and capability bounds. It avoids making Kotlin compatibility a hidden source of truth.

A Rust-like generic and bound grammar could align with systems programming expectations and capability constraints, but it conflicts with the Kotlin-like syntax constraint and would create surface inconsistency with ADR-0022 declarations.

Continuing to defer syntax avoids premature grammar decisions, but leaves M0012 blocked and prevents type representation, type checking, generic constraints, and capability analysis from making parser-backed progress.

## Recommended Draft Choice

Define a small Kotlin-like custom type grammar for the bootstrap compiler.

The accepted version should specify only the type forms required by the near-term frontend pipeline:

- named type references
- nullable type syntax
- generic parameter syntax
- generic argument syntax
- function type syntax
- capability-bound syntax
- type syntax diagnostics

The accepted version must not rely on Kotlin, Rust, Go, or existing compiler behavior as implicit authority.

## Draft Syntax Direction

This section is a draft direction, not accepted grammar.

The draft direction is:

- Use qualified names from ADR-0022 for named type references.
- Use postfix `?` for nullable type syntax if accepted by review.
- Use angle brackets for generic parameter and argument lists if accepted by review.
- Use colon-introduced bounds in generic parameter lists if accepted by review.
- Use comma-separated bound lists only if capability-bound review accepts the ambiguity and diagnostics trade-off.
- Use parenthesized function type parameter lists followed by `->` and a return type if accepted by review.
- Keep variance, wildcard types, receiver function types, intersection types, union types, dependent types, and type aliases deferred unless explicitly accepted.

## Concrete Draft Grammar

This concrete grammar is a draft only and is not accepted source of truth.

### Type Grammar Overview

```text
type = nullable-type
nullable-type = primary-type `?`?
primary-type = named-type | function-type | grouped-type
```

`?` is postfix and binds only to the immediately preceding primary type.

There is no general type precedence table in the bootstrap grammar. Grouping is explicit.

### Named Type References

```text
named-type = qualified-name generic-arguments?
qualified-name = identifier (`.` identifier)*
```

Qualified type names use the qualified-name grammar from ADR-0022.

Package and import syntax control name lookup later; this draft grammar only recognizes the syntactic shape of qualified names.

### Nullable Type Syntax

```text
nullable-type = primary-type `?`?
```

Nullable syntax may apply to:

- named types
- generic type applications
- grouped function types

Draft binding examples:

- `Box<T?>?` parses as nullable `Box` applied to nullable `T`.
- `(T) -> U?` parses as a function type returning nullable `U`.
- `((T) -> U)?` parses as a nullable function type.

Repeated nullable markers such as `T??` are malformed in the bootstrap grammar.

### Generic Parameter Syntax

```text
generic-parameters = `<` generic-parameter (`,` generic-parameter)* `>`
generic-parameter = identifier generic-bound-clause?
generic-bound-clause = `:` capability-bound-list
```

Generic parameter lists may appear after the declaration name on functions, structs, enums, and interfaces.

Generic parameter names use ADR-0021 identifiers.

Empty generic parameter lists are malformed.

Variance annotations are deferred.

### Generic Argument Syntax

```text
generic-arguments = `<` type (`,` type)* `>`
```

Generic arguments attach to named type references only.

Nested generic arguments are parsed by recursively parsing `type`; `>` closes the innermost generic argument list.

Because `>` is a lexer token from ADR-0021, nested generic closers are token-based rather than split from a combined token.

Empty generic argument lists are malformed.

### Capability-Bound Syntax

```text
capability-bound-list = capability-bound (`&` capability-bound)*
capability-bound = qualified-name
```

`&` is the only draft conjunction operator for multiple bounds.

Commas separate generic parameters, not bounds.

Draft interpretation:

```text
fun f<T: Send & Share>();
```

This declares one generic parameter `T` with two bounds, `Send` and `Share`.

The comma form below is malformed because it is ambiguous with a second generic parameter:

```text
fun f<T: Send, Share>();
```

The grammar does not assign semantic meaning to capability names. Later capability analysis decides whether a bound is an ordinary type/interface bound or an ownership, borrow, send, share, or coroutine-relevant capability.

### Function Type Syntax

```text
function-type = `(` function-type-parameters? `)` `->` type
function-type-parameters = type (`,` type)*
```

Function type parameters are types only. Parameter names in function types are deferred.

Function type syntax does not encode coroutine suspension, effects, unsafe boundaries, or ownership behavior.

Draft binding examples:

- `(T) -> U?` is a function type returning nullable `U`.
- `((T) -> U)?` is a nullable function type.

### Type Grouping And Binding

```text
grouped-type = `(` type `)`
```

Parentheses group exactly one type unless followed by `->`, in which case the parser treats the parentheses as a function type parameter list.

The parser should prefer `function-type` when a parenthesized type list is followed by `->`.

### Recovery Boundaries

Type syntax recovery boundaries are:

- comma
- right angle bracket
- right parenthesis
- left brace
- right brace
- semicolon
- declaration-starting keyword
- end of file

Generic parameter list recovery also synchronizes at `>`, declaration-starting keywords, and end of file.

Generic argument list recovery also synchronizes at `>`, declaration boundaries, and end of file.

Function type recovery also synchronizes at `->`, right parenthesis, declaration boundaries, and end of file.

### Type Syntax Diagnostics

Accepted type syntax must define these diagnostic categories before parser implementation:

| Diagnostic | Primary span | Recovery action | Safe suggestion |
| --- | --- | --- | --- |
| `missing_type_name` | expected type position | skip to type recovery boundary | none |
| `malformed_nullable_type` | extra or misplaced `?` | skip to type recovery boundary | remove extra nullable marker, if unambiguous |
| `malformed_generic_parameter_list` | malformed generic parameter list range | skip to `>` or declaration boundary | none |
| `malformed_generic_argument_list` | malformed generic argument list range | skip to `>` or declaration boundary | none |
| `missing_generic_bound` | `:` token | skip to generic parameter boundary | none |
| `malformed_capability_bound` | malformed bound range | skip to generic parameter boundary | use `&` between bounds, if unambiguous |
| `malformed_function_type` | malformed function type range | skip to type recovery boundary | none |
| `unsupported_type_form` | unsupported type-form token | skip to type recovery boundary | none |
| `unexpected_token_in_type` | unexpected token | skip to type recovery boundary | none |

Each type syntax diagnostic must define a primary span, recovery action, source-of-truth citation, and safe suggestion policy.

### Review Attack Cases

`Box<T?>?` has the draft parse: nullable `Box` application with a nullable `T` argument.

`fun f<T: Send & Share>();` has the draft parse: one generic parameter `T` with two capability bounds.

`fun f<T: Send, Share>();` is malformed in this draft because comma separates generic parameters and `Share` lacks a parameter name/bound structure.

`(T) -> U?` has the draft parse: function type returning nullable `U`.

`((T) -> U)?` has the draft parse: nullable function type.

### Concrete Deferrals

The concrete draft grammar defers:

- variance annotations
- wildcard or star-projection types
- receiver function types
- parameter names in function types
- annotation syntax on types
- type aliases
- associated types
- higher-kinded types
- dependent types
- intersection and union type syntax
- inferred placeholder types
- layout or effect types
- coroutine suspension markers in function types

## Required Accepted Content

Before this proposal can become source of truth, it must define:

- named type reference grammar
- qualified type name grammar and interaction with package/import syntax
- nullable marker placement and associativity
- whether nullable markers may apply to function types and generic type applications
- generic parameter list placement in declarations
- generic parameter name grammar
- generic parameter bound grammar
- capability-bound syntax
- generic argument list grammar
- nested generic closing delimiter policy
- function type syntax
- function type parameter naming policy, if any
- type grouping rules
- type grammar precedence or explicit absence of precedence
- recovery boundaries for malformed type syntax
- type syntax diagnostics required by ADR-0015
- explicit deferral list for type forms not in the bootstrap grammar

## Required Diagnostics

Accepted type syntax must define diagnostic categories before parser implementation.

At minimum, review must decide diagnostics for:

- missing type name
- malformed nullable type
- malformed generic parameter list
- malformed generic argument list
- missing generic bound
- malformed capability bound
- malformed function type
- unsupported type form
- unexpected token in type syntax

Each type syntax diagnostic must define a primary span, recovery action, source-of-truth citation, and safe suggestion policy.

## Explicit Draft Deferrals

This draft expects the bootstrap type grammar to defer:

- variance annotations
- wildcard or star-projection types
- receiver function types
- annotation syntax on types
- type aliases
- associated types
- higher-kinded types
- dependent types
- intersection and union type syntax
- inferred placeholder types
- layout or effect types
- coroutine suspension markers in function types

## Downstream Consequences

- M0012 can add concrete type and generic parser fixtures only after acceptance.
- M0012 can add concrete type AST nodes only after acceptance.
- M0017 type representation must align with the accepted type forms.
- M0018 type checking must not infer syntax that the parser does not accept.
- M0020 generic constraints and capability bounds depend on accepted bound syntax.
- Ownership, borrowing, thread safety, and coroutine analyses must treat capability-bound syntax as source-level commitments only after acceptance.

## Dependencies

- `docs/SPEC.md`
- `docs/adr/ADR-0006-nullability-and-absence.md`
- `docs/adr/ADR-0010-type-system-shape.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- `docs/adr/ADR-0022-declaration-syntax.md`
- `docs/ambiguities/M0008-type-generic-syntax.md`
- `docs/syntax/grammar-authority-ledger.md`
- main-task language review audit
- main-task adversarial check review
- main-task diagnostics check review
- main-task simplicity check review
- main task approval
