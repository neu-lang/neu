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

## Concrete Draft Grammar

This concrete grammar is a draft only and is not accepted source of truth.

### Source File Order

```text
source-file = package-declaration? import-declaration* top-level-declaration*
```

A source file may contain zero or one package declaration.

If present, the package declaration must appear before imports and top-level declarations.

Imports must appear after the package declaration, if any, and before top-level declarations.

Top-level declarations may appear in any order after the package and import section.

### Qualified Names

```text
qualified-name = identifier (`.` identifier)*
```

Qualified names use identifiers from ADR-0021 and dot separators.

### Package Declarations

```text
package-declaration = `package` qualified-name
```

Package declarations have no terminator in the bootstrap grammar. They end before the next declaration-starting keyword or end of file.

### Import Declarations

```text
import-declaration = `import` qualified-name import-alias?
import-alias = `as` identifier
```

Wildcard imports are deferred.

Grouped imports are deferred.

Import declarations have no terminator in the bootstrap grammar. They end before the next declaration-starting keyword or end of file.

### Visibility And Modifiers

```text
visibility = `public` | `private` | `internal`
```

Visibility is allowed on top-level functions, structs, enums or sealed sums, and interfaces.

Visibility is optional. When omitted, default visibility is a later semantic decision and is not encoded by the parser.

Only one visibility modifier may appear on a declaration.

All other declaration modifiers are deferred, including:

- annotations
- `abstract`
- `open`
- `override`
- `inline`
- `suspend`
- `extern` as a declaration modifier

### Function Declarations

```text
function-declaration = visibility? `fun` identifier parameter-list return-type-placeholder? function-body-placeholder
parameter-list = `(` parameter-list-placeholder? `)`
return-type-placeholder = `:` type-placeholder
function-body-placeholder = declaration-body | `;`
```

The parser may recognize the position of a parameter list but must not parse parameter declarations until type syntax is accepted.

The parser may recognize the position of a return type after `:` but must treat the type contents as a placeholder until M0012 resolves type grammar.

A function body may be a declaration body or semicolon placeholder. Statement and expression parsing remain out of scope.

### Struct Declarations

```text
struct-declaration = visibility? `struct` identifier declaration-body
```

Struct primary-constructor syntax is deferred.

Struct member grammar is limited to nested declaration headers allowed by this draft. Field syntax is deferred until type syntax is accepted.

### Enum Or Sealed Sum Declarations

```text
enum-declaration = visibility? `enum` identifier declaration-body
```

Enum variant grammar is deferred.

Sealed modifier syntax is deferred. The bootstrap grammar uses `enum` as the only sealed-sum declaration keyword.

### Interface Declarations

```text
interface-declaration = visibility? `interface` identifier declaration-body
```

Interface member grammar is limited to function declaration headers with semicolon placeholders.

Default method bodies are deferred until statement grammar is accepted.

### Declaration Bodies

```text
declaration-body = `{` declaration-member* `}`
declaration-member = function-declaration | struct-declaration | enum-declaration | interface-declaration
top-level-declaration = function-declaration | struct-declaration | enum-declaration | interface-declaration
```

This draft intentionally excludes properties, fields, constructors, companion-like declarations, and initializers from the bootstrap declaration grammar.

### Declaration Terminators And Recovery

Declaration boundaries are:

- right brace
- semicolon
- top-level declaration-starting keyword
- end of file

Declaration recovery uses `skip-to-declaration-boundary`.

When a parser encounters a malformed declaration header, it should emit one declaration diagnostic and skip to the next declaration boundary.

### Declaration Diagnostics

Accepted declaration syntax must define these diagnostic categories before parser implementation:

Each declaration diagnostic must define a primary span, recovery action, source-of-truth citation, and safe suggestion policy.

| Diagnostic | Primary span | Recovery action | Safe suggestion |
| --- | --- | --- | --- |
| `misplaced_package_declaration` | `package` keyword | `skip-to-declaration-boundary` | Move package declaration to top of file, if unambiguous. |
| `misplaced_import_declaration` | `import` keyword | `skip-to-declaration-boundary` | Move import before top-level declarations, if unambiguous. |
| `duplicate_visibility_modifier` | second visibility keyword | `skip-to-declaration-boundary` | Remove duplicate visibility modifier. |
| `unsupported_declaration_modifier` | unsupported modifier token | `skip-to-declaration-boundary` | none |
| `missing_declaration_name` | declaration keyword | `skip-to-declaration-boundary` | none |
| `malformed_declaration_header` | malformed header range | `skip-to-declaration-boundary` | none |
| `invalid_member_declaration_position` | member declaration keyword | `skip-to-declaration-boundary` | none |
| `unexpected_token_in_declaration_body` | unexpected token | `skip-to-declaration-boundary` | none |

All declaration diagnostics must cite ADR-0015 and the accepted declaration syntax source of truth.

### Explicit Deferrals

The bootstrap declaration grammar defers:

- annotations
- wildcard imports
- grouped imports
- declaration modifier sets beyond visibility
- constructors
- properties
- fields
- enum variants
- companion-like declarations
- generic parameter lists
- concrete type syntax
- expression bodies
- statement bodies
- default interface method bodies
- sealed modifier spelling

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
