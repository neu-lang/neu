# AST Data Model

Status: expression statement and pattern AST shell

Source of truth:

- `docs/SPEC.md`
- `docs/adr/`
- `docs/syntax/grammar-authority-ledger.md`

## Scope

The AST model defines syntax-independent node identity and span retention.

ADR-0022 declaration node kinds, ADR-0023 type and generic node kinds, and ADR-0024 expression, statement, block, and pattern node kinds are accepted for parser-facing syntax shells.

## Accepted Model

- AST nodes have stable `AstNodeId` values assigned in insertion order.
- AST nodes retain a `ByteSpan` from the source model.
- The accepted root node kind is `SourceFile`.
- The accepted declaration shell node kinds are:
  - `PackageDeclaration`
  - `ImportDeclaration`
  - `FunctionDeclaration`
  - `StructDeclaration`
  - `EnumDeclaration`
  - `InterfaceDeclaration`
  - `DeclarationBody`
- The accepted type and generic shell node kinds are:
  - `NamedType`
  - `NullableType`
  - `GenericParameter`
  - `GenericArgument`
  - `CapabilityBound`
  - `FunctionType`
  - `GroupedType`
- The accepted expression, statement, block, and pattern shell node kinds are:
  - `Block`
  - `LiteralExpression`
  - `NameExpression`
  - `GroupedExpression`
  - `IfExpression`
  - `BinaryExpression`
  - `UnaryExpression`
  - `CallExpression`
  - `MemberExpression`
  - `VariableDeclarationStatement`
  - `AssignmentStatement`
  - `ReturnStatement`
  - `ExpressionStatement`
  - `WildcardPattern`
  - `LiteralPattern`
  - `BindingPattern`
  - `QualifiedCasePattern`
  - `GroupedPattern`
- The source-file root span covers the source range selected by the caller.
- Declaration node spans cover the source range selected by the parser.
- Parser-facing node spans cover the source range selected by the parser.
- AST shell nodes do not yet store names, modifiers, child relationships, expression values, binding modes, semantic types, flow facts, or body contents.

## Deferred Nodes

The following are intentionally absent until future accepted syntax authority exists:

- parser recovery nodes
- match or `when` nodes
- loop nodes
- coroutine nodes
- unsafe block nodes
- indexing nodes
- lambda nodes

## Semantic Boundary

The AST must not encode name resolution, type checking, ownership, borrowing, flow typing, exhaustiveness, capability analysis, coroutine analysis, unsafe analysis, HIR, MIR, or backend concepts.

Future parser work may add concrete AST nodes only after the relevant ambiguity report is resolved by accepted source of truth.
