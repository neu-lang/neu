# ADR-0024: Expression Statement And Pattern Syntax

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language syntax, not an accepted ADR, and not a valid basis for parser implementation.

No parser implementation may depend on this proposal until accepted by Chief Architect and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`.

## Question

What concrete expression grammar, statement grammar, block grammar, pattern grammar, operator precedence, coroutine syntax, unsafe block syntax, parser recovery, and parser diagnostic obligations should the language use for the bootstrap compiler?

## Competing Designs

1. Adopt Kotlin expression, statement, and `when` syntax directly.
2. Define a small Kotlin-like custom body grammar.
3. Define a Rust-like expression-oriented block and pattern grammar with Kotlin-like declarations.
4. Split expression, statement, and pattern syntax into separate ADRs.
5. Continue deferring body syntax until after name resolution and type representation milestones.

## Trade-offs

Adopting Kotlin syntax maximizes surface familiarity, but imports grammar and semantic interactions that are not yet accepted, including expression bodies, receiver forms, labels, lambdas, destructuring, platform-specific smart-cast edge cases, and coroutine modifiers.

A small Kotlin-like custom body grammar preserves the project syntax direction while forcing explicit decisions for precedence, block boundaries, statement termination, pattern forms, ownership scope, unsafe block syntax, coroutine syntax, and diagnostics.

A Rust-like expression-oriented grammar could align with ownership scopes and deterministic destruction, but it would conflict with the Kotlin-like syntax constraint unless heavily adapted.

Splitting expression, statement, and pattern syntax into separate ADRs reduces review scope, but M0013 needs an integrated decision because block grammar, pattern grammar, smart casts, and diagnostics interact.

Continuing to defer avoids premature syntax decisions, but leaves executable bodies, pattern matching, flow typing, and later semantic passes blocked.

## Recommended Draft Direction

Define a small Kotlin-like custom body grammar for the bootstrap compiler.

The accepted version should specify only the body forms required by the near-term frontend pipeline:

- expression grammar
- operator precedence and associativity
- statement grammar
- block grammar
- variable declaration statement syntax, if included
- assignment statement syntax, if included
- return and error-propagation statement syntax, if included
- `if` syntax
- loop syntax, if included
- `when` or match syntax, if included
- pattern grammar
- unsafe block syntax
- coroutine syntax, if any syntax is included in M0013
- parser recovery boundaries
- parser diagnostic categories

The accepted version must not rely on Kotlin, Rust, Go, or existing compiler behavior as implicit authority.

## Required Accepted Content

Before this proposal can become source of truth, it must define:

- whether blocks are expressions, statements, or both
- statement separator and terminator rules
- expression grammar entry points
- operator precedence and associativity
- assignment grammar and whether assignment is an expression
- call syntax
- member access syntax
- indexing syntax, if included
- literal expression syntax included by M0013
- variable declaration statement grammar
- return statement grammar
- error propagation syntax, if included
- `if` expression or statement grammar
- loop syntax and loop control grammar, if included
- match or `when` syntax, if included
- pattern grammar for literals, identifiers, enum cases, wildcards, and destructuring if included
- pattern binding rules at the parser level
- unsafe block syntax, or explicit deferral
- coroutine syntax, or explicit deferral
- recovery boundaries for expressions, statements, blocks, and patterns
- parser diagnostics required by ADR-0015
- explicit ownership scope notes for block and statement boundaries
- explicit deferral list for body forms outside the bootstrap grammar

## Required Diagnostics

Accepted expression, statement, and pattern syntax must define diagnostic categories before parser implementation.

At minimum, review must decide diagnostics for:

- missing expression
- unexpected token in expression
- unsupported expression form
- malformed binary expression
- malformed call expression
- malformed member access
- malformed block
- missing statement
- unexpected token in statement
- unsupported statement form
- malformed variable declaration
- malformed assignment
- malformed return statement
- malformed conditional
- malformed loop
- malformed pattern
- unsupported pattern form
- missing pattern arm body
- malformed unsafe block
- malformed coroutine construct

Each diagnostic must define a primary span, recovery action, source-of-truth citation, and safe suggestion policy.

## Explicit Draft Deferrals

This draft expects the bootstrap body grammar to defer unless explicitly accepted later:

- macros
- operator overloading syntax
- custom infix declarations
- lambdas and closures
- receiver lambdas
- destructuring declarations
- labels
- `try`/`catch` syntax
- `defer` or scope guard syntax
- generator syntax
- async stream syntax
- inline assembly
- compile-time evaluation syntax
- annotations on expressions, statements, or patterns
- advanced pattern guards
- view patterns
- active patterns
- spread operators
- range pattern syntax
- destructuring patterns beyond enum or tuple-like forms

## Downstream Consequences

- M0013 parser fixtures can be created only after an accepted version defines concrete grammar.
- M0013 parser implementation can proceed only for accepted body constructs.
- M0016 name resolution depends on accepted binding positions in statements and patterns.
- M0018 type checking depends on expression precedence and block result rules.
- M0019 flow typing depends on accepted conditional and pattern syntax.
- M0021 exhaustiveness depends on accepted pattern forms.
- M0022 ownership and move analysis depends on accepted block and statement ownership scope.
- M0025 coroutine analysis depends on accepted coroutine syntax or explicit deferral.
- Unsafe and FFI checks depend on accepted unsafe block syntax or explicit deferral.

## Dependencies

- `docs/SPEC.md`
- `docs/adr/ADR-0007-error-handling.md`
- `docs/adr/ADR-0008-structured-concurrency-semantics.md`
- `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
- `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
- `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- `docs/syntax/grammar-authority-ledger.md`
- Language Lawyer audit
- Adversarial Engineer review
- Diagnostics Engineer review
- Simplicity Guardian review
- Chief Architect approval
