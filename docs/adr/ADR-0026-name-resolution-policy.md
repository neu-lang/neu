# ADR-0026: Name Resolution Policy

Status: Accepted

## Question

What bootstrap name-resolution subset, lookup order, scope boundaries, import semantics, duplicate-name behavior, unresolved-name diagnostics, and ambiguity handling should the compiler implement before type checking?

## Competing Designs

1. Top-level-only resolution within one module.
2. Module package top-level resolution with inactive import syntax.
3. Local lexical scope plus module top-level resolution.
4. Full package, import, local, member, and cross-module resolution.

## Trade-offs

Top-level-only resolution is simple and safe, but does not exercise local bindings from ADR-0024 and leaves expression name references mostly unresolved.

Module package top-level resolution fits the existing infrastructure, but still defers local scope and import behavior.

Local lexical scope plus module top-level resolution supports the smallest useful frontend pipeline before type checking, but requires explicit shadowing, duplicate, and diagnostic rules.

Full resolution is closer to a real language implementation, but imports, dependency lookup, overloads, members, extensions, and cross-module visibility are not yet specified enough to implement safely.

## Decision

The compiler uses a bootstrap name-resolution policy with local lexical scope plus same-module package top-level declarations.

Imports remain syntax-only and do not participate in lookup. Cross-module dependency lookup, member lookup, overload resolution, extension method lookup, and type-directed lookup remain unsupported.

This ADR defines only the source-of-truth model needed by the near-term frontend pipeline before type checking. It does not define package manager behavior, module dependency metadata, active imports, re-exports, members, overloads, extensions, protocols, or compile-time generated names.

## Resolvable AST Node Kinds

Included name-reference nodes for this implementation are:

- simple identifier expression
- qualified name expression when it is syntactically a package-qualified name
- type name node in an accepted declaration, local binding, or explicit type annotation position
- package-qualified name in an accepted type or expression name-reference position

Excluded name-reference nodes for this implementation are:

- member access names
- method call names
- constructor call names
- operator names
- import path names
- import alias names
- package declaration names
- field names
- parameter names
- pattern element names
- generated or macro-created names

## Declaration And Binding Positions

Included declaration and binding positions for this implementation are:

- function declaration name
- type declaration name
- local `val` statement
- local `var` statement

Excluded declaration and binding positions for this implementation are:

- function parameters are excluded until accepted parameter AST representation exists
- pattern bindings are excluded until accepted pattern binding, ownership, and scope rules exist
- import aliases are excluded because imports remain syntax-only
- member declarations are excluded because member lookup remains unsupported
- fields are excluded because field syntax remains outside the bootstrap subset

Top-level declarations bind in the tuple `(module, package namespace, declaration name, declaration kind)`.

Local `val` and `var` statements bind in the nearest containing lexical block.

## Scope And Declaration Order

Declaration bodies introduce lexical scopes. Block expressions introduce lexical scopes for local statements inside the block.

Top-level declarations in the same module and package namespace are visible throughout that module/package namespace regardless of source-file order.

local bindings are not visible before their declaration statement. A local binding is visible after its declaration statement through the end of the containing lexical block, including nested child blocks unless shadowed by a nearer declaration.

Nested declaration bodies do not inherit local bindings from the enclosing declaration body unless a later accepted closure or capture model defines that behavior.

## Shadowing And Duplicate Rules

An inner local declaration shadows an outer local declaration with the same name after the inner declaration statement.

A local declaration shadows a top-level declaration with the same name inside the local declaration's visible range.

Shadowing never chooses a less local declaration while a more local declaration is visible.

A same-scope duplicate local binding is rejected with `duplicate_name`.

A same-module same-package duplicate top-level declaration with the same declaration kind is rejected with `duplicate_name`.

If multiple candidates remain after applying scope, package, and duplicate rules, resolution reports ambiguity instead of choosing by insertion order, source-file order, or parser traversal order.

## Lookup Rules

For an unqualified simple identifier expression, lookup order is:

1. Innermost local lexical scope outward.
2. Current source file's package namespace in the current module.
3. No implicit import, prelude, cross-module, member, overload, extension, or type-directed lookup.

