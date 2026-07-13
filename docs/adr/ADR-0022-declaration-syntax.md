# ADR-0022: Declaration Syntax

Status: Accepted

## Question

What concrete declaration grammar should the language use for packages, imports, visibility, functions, structs, enums or sealed sums, interfaces, and member declarations in the bootstrap compiler?

## Decision

The language uses a small Kotlin-like custom declaration grammar for the bootstrap compiler.

This ADR specifies only the declaration forms required by the parser:

- package declarations
- import declarations
- visibility modifiers
- function declarations
- struct declarations
- enum or sealed sum declarations
- interface declarations
- declaration bodies containing nested declaration members

This ADR does not rely on Kotlin, Rust, Go, or existing compiler behavior as implicit authority.

## Competing Designs

1. Adopt Kotlin declaration syntax directly.
2. Define a small Kotlin-like custom declaration grammar.
3. Define a Rust-like declaration grammar with Kotlin-like surface names.
4. Continue deferring declaration syntax until type and expression syntax are accepted.

## Trade-offs

Adopting Kotlin declaration syntax maximizes familiarity, but imports semantics and grammar interactions not accepted by this project, including modifier sets, primary constructors, properties, companion-like constructs, annotations, and expression-bodied members.

A small Kotlin-like custom declaration grammar preserves the ergonomic direction while keeping parser and AST design explicit. It requires deliberate decisions for each declaration form and defers familiar features until they are justified by accepted source of truth.

A Rust-like declaration grammar could align with ownership-oriented systems programming expectations, but conflicts with the Kotlin-like syntax constraint and risks creating an incoherent surface language.

Continuing to defer syntax avoids premature decisions, but delays name resolution, module modeling, and future type-checking work.

## Concrete Grammar

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

The parser may recognize the position of a return type after `:` but must treat the type contents as a placeholder until accepted type grammar exists.

A function body may be a declaration body or semicolon placeholder. Statement and expression parsing remain out of scope.

### Struct Declarations

```text
struct-declaration = visibility? `struct` identifier declaration-body
```

Struct primary-constructor syntax is deferred.

Struct member grammar is limited to nested declaration headers allowed by this ADR. Field syntax is deferred until type syntax is accepted.

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

This ADR intentionally excludes properties, fields, constructors, companion-like declarations, and initializers from the bootstrap declaration grammar.

### Declaration Terminators And Recovery

Declaration boundaries are:

- right brace
- semicolon
- top-level declaration-starting keyword
- end of file

Declaration recovery uses `skip-to-declaration-boundary`.

When a parser encounters a malformed declaration header, it emits one declaration diagnostic and skips to the next declaration boundary.

### Declaration Diagnostics

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

All declaration diagnostics must cite ADR-0015 and ADR-0022.

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

## Downstream Consequences

- The compiler may add concrete declaration parser fixtures and implementation tasks for only the constructs specified in this ADR.
- The compiler must continue to reject or recover from deferred declaration forms instead of treating them as accepted syntax.
- The compiler must align type annotations and generic declaration positions with this declaration grammar.
- The compiler must align expression, statement, and pattern parsing with the body placeholders in this ADR.
- Module and package modeling depends on package and import syntax accepted here.
- Name resolution depends on declaration names and visibility placement accepted here.
- Parser recovery diagnostics must cite ADR-0015 and this ADR.

## Dependencies

- ADR-0010
- ADR-0012
- ADR-0015
- ADR-0017
- ADR-0021
