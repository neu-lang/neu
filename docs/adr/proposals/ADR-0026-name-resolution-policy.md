# ADR-0026: Name Resolution Policy

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language semantics, not an accepted ADR, and not a valid basis for implementation.

No implementation may depend on this proposal until accepted by Chief Architect and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0016-name-resolution-policy.md`.

## Question

What bootstrap name-resolution subset, lookup order, scope boundaries, import semantics, duplicate-name behavior, unresolved-name diagnostics, and ambiguity handling should M0016 implement before type checking?

## Competing Designs

1. Top-level-only resolution within one module.
2. Module package top-level resolution with inactive import syntax.
3. Local lexical scope plus module top-level resolution.
4. Full package, import, local, member, and cross-module resolution.

## Trade-offs

Top-level-only resolution is simple and safe, but does not exercise local bindings from ADR-0024 and leaves expression name references mostly unresolved.

Module package top-level resolution fits M0014 and M0015 infrastructure, but still defers local scope and import behavior.

Local lexical scope plus module top-level resolution supports the smallest useful frontend pipeline before type checking, but requires explicit shadowing, duplicate, and diagnostic rules.

Full resolution is closer to a real language implementation, but imports, dependency lookup, overloads, members, extensions, and cross-module visibility are not yet specified enough to implement safely.

## Recommended Draft Choice

Define a small bootstrap name-resolution policy with local lexical scope plus same-module package top-level declarations, while keeping imports syntax-only and cross-module dependency lookup deferred.

The accepted version should specify only the model needed by the near-term frontend pipeline:

- approved name-reference forms for M0016
- declaration binding positions
- local lexical scope boundaries
- same-module package top-level lookup
- lookup order
- duplicate-name behavior
- unresolved-name diagnostics
- ambiguous-name diagnostics
- visibility interaction for same-module declarations
- explicit deferrals for imports, cross-module dependencies, members, overloads, extensions, and type-directed lookup

The accepted version must not rely on Kotlin, Rust, Go, existing name-table behavior, or parser behavior as implicit authority.

## Draft Bootstrap Resolution Subset

This section is a draft direction, not accepted semantics.

The draft direction is:

- M0016 resolves syntactic name references that are already present in accepted parser output.
- A name reference is a simple identifier expression or a qualified case/name syntax node if accepted by the parser for the relevant grammar position.
- Top-level declarations bind names in their declaring module and package namespace.
- Local `val` and `var` statements bind names in the containing block after the declaration statement.
- Function parameters are not resolved until parameter syntax has accepted AST representation.
- Pattern bindings are deferred unless a later accepted ADR defines binding positions and ownership behavior for patterns before M0016 acceptance.
- Declaration bodies introduce nested lexical scopes.
- Block expressions introduce lexical scopes for local statements inside the block.
- Same-block duplicate local bindings are rejected if accepted by review.
- Same-package duplicate top-level declarations are rejected for the bootstrap subset if accepted by review.
- The same textual top-level name may exist in distinct modules.
- The same textual top-level name may exist in distinct packages inside the same module.
- Imports are syntax only for the bootstrap subset and do not participate in lookup.
- Cross-module lookup is deferred until module dependency metadata is accepted.
- Member lookup, method lookup, constructor lookup, overload resolution, extension methods, and type-directed lookup are deferred.

## Draft Concrete Resolution Model

This section is a draft direction, not accepted semantics.

M0016 should resolve a deliberately small set of syntactic names from already accepted AST output. Resolution should produce stable symbol references for accepted declaration and local binding positions, or a resolution diagnostic when a referenced name cannot be resolved without guessing.

The draft model should be treated as a bootstrap model only. It must not activate imports, cross-module lookup, member lookup, overload resolution, extension method lookup, or type-directed lookup.

## Draft Resolvable AST Node Kinds

This section is a draft direction, not accepted semantics.

Included name-reference nodes for M0016 should be:

- simple identifier expression
- qualified name expression when it is syntactically a package-qualified name
- type name node in an accepted declaration, local binding, or explicit type annotation position
- package-qualified name in an accepted type or expression name-reference position

Excluded name-reference nodes for M0016 should be:

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

## Draft Declaration And Binding Positions

This section is a draft direction, not accepted semantics.

Included declaration and binding positions for M0016 should be:

- function declaration name
- type declaration name
- local `val` statement
- local `var` statement

Excluded declaration and binding positions for M0016 should be:

- function parameters are excluded until accepted parameter AST representation exists
- pattern bindings are excluded until accepted pattern binding, ownership, and scope rules exist
- import aliases are excluded because imports remain syntax-only
- member declarations are excluded because member lookup remains unsupported
- fields are excluded because field syntax remains outside the bootstrap subset

Top-level declarations should bind in the tuple `(module, package namespace, declaration name, declaration kind)`.

Local `val` and `var` statements should bind in the nearest containing lexical block.

## Draft Scope And Declaration Order

This section is a draft direction, not accepted semantics.

Declaration bodies introduce lexical scopes. Block expressions introduce lexical scopes for local statements inside the block.

Top-level declarations in the same module and package namespace should be visible throughout that module/package namespace regardless of source-file order.

local bindings are not visible before their declaration statement. A local binding should be visible after its declaration statement through the end of the containing lexical block, including nested child blocks unless shadowed by a nearer declaration.

Nested declaration bodies should not inherit local bindings from the enclosing declaration body unless a later accepted closure or capture model defines that behavior.

## Draft Shadowing And Duplicate Rules

This section is a draft direction, not accepted semantics.

An inner local declaration shadows an outer local declaration with the same name after the inner declaration statement.

A local declaration shadows a top-level declaration with the same name inside the local declaration's visible range.

Shadowing never chooses a less local declaration while a more local declaration is visible.

A same-scope duplicate local binding should be rejected with `duplicate_name`.

A same-module same-package duplicate top-level declaration with the same declaration kind should be rejected with `duplicate_name`.

If multiple candidates remain after applying scope, package, and duplicate rules, resolution should report ambiguity instead of choosing by insertion order, source-file order, or parser traversal order.

## Draft Lookup Rules

This section is a draft direction, not accepted semantics.

For an unqualified simple identifier expression, lookup order should be:

1. Innermost local lexical scope outward.
2. Current source file's package namespace in the current module.
3. No implicit import, prelude, cross-module, member, overload, extension, or type-directed lookup.

For a type name node, the same lookup order should apply unless a later accepted type namespace ADR defines separate value and type namespaces.

For a package-qualified name, lookup should use the explicitly named package namespace in the current module only.

Imports remain syntax-only and must not add lookup candidates.

cross-module lookup remains unsupported because module dependency metadata is not yet accepted.

member lookup remains unsupported.

overload resolution remains unsupported.

extension method lookup remains unsupported.

type-directed lookup remains unsupported.

## Draft Visibility Rule

This section is a draft direction, not accepted semantics.

M0016 should record the visibility metadata attached to resolved same-module top-level declarations, but visibility enforcement should be deferred until cross-module lookup and API boundary rules are accepted.

Same-module lookup should not reject declarations solely because they are `private`, `internal`, `protected`, or `public`; those categories should remain metadata during the bootstrap resolution pass.

## Draft Resolution Diagnostics

This section is a draft direction, not accepted semantics.

Diagnostic: `unresolved_name`

- Primary span: the full source span of the unresolved name-reference node.
- Recovery action: continue analysis with an unresolved-name placeholder so later diagnostics can avoid cascading resolution guesses.
- Source-of-truth citation: accepted ADR-0026 section defining M0016 lookup order.
- Safe suggestion policy: suggestions may mention names found in the same lookup tier only; suggestions must not imply import, cross-module, member, overload, extension, or type-directed lookup.

Diagnostic: `duplicate_name`

- Primary span: the later declaration or local binding that violates the duplicate rule.
- Recovery action: keep the first declaration as the canonical candidate for subsequent lookup in the same scope and mark the duplicate declaration invalid.
- Source-of-truth citation: accepted ADR-0026 section defining duplicate-name behavior.
- Safe suggestion policy: suggest renaming only; do not suggest changing visibility, imports, or module/package placement.

Diagnostic: `ambiguous_name`

- Primary span: the ambiguous name-reference node.
- Recovery action: continue analysis with an unresolved-name placeholder.
- Source-of-truth citation: accepted ADR-0026 section defining ambiguity behavior.
- Safe suggestion policy: list candidate declaration locations without selecting a winner or exposing compiler-internal table keys.

Diagnostic: `unsupported_import_resolution`

- Primary span: the import path or alias used as a lookup source.
- Recovery action: ignore the import for M0016 lookup.
- Source-of-truth citation: accepted ADR-0026 section deferring active imports.
- Safe suggestion policy: state that import resolution is not part of M0016; do not suggest equivalent active-import syntax.

Diagnostic: `unsupported_cross_module_lookup`

- Primary span: the package-qualified name or other syntax that requires another module.
- Recovery action: continue analysis with an unresolved-name placeholder.
- Source-of-truth citation: accepted ADR-0026 section deferring cross-module lookup.
- Safe suggestion policy: do not suggest adding dependencies or imports.

Diagnostic: `unsupported_member_resolution`

- Primary span: the member, method, constructor, overload, extension, or type-directed name requiring unsupported lookup.
- Recovery action: leave the referenced name unresolved for M0016.
- Source-of-truth citation: accepted ADR-0026 section deferring member and advanced lookup.
- Safe suggestion policy: state that the lookup form is outside M0016; do not infer candidate members.

## Draft Unsupported Forms

This section is a draft direction, not accepted semantics.

M0016 should reject or defer the following forms without inventing resolution behavior:

- active imports
- wildcard imports
- grouped imports
- import aliases
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

## Draft Lookup Order

This section is a draft direction, not accepted semantics.

For an unqualified identifier expression, lookup order should be:

1. Innermost local lexical scope outward.
2. Current source file's package namespace in the current module.
3. No implicit import or cross-module lookup.

For a package-qualified name in an approved M0016 position, lookup should use the current module and the explicitly named package namespace only.

If more than one declaration is found in the same lookup tier and duplicate rules did not reject it earlier, resolution reports ambiguity instead of choosing one.

## Required Accepted Content

Before implementation, the accepted source of truth must define:

- exact AST node kinds that contain resolvable name references
- exact declaration kinds that introduce symbols
- exact local binding constructs that introduce symbols
- whether pattern bindings participate in M0016
- lexical scope boundaries for declaration bodies and statement blocks
- whether declaration order affects lookup
- whether local bindings are visible before their declaration statement
- top-level declaration key: module, package namespace, name, and declaration kind
- duplicate local binding behavior
- duplicate top-level declaration behavior
- shadowing behavior between local and top-level names
- lookup order for unqualified names
- lookup rules for package-qualified names
- whether imports are syntax-only or active
- whether aliases from imports are active
- whether visibility is checked during M0016 or deferred
- unresolved-name diagnostic obligations
- duplicate-name diagnostic obligations
- ambiguous-name diagnostic obligations
- recovery behavior after resolution failure

## Required Diagnostics

The accepted version must define diagnostics for:

- `unresolved_name`
- `duplicate_name`
- `ambiguous_name`
- `inaccessible_name`, if visibility is enforced during M0016
- `unsupported_import_resolution`, if imports remain syntax-only
- `unsupported_cross_module_lookup`, if module dependencies remain deferred
- `unsupported_member_resolution`, if member lookup remains deferred

Each diagnostic must define a primary span, recovery action, source-of-truth citation, and safe suggestion policy.

## Explicit Draft Deferrals

This draft does not define:

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

- M0016 remains blocked until this proposal is reviewed, revised, and accepted into source of truth.
- M0017 type representation depends on stable resolved type-name symbols.
- M0018 type checking depends on resolved expression and declaration names.
- M0022 ownership and M0023 borrowing depend on correct binding references.
- Diagnostics infrastructure needs accepted resolution diagnostic obligations before snapshots are authoritative.

## Dependencies

- `docs/adr/ADR-0010-type-system-shape.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
- `docs/adr/ADR-0022-declaration-syntax.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0025-module-package-visibility-model.md`