For a type name node, the same lookup order applies unless a later accepted type namespace ADR defines separate value and type namespaces.

For a package-qualified name, lookup uses the explicitly named package namespace in the current module only.

Imports remain syntax-only and must not add lookup candidates.

cross-module lookup remains unsupported because module dependency metadata is not yet accepted.

member lookup remains unsupported.

overload resolution remains unsupported.

extension method lookup remains unsupported.

type-directed lookup remains unsupported.

## Visibility Rule

The compiler records the visibility metadata attached to resolved same-module top-level declarations, but visibility enforcement is deferred until cross-module lookup and API boundary rules are accepted.

Same-module lookup does not reject declarations solely because they are `private`, `internal`, or `public`; those categories remain metadata during the bootstrap resolution pass.

## Required Diagnostics

Diagnostic: `unresolved_name`

- Primary span: the full source span of the unresolved name-reference node.
- Recovery action: continue analysis with an unresolved-name placeholder so later diagnostics can avoid cascading resolution guesses.
- Source-of-truth citation: ADR-0015 and ADR-0026.
- Safe suggestion policy: suggestions may mention names found in the same lookup tier only; suggestions must not imply import, cross-module, member, overload, extension, or type-directed lookup.

Diagnostic: `duplicate_name`

- Primary span: the later declaration or local binding that violates the duplicate rule.
- Recovery action: keep the first declaration as the canonical candidate for subsequent lookup in the same scope and mark the duplicate declaration invalid.
- Source-of-truth citation: ADR-0015 and ADR-0026.
- Safe suggestion policy: suggest renaming only; do not suggest changing visibility, imports, or module/package placement.

Diagnostic: `ambiguous_name`

- Primary span: the ambiguous name-reference node.
- Recovery action: continue analysis with an unresolved-name placeholder.
- Source-of-truth citation: ADR-0015 and ADR-0026.
- Safe suggestion policy: list candidate declaration locations without selecting a winner or exposing compiler-internal table keys.

Diagnostic: `unsupported_import_resolution`

- Primary span: the import path or alias used as a lookup source.
- Recovery action: ignore the import for lookup.
- Source-of-truth citation: ADR-0015 and ADR-0026.
- Safe suggestion policy: state that import resolution is not part of this implementation; do not suggest equivalent active-import syntax.

Diagnostic: `unsupported_cross_module_lookup`

- Primary span: the package-qualified name or other syntax that requires another module.
- Recovery action: continue analysis with an unresolved-name placeholder.
- Source-of-truth citation: ADR-0015 and ADR-0026.
- Safe suggestion policy: do not suggest adding dependencies or imports.

Diagnostic: `unsupported_member_resolution`

- Primary span: the member, method, constructor, overload, extension, or type-directed name requiring unsupported lookup.
- Recovery action: leave the referenced name unresolved for this implementation.
- Source-of-truth citation: ADR-0015 and ADR-0026.
- Safe suggestion policy: state that the lookup form is outside this implementation; do not infer candidate members.

## Explicit Deferrals

This ADR does not define:

- package manager behavior
- module dependency metadata
- active import lookup
- wildcard imports
- grouped imports
- import alias lookup
- cross-module lookup
- re-exports
- member lookup
- method lookup
- constructor lookup
- overload resolution
- operator resolution
- extension method lookup
- type-directed lookup
- associated type lookup
- protocol or interface conformance lookup
- macro or compile-time generated names
- IDE incremental name resolution

## Downstream Consequences

- The compiler can implement only the accepted bootstrap subset in this ADR.
- Type representation depends on stable resolved type-name symbols.
- Type checking depends on resolved expression and declaration names.
- Ownership analysis and borrow analysis depend on correct binding references.
- Diagnostics infrastructure must follow the accepted resolution diagnostic obligations in this ADR and ADR-0015.

## Dependencies

- `docs/adr/ADR-0010-type-system-shape.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
- `docs/adr/ADR-0022-declaration-syntax.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0025-module-package-visibility-model.md`
