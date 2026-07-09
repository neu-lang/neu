# AST Data Model

Status: M0009 syntax-independent AST shell

Source of truth:

- `docs/SPEC.md`
- `docs/adr/`
- `docs/syntax/grammar-authority-ledger.md`

## Scope

The M0009 AST model defines syntax-independent node identity and span retention only.

Concrete declaration, type, expression, statement, and pattern nodes are deferred because `docs/syntax/grammar-authority-ledger.md` classifies those grammar areas as ambiguous.

## Accepted Model

- AST nodes have stable `AstNodeId` values assigned in insertion order.
- AST nodes retain a `ByteSpan` from the source model.
- The only accepted node kind in M0009 is `SourceFile`.
- The source-file root span covers the source range selected by the caller.

## Deferred Nodes

The following are intentionally absent until future accepted syntax authority exists:

- declaration nodes
- type syntax nodes
- generic syntax nodes
- expression nodes
- statement nodes
- pattern nodes
- parser recovery nodes

## Semantic Boundary

The AST must not encode name resolution, type checking, ownership, borrowing, capability analysis, HIR, MIR, or backend concepts.

Future parser milestones may add concrete AST nodes only after the relevant ambiguity report is resolved by accepted source of truth.
