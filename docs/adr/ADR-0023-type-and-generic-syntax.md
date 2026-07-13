# ADR-0023: Type And Generic Syntax

Status: Accepted

## Question

What concrete type, nullable type, generic parameter, generic argument, function type, and capability-bound syntax should the language use for the bootstrap compiler?

## Competing Designs

1. Adopt Kotlin type and generic syntax directly.
2. Define a small Kotlin-like custom type grammar.
3. Define a Rust-like generic and bound grammar with Kotlin-like declarations.
4. Continue deferring type and generic syntax until type representation and checking are ready.

## Trade-offs

Adopting Kotlin syntax maximizes familiarity, especially for nullable types and function types, but imports features and grammar interactions not yet accepted by this project, including variance annotations, star projections, receiver function types, annotations, flexible types, and platform type concerns.

A small Kotlin-like custom type grammar preserves ergonomic direction while requiring explicit decisions for nullable marker placement, generic argument shape, generic parameter lists, and capability bounds. It avoids making Kotlin compatibility a hidden source of truth.

A Rust-like generic and bound grammar could align with systems programming expectations and capability constraints, but it conflicts with the Kotlin-like syntax constraint and would create surface inconsistency with ADR-0022 declarations.

Continuing to defer syntax avoids premature grammar decisions, but prevents type representation, type checking, generic constraints, and capability analysis from making parser-backed progress.

## Decision

Define a small Kotlin-like custom type grammar for the bootstrap compiler.

This ADR specifies only the type forms required by the near-term frontend pipeline:

- named type references
- nullable type syntax
- generic parameter syntax
- generic argument syntax
- function type syntax
- capability-bound syntax
- type syntax diagnostics

This ADR does not rely on Kotlin, Rust, Go, or existing compiler behavior as implicit authority.

## Concrete Grammar

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

Package and import syntax control name lookup later; this grammar only recognizes the syntactic shape of qualified names.

### Nullable Type Syntax

```text
nullable-type = primary-type `?`?
```

Nullable syntax may apply to:

- named types
- generic type applications
- grouped function types

Binding examples:

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

`&` is the only accepted conjunction operator for multiple bounds.

Commas separate generic parameters, not bounds.

Interpretation:

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

Binding examples:

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

The parser must define these diagnostic categories before type syntax implementation:

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

All type syntax diagnostics must cite ADR-0015 and ADR-0023.

Each type syntax diagnostic must define a primary span, recovery action, source-of-truth citation, and safe suggestion policy.

### Review Attack Cases

`Box<T?>?` parses as nullable `Box` application with a nullable `T` argument.

`fun f<T: Send & Share>();` parses as one generic parameter `T` with two capability bounds.

`fun f<T: Send, Share>();` is malformed because comma separates generic parameters and `Share` lacks a parameter name/bound structure.

`(T) -> U?` parses as a function type returning nullable `U`.

`((T) -> U)?` parses as nullable function type.

### Concrete Deferrals

The bootstrap type grammar defers:

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

## Downstream Consequences

- The compiler may add concrete type and generic parser fixtures for ADR-0023 constructs.
- The compiler may add concrete type AST nodes only for ADR-0023 constructs.
- Type representation must align with the accepted type forms.
- Type checking must not infer syntax that the parser does not accept.
- Generic constraints and capability bounds depend on accepted bound syntax.
- Ownership, borrowing, thread safety, and coroutine analyses must treat capability-bound syntax as source-level commitments after acceptance.
- Expression, statement, pattern, coroutine, unsafe, and deferred type forms remain outside this ADR.

## Dependencies

- `docs/SPEC.md`
- `docs/adr/ADR-0006-nullability-and-absence.md`
- `docs/adr/ADR-0010-type-system-shape.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- `docs/adr/ADR-0022-declaration-syntax.md`
- type-and-generic-syntax ambiguity report
- `docs/syntax/grammar-authority-ledger.md`
- main-task language review audit
- main-task adversarial check review
- main-task diagnostics check review
- main-task simplicity check review
- main task approval
