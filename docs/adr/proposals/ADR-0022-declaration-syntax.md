# ADR-0022: Declaration Syntax

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language syntax, not an accepted ADR, and not a valid basis for parser implementation.

No parser implementation may depend on this proposal until accepted by Chief Architect and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0008-declaration-syntax.md`.

## Question

What concrete declaration grammar should the language use for packages, imports, visibility, functions, structs, enums or sealed sums, interfaces, and member declarations?

## Competing Designs

1. Adopt Kotlin declaration syntax directly.
2. Define a small Kotlin-like custom declaration grammar.
3. Define a Rust-like declaration grammar with Kotlin-like surface names.
4. Continue deferring declaration syntax until type and expression syntax are accepted.

## Trade-offs

Adopting Kotlin declaration syntax maximizes familiarity, but it imports semantics and grammar interactions the project has not accepted, including modifiers, primary constructors, properties, companion-like constructs, and annotation placement.

A small Kotlin-like custom declaration grammar preserves ergonomic direction while keeping parser and AST design deliberate. It requires explicit decisions for each declaration form and may omit familiar Kotlin features until justified.

A Rust-like declaration grammar could align with ownership-oriented systems programming expectations, but it conflicts with the Kotlin-like syntax constraint and risks creating an incoherent surface language.

Continuing to defer syntax avoids premature decisions, but keeps M0011 blocked and delays name resolution, module modeling, and type checking milestones.

## Recommended Draft Choice

Define a small Kotlin-like custom declaration grammar for the bootstrap compiler.

The accepted version should specify only the declaration forms required by the near-term compiler pipeline:

- package declarations
- import declarations
- visibility modifiers
- function declarations
- struct declarations
- enum or sealed sum declarations
- interface declarations
- member declarations

The accepted version should not rely on Kotlin, Rust, Go, or existing compiler behavior as implicit authority.

## Required Accepted Content

Before this proposal can become source of truth, it must define:

- whether a source file may contain zero or one package declaration
- package declaration position and qualified-name syntax
- import declaration position, grouping, aliasing, and wildcard policy
- accepted visibility modifier spellings and placement
- accepted declaration modifier ordering, or a rule that modifiers are not yet supported
- function declaration header syntax
- function parameter list syntax at the declaration level
- whether return type syntax is allowed before M0012 resolves type grammar
- function body placeholder policy before expression and statement grammar is accepted
- struct declaration header syntax
- field or member declaration syntax, if any
- enum or sealed sum declaration syntax and allowed member forms
- interface declaration syntax and allowed member forms
- declaration terminator rules
- declaration recovery boundaries and synchronization tokens
- declaration diagnostics required by ADR-0015
- explicit deferral list for declaration forms not in the bootstrap grammar

## Downstream Consequences

- M0011 can add concrete declaration parser fixtures only after acceptance.
- M0011 can add concrete declaration AST nodes only after acceptance.
- M0012 must align type annotations and generic declaration positions with accepted declaration grammar.
- M0014 module and package modeling will depend on package and import syntax accepted here.
- M0016 name resolution will depend on declaration names and visibility placement accepted here.
- Parser recovery diagnostics must cite accepted declaration rules, not this draft.

## Dependencies

- `docs/SPEC.md`
- `docs/adr/ADR-0010-type-system-shape.md`
- `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/ambiguities/M0008-declaration-syntax.md`
- `docs/syntax/grammar-authority-ledger.md`
- Language Designer ownership review
- Language Lawyer audit
- Diagnostics Engineer review
- Simplicity Guardian review
- Chief Architect approval
