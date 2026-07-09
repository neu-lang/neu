# AST Data Model

Status: M0011 declaration AST shell

Source of truth:

- `docs/SPEC.md`
- `docs/adr/`
- `docs/syntax/grammar-authority-ledger.md`

## Scope

The AST model defines syntax-independent node identity and span retention.

ADR-0022 declaration node kinds are accepted for parser-facing declaration shells. Type, expression, statement, and pattern nodes remain deferred because `docs/syntax/grammar-authority-ledger.md` classifies those grammar areas as ambiguous.

## Accepted Model

- AST nodes have stable `AstNodeId` values assigned in insertion order.
- AST nodes retain a `ByteSpan` from the source model.
- The accepted M0009 root node kind is `SourceFile`.
- The accepted M0011 declaration shell node kinds are:
  - `PackageDeclaration`
  - `ImportDeclaration`
  - `FunctionDeclaration`
  - `StructDeclaration`
  - `EnumDeclaration`
  - `InterfaceDeclaration`
  - `DeclarationBody`
- The source-file root span covers the source range selected by the caller.
- Declaration node spans cover the source range selected by the parser.
- Declaration nodes do not yet store names, modifiers, child relationships, type placeholders, parameter placeholders, or body contents.

## Deferred Nodes

The following are intentionally absent until future accepted syntax authority exists:

- type syntax nodes
- generic syntax nodes
- expression nodes
- statement nodes
- pattern nodes
- parser recovery nodes

## Semantic Boundary

The AST must not encode name resolution, type checking, ownership, borrowing, capability analysis, HIR, MIR, or backend concepts.

Future parser milestones may add concrete AST nodes only after the relevant ambiguity report is resolved by accepted source of truth.
